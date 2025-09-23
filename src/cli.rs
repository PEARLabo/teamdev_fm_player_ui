use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::{MoveLeft, MoveRight, MoveTo},
    event::{self, EventStream, KeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute,
    style::{self, Color},
    terminal::{self, Clear, ClearType},
};
use futures::StreamExt;
use micromap::Set;
use serial2_tokio::SerialPort;
use std::io::{Write, stdout};

use crate::{
    Args,
    cli::{
        dialog::{draw_suggest, file_dialog, update_file_path},
        structs::PlayingLog,
    },
    sequence_msg::SequenceEventFlag,
    serial_com,
    utils::{DirItem, get_title, interpolation_path, u32_from_le},
};

mod dialog;
mod key_command;
mod keyboard;
mod structs;
mod view;

use key_command::KeyCommand;
use structs::TrackInfo;

const MAX_CHANNEL: u8 = 6;
const MOVETOP: MoveTo = MoveTo(0, 0);

struct AppState {
    track_info: Vec<TrackInfo>,
    keyboard_state: Vec<Set<u8, 8>>,
    title: String,
    tempo: u32,
    logs: PlayingLog,
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
            eprintln!("Failed to disable raw mode: {}", e);
        }
    }
}

pub async fn run(args: Args) -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    let _raw_mode_guard = RawModeGuard;
    let mut stdout = stdout();
    execute!(
        stdout,
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
    )?;

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
    let mut port = crate::utils::open_serial_port(&port_name).unwrap();
    serial_com::clear_buffer(&mut port);

    let mut app_state = AppState {
        track_info: vec![TrackInfo::default(); MAX_CHANNEL as usize],
        keyboard_state: vec![Set::default(); 72],
        title: String::new(),
        tempo: 60,
        logs: PlayingLog::default(),
    };
    let mut event_stream = EventStream::new();
    let (ui_state, maybe_title) = try_send_midi(&mut port, args.input.as_ref()).await?;

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

    loop {
        tokio::select! {
            Ok(v) = serial_com::receive_byte(&mut port) => {
                msg_event = sequencer_msg_rcv(v, &mut app_state, &mut port).await?;
            }
            Some(Ok(key)) = event_stream.next() => {
                if let event::Event::Key(key_event) = key {
                    if handle_keyboard_event(key_event, &mut ui_model, &mut stdout)? {
                        break;
                    }
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

    stdout.queue(MOVETOP)?.execute(Clear(ClearType::All))?;
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
                let (interpolated, ent_list) = interpolation_path(&ui_model.input_chars);
                ui_model.input_chars = interpolated;
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
                    let (new_ui_state, maybe_title) = try_send_midi(port, Some(input)).await?;
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
                    view::draw_table_at(&app_state.track_info, *ch as usize)?;
                    if *is_update_keyboard {
                        keyboard::draw_keyboard(&app_state.keyboard_state)?;
                    }
                }
                EventInfo::UpdateAll => {
                    view::draw_display(
                        &app_state.title,
                        app_state.tempo,
                        &app_state.track_info,
                        &app_state.keyboard_state,
                        &app_state.logs,
                    )?;
                }
            }
            view::draw_logs(&app_state.logs)?;
        }
        UiState::FileDialog => {
            if ui_model.is_error_msg_update {
                file_dialog(ui_model.dialog_msg.as_ref())?;
                ui_model.is_error_msg_update = false;
            }
            update_file_path(&mut ui_model.input_chars, ui_model.cursor_pos)?;
            if let Some(entries) = &ui_model.dir_entries {
                draw_suggest(entries)?;
            }
        }
    }
    stdout.flush()
}

async fn try_send_midi(
    port: &mut SerialPort,
    path: Option<impl AsRef<str>>,
) -> std::io::Result<(UiState, Result<String, String>)> {
    let path = match path {
        Some(p) => p,
        None => return Ok((UiState::FileDialog, Err(String::new()))),
    };

    match crate::file_ctrl::file_check(Some(path.as_ref())) {
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
) -> std::result::Result<EventInfo, std::io::Error> {
    let msg = match serial_com::receive_sequence_msg(first_byte, port).await {
        Some(serial_com::Message::Sequence(msg)) => msg,
        _ => return Ok(EventInfo::None),
        // serial_com::Message::Sequence(sequence_msg) => todo!(),
    };

    app_state.logs.add(msg.to_string());

    if let Some(ch) = msg.get_channel() {
        if ch >= MAX_CHANNEL {
            return Ok(EventInfo::None);
        }
        let track = &mut app_state.track_info[ch as usize];
        let mut is_update_keyboard = false;

        match msg.get_event_name() {
            SequenceEventFlag::ProgramChange => {
                if let Some(data) = msg.get_data() {
                    track.set_inst(crate::char_code_lut::string_from_raw(data));
                }
            }
            SequenceEventFlag::KeyEvent => {
                if let Some(data) = msg.get_data() {
                    let note = data[0] as usize;
                    if data[1] == 0 {
                        track.set_key_state(None);
                        if (24..96).contains(&note) {
                            app_state.keyboard_state[note - 24].remove(&ch);
                            is_update_keyboard = true;
                        }
                    } else {
                        track.set_key_state(Some(data[0]));
                        if (24..96).contains(&note) {
                            app_state.keyboard_state[note - 24].insert(ch);
                            is_update_keyboard = true;
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
