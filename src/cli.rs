use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{MoveLeft, MoveRight, MoveTo},
    event::{self, Event, EventStream, KeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute,
    terminal::{self, Clear, ClearType},
};
use futures::StreamExt;
use micromap::Set;
use serial2_tokio::SerialPort;
use std::{
    io::{Write, stdout},
    sync::atomic::AtomicU8,
};

use crate::{
    Args,
    cli::{
        dialog::{draw_suggest, file_dialog, update_file_path},
        structs::PlayingLog,
        view::Rhythm,
    },
    midi::MidiConfig,
    sequence_msg::SequenceEventFlag,
    serial_com,
    utils::{DirItem, generate_suggestion, get_title, u32_from_le},
};

mod dialog;
mod key_command;
mod keyboard;
mod structs;
mod view;

use key_command::KeyCommand;
use structs::TrackInfo;

// const MAX_CHANNEL: u8 = 6;
const MOVE_TOP: MoveTo = MoveTo(0, 0);
const MAX_CHANNELS_YM2608: u8 = 10;
const WAIT_TIME_FOR_RHYTHM: u64 = 100;

pub static MAX_CHANNEL: AtomicU8 = AtomicU8::new(6);
struct AppState {
    track_info: Vec<TrackInfo>,
    keyboard_state: Vec<Set<u8, 8>>,
    percussion_state: [u8; 6],
    title: String,
    tempo: u32,
    logs: PlayingLog,
    midi_config: MidiConfig,
    validation_conf: crate::utils::ValidationConf,
    ym2608: bool,
}
impl Default for AppState {
    fn default() -> Self {
        Self {
            track_info: vec![
                TrackInfo::default();
                MAX_CHANNEL.load(std::sync::atomic::Ordering::Relaxed) as usize
            ],
            keyboard_state: vec![Set::default(); 72],
            percussion_state: [0; 6],
            title: String::new(),
            tempo: 60,
            logs: PlayingLog::default(),
            midi_config: MidiConfig::default(),
            validation_conf: crate::utils::ValidationConf::default(),
            ym2608: false,
        }
    }
}

struct UiModel {
    state: UiState,
    dialog_msg: Option<String>,
    input_chars: String,
    fired_command: Option<KeyCommand>,
    is_error_msg_update: bool,
    cursor_pos: usize,
    before_key_command: KeyCommand,
    dir_entries: Option<Vec<DirItem>>,
}

enum EventInfo {
    None,
    Tempo(u32),
    ChInfo((u8, bool)),
    UpdateAll,
    RhythmNoteOff(u8),
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
enum UiState {
    #[default]
    Monitor,
    FileDialog,
}

struct RawModeGuard;
impl Drop for RawModeGuard {
    fn drop(&mut self) {
        if let Err(e) = terminal::disable_raw_mode() {
            eprintln!("Failed to disable raw mode: {e}");
        }
    }
}

pub async fn run(args: Args) -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    let _raw_mode_guard = RawModeGuard;
    if args.ym2608 {
        MAX_CHANNEL.store(MAX_CHANNELS_YM2608, std::sync::atomic::Ordering::Relaxed);
        dbg!(MAX_CHANNEL.load(std::sync::atomic::Ordering::Relaxed));
    }
    let mut stdout = stdout();
    execute!(
        stdout,
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
    )?;
    // SerialPort Initialization
    let port_name = match args.port_name {
        Some(name) => name,
        None => SerialPort::available_ports()
            .map_err(|e| std::io::Error::other(e.to_string()))?
            .get(args.port)
            .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Invalid port index"))?
            .to_str()
            .unwrap()
            .to_string(),
    };
    let mut port = crate::utils::open_serial_port(&port_name, args.baud_rate).unwrap();
    serial_com::clear_buffer(&mut port);
    // App Initialization
    let mut app_state = AppState {
        midi_config: MidiConfig {
            ignore_text: args.ignore_text,
            sysex_ignore: args.ignore_sysex,
        },
        validation_conf: crate::utils::ValidationConf {
            ym2608: args.ym2608,
        },
        ym2608: args.ym2608,
        ..Default::default()
    };
    let mut event_stream = EventStream::new();
    let (ui_state, maybe_title) = try_send_midi(
        &mut port,
        args.input.as_ref(),
        &app_state.midi_config,
        &app_state.validation_conf,
    )
    .await?;

    let mut ui_model = UiModel {
        state: ui_state,
        dialog_msg: maybe_title.as_ref().err().cloned(),
        input_chars: String::new(),
        fired_command: None,
        is_error_msg_update: true,
        cursor_pos: 0,
        before_key_command: KeyCommand::Other(String::new()),
        dir_entries: None,
    };
    if let Ok(t) = maybe_title {
        app_state.title = t;
    }

    update_view(
        &mut ui_model,
        &app_state,
        &EventInfo::UpdateAll,
        &mut stdout,
    )?;

    let mut msg_event = EventInfo::None;
    let (tx, mut rx) = tokio::sync::mpsc::channel::<EventInfo>(1);
    // UI Main Loop
    loop {
        tokio::select! {
            Ok(v) = serial_com::receive_byte(&mut port) => {
                msg_event = sequencer_msg_rcv(v, &mut app_state, &mut port, tx.clone()).await?;
            }
            Some(Ok(key)) = event_stream.next() => {
                if let event::Event::Key(key_event) = key
                    && handle_keyboard_event(key_event, &mut ui_model, &mut stdout)? {
                        break;
                    }
            }
            Some(e) = rx.recv() => {
                msg_event = e;
                if let EventInfo::RhythmNoteOff(r) = msg_event {
                    app_state.percussion_state[r as usize] = app_state.percussion_state[r as usize].saturating_sub(1);
                }
            }
        }

        if let EventInfo::Tempo(data) = msg_event {
            app_state.tempo = data;
        }

        if handle_command(&mut ui_model, &mut app_state, &mut port, &mut msg_event).await? {
            break;
        }

        update_view(&mut ui_model, &app_state, &msg_event, &mut stdout)?;
        msg_event = EventInfo::None;
    }

    stdout.queue(MOVE_TOP)?.execute(Clear(ClearType::All))?;
    Ok(())
}

fn handle_keyboard_event(
    key_event: event::KeyEvent,
    ui_model: &mut UiModel,
    stdout: &mut std::io::Stdout,
) -> std::io::Result<bool> {
    if key_event.kind != event::KeyEventKind::Press {
        return Ok(false);
    }

    if key_event.code == event::KeyCode::Char('c')
        && key_event.modifiers == event::KeyModifiers::CONTROL
    {
        return Ok(true);
    }

    match key_event.code {
        event::KeyCode::Enter => {
            ui_model.fired_command = Some(KeyCommand::from(ui_model.input_chars.as_str()));
            ui_model.input_chars.clear();
            ui_model.cursor_pos = 0;
        }
        event::KeyCode::Esc => ui_model.fired_command = Some(KeyCommand::GoMonitor),
        event::KeyCode::Tab => ui_model.fired_command = Some(KeyCommand::InterPolation),
        event::KeyCode::Char(c) => {
            ui_model.input_chars.insert(ui_model.cursor_pos, c);
            ui_model.cursor_pos += 1;
        }
        event::KeyCode::Backspace => {
            if ui_model.cursor_pos > 0 {
                ui_model.cursor_pos -= 1;
                ui_model.input_chars.remove(ui_model.cursor_pos);
            }
        }
        event::KeyCode::Delete => {
            if ui_model.cursor_pos < ui_model.input_chars.len() {
                ui_model.input_chars.remove(ui_model.cursor_pos);
            }
        }
        event::KeyCode::Left => {
            if ui_model.cursor_pos > 0 {
                ui_model.cursor_pos -= 1;
                execute!(stdout, MoveLeft(1))?;
            }
        }
        event::KeyCode::Right => {
            if ui_model.cursor_pos < ui_model.input_chars.len() {
                ui_model.cursor_pos += 1;
                execute!(stdout, MoveRight(1))?;
            }
        }
        _ => {}
    }
    Ok(false)
}

async fn handle_command(
    ui_model: &mut UiModel,
    app_state: &mut AppState,
    port: &mut SerialPort,
    msg_event: &mut EventInfo,
) -> std::io::Result<bool> {
    if let Some(key_event) = ui_model.fired_command.clone() {
        match key_event {
            KeyCommand::GoMonitor => {
                ui_model.state = UiState::Monitor;
                *msg_event = EventInfo::UpdateAll;
            }
            KeyCommand::InterPolation => {
                let (interpolated, ent_list) = generate_suggestion(&ui_model.input_chars);
                ui_model.input_chars = interpolated.get_content().clone();
                ui_model.cursor_pos = ui_model.input_chars.len();
                if ui_model.before_key_command == KeyCommand::InterPolation {
                    if ent_list.is_none() {
                        ui_model.fired_command = Some(KeyCommand::None);
                    }
                    ui_model.dir_entries = ent_list;
                }
            }
            KeyCommand::DialogOpen => {
                ui_model.state = UiState::FileDialog;
                ui_model.is_error_msg_update = true;
            }
            KeyCommand::Quit => return Ok(true),
            KeyCommand::Other(ref input) => {
                if ui_model.state == UiState::FileDialog {
                    let (new_ui_state, maybe_title) = try_send_midi(
                        port,
                        Some(input),
                        &app_state.midi_config,
                        &app_state.validation_conf,
                    )
                    .await?;
                    ui_model.state = new_ui_state;
                    match maybe_title {
                        Ok(t) => {
                            app_state.title = t;
                            *msg_event = EventInfo::UpdateAll;
                            ui_model.dialog_msg = None;
                            ui_model.state = UiState::Monitor;
                        }
                        Err(msg) => {
                            ui_model.dialog_msg = Some(msg);
                            ui_model.is_error_msg_update = true;
                        }
                    }
                } else {
                    ui_model.input_chars.clear();
                }
            }
            KeyCommand::None => {}
        }
    }
    ui_model.before_key_command = ui_model.fired_command.clone().unwrap_or(KeyCommand::None);
    ui_model.fired_command = None;
    Ok(false)
}

fn update_view(
    ui_model: &mut UiModel,
    app_state: &AppState,
    msg_event: &EventInfo,
    stdout: &mut std::io::Stdout,
) -> std::io::Result<()> {
    match ui_model.state {
        UiState::Monitor => {
            match msg_event {
                EventInfo::None => {}
                EventInfo::Tempo(_) => view::draw_tempo(app_state.tempo)?,
                EventInfo::ChInfo((ch, is_update_keyboard)) => {
                    view::draw_table_at(&app_state.track_info, *ch as usize, app_state.ym2608)?;
                    if *is_update_keyboard {
                        if *ch < 9 {
                            keyboard::draw_keyboard(&app_state.keyboard_state)?;
                        } else {
                            keyboard::draw_rhythm(&app_state.percussion_state)?;
                        }
                    }
                }
                EventInfo::UpdateAll => {
                    view::draw_display(app_state)?;
                }
                EventInfo::RhythmNoteOff(_) => {
                    keyboard::draw_rhythm(&app_state.percussion_state)?;
                }
            }
            view::draw_logs(&app_state.logs)?;
        }
        UiState::FileDialog => {
            if ui_model.is_error_msg_update {
                file_dialog(ui_model.dialog_msg.as_ref())?;
                ui_model.is_error_msg_update = false;
            }
            update_file_path(&mut ui_model.input_chars)?;
            if let Some(entries) = &ui_model.dir_entries {
                draw_suggest(entries)?;
            }
            stdout.queue(MoveTo(ui_model.cursor_pos as u16 + 12, 4))?;
        }
    }
    stdout.flush()
}

async fn try_send_midi(
    port: &mut SerialPort,
    path: Option<impl AsRef<str>>,
    midi_convert_config: &MidiConfig,
    validation_conf: &crate::utils::ValidationConf,
) -> std::io::Result<(UiState, Result<String, String>)> {
    let path = match path {
        Some(p) => p,
        None => return Ok((UiState::FileDialog, Err(String::new()))),
    };

    match crate::file_ctrl::file_check(Some(path.as_ref()), midi_convert_config, validation_conf) {
        Ok(Some((info, raw_data))) => {
            let res = serial_com::send_midi_file(port, &raw_data).await;
            if let Err(e) = res {
                Ok((UiState::FileDialog, Err(e)))
            } else {
                let title = get_title(&info).unwrap_or_else(|| path.as_ref().to_string());
                Ok((UiState::Monitor, Ok(title)))
            }
        }
        Ok(None) => Ok((UiState::FileDialog, Err(String::new()))),
        Err(e) => Ok((UiState::FileDialog, Err(e.to_msg()))),
    }
}

async fn sequencer_msg_rcv(
    first_byte: u8,
    app_state: &mut AppState,
    port: &mut SerialPort,
    tx_clone: tokio::sync::mpsc::Sender<EventInfo>,
) -> std::result::Result<EventInfo, std::io::Error> {
    let msg = match serial_com::receive_sequence_msg(first_byte, port).await {
        Some(serial_com::Message::Sequence(msg)) => msg,
        _ => return Ok(EventInfo::None),
        // serial_com::Message::Sequence(sequence_msg) => todo!(),
    };

    app_state.logs.add(msg.to_string());

    if let Some(ch) = msg.get_channel() {
        if ch >= MAX_CHANNEL.load(std::sync::atomic::Ordering::Relaxed) {
            return Ok(EventInfo::None);
        }
        let track = &mut app_state.track_info[ch as usize];
        let mut is_update_keyboard = false;

        match msg.event() {
            SequenceEventFlag::ProgramChange => {
                if let Some(data) = msg.get_data() {
                    let name = crate::char_code_lut::string_from_raw(data);
                    eprintln!("Ch: {ch}, inst: {name}");
                    track.set_inst(name);
                }
            }
            SequenceEventFlag::KeyEvent => {
                if let Some(data) = msg.get_data() {
                    let note = data[0] as usize;
                    track.set_key_state(if data[1] == 0 { None } else { Some(data[0]) });
                    // otherwise percussion
                    if ch < 9 {
                        if data[1] == 0 {
                            if (24..96).contains(&note) {
                                app_state.keyboard_state[note - 24].remove(&ch);
                                is_update_keyboard = true;
                            }
                        } else if (24..96).contains(&note) {
                            app_state.keyboard_state[note - 24].insert(ch);
                            is_update_keyboard = true;
                        }
                    } else {
                        // change state for PERCUSSION
                        if let Ok(r) = Rhythm::try_from(note as u8)
                            && data[1] != 0
                        {
                            app_state.percussion_state[r as usize] += 1;
                            is_update_keyboard = true;
                            // ノートオフのイベント遅延発火
                            // リズムは単発遅延発火可能にする
                            tokio::spawn(async move {
                                tokio::time::sleep(std::time::Duration::from_millis(
                                    WAIT_TIME_FOR_RHYTHM,
                                ))
                                .await;
                                // メインループにノートオフイベントを送信
                                let _ = tx_clone.send(EventInfo::RhythmNoteOff(r as u8)).await;
                            });
                        }
                    }
                }
            }
            SequenceEventFlag::Expression => {
                if let Some(data) = msg.get_data() {
                    track.set_expression(data[0]);
                }
            }
            SequenceEventFlag::PitchBend => {
                if let Some(data) = msg.get_data() {
                    track.set_pitch_bend(((data[0] as i32) | ((data[1] as i32) << 8)) - 8192);
                }
            }
            SequenceEventFlag::PanPot => {
                if let Some(data) = msg.get_data() {
                    track.set_pan_pot(data[0]);
                }
            }
            SequenceEventFlag::EventResetAllControllers => {
                app_state.track_info.iter_mut().for_each(|t| {
                    t.clear();
                });
            }
            _ => {}
        }
        Ok(EventInfo::ChInfo((ch, is_update_keyboard)))
    } else if msg.is_tempo() {
        if let Some(data) = msg.get_data() {
            Ok(EventInfo::Tempo(u32_from_le(data)))
        } else {
            Ok(EventInfo::None)
        }
    } else {
        Ok(EventInfo::None)
    }
}
