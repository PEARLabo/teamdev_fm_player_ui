use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::*,
    event::{self, EventStream, KeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute,
    style::{self, Color},
    terminal::{self, Clear, ClearType},
};
use futures::StreamExt;
//
use crate::utils::u32_from_le;
use crate::{Args, serial_com};
use crate::{cli::dialog::interpolation_path, sequence_msg::SequenceEventFlag};
use crate::{
    cli::{dialog::update_file_path, structs::PlayingLog},
    utils::get_title,
};
use micromap::Set;
use serial2_tokio::SerialPort;
use std::collections::VecDeque;
use std::io::{Write, stdout};
mod key_command;
use key_command::KeyCommand;
mod structs;
use structs::{LogItem, TrackInfo};
mod dialog;
use dialog::file_dialog;
mod keyboard;
use keyboard::draw_keyboard;
// type Stdin = tokio::io::Lines<tokio::io::BufReader<tokio::io::Stdin>>;
const TABLE_TOP: u16 = 4;
// YM2203の場合
const MAX_CHANNEL: u8 = 6;
const MoveTop: MoveTo = MoveTo(0, 0);
static CH_COLOR: &[style::Color; MAX_CHANNEL as usize] = &[
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Cyan,
];
enum EventInfo {
    None,
    // Updating tempo
    Tempo(u32),
    // Updating ChInfo / Is a keyboard update required?
    ChInfo((u8, bool)),
    UpdateAll,
    Title,
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
        terminal::disable_raw_mode().unwrap();
    }
}

pub async fn run(args: Args) -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    let _raw_mode_guard = RawModeGuard;
    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
    )?;
    // Init SerialPort
    let mut port = if let Ok(port) = if let Some(port_name) = args.port_name {
        crate::utils::open_serial_port(port_name)
    } else if let Ok(port_info) = SerialPort::available_ports() {
        crate::utils::open_serial_port(port_info[args.port].to_str().unwrap())
    } else {
        panic!("No ports");
    } {
        port
    } else {
        panic!("Could not open port");
    };
    serial_com::clear_buffer(&mut port);
    // play information
    let mut track_info = vec![TrackInfo::default(); MAX_CHANNEL as usize];
    let mut state = vec![Set::default(); 72];
    let mut title = String::new();
    let mut tempo = 60;
    let mut logs: PlayingLog = PlayingLog::default();
    // UI Data
    let mut event_stream = EventStream::new();
    let mut dialog_msg = None;
    // Validation and send MIDI File
    // ファイルダイアログの機能を分離する
    let (mut ui_state, mut maybe_title) = try_send_midi(&mut port, args.input).await;

    if let Ok(t) = maybe_title {
        title = t;
    } else {
        let _ = dialog_msg.insert(maybe_title.unwrap_err());
    }
    // Display Initialization
    match ui_state {
        UiState::Monitor => {
            draw_display(&title, tempo, &track_info, &state, &logs)?;
        }
        UiState::FileDialog => {
            file_dialog(None)?;
        }
    }
    stdout.flush()?;
    // Data
    let mut input_chars = String::new();
    let mut msg_event = EventInfo::None;
    let mut is_fired: Option<KeyCommand> = None;
    let mut is_error_msg_update = true;
    let mut cursor_pos = 0;
    // Main Loop
    loop {
        // let mut key_event_buffer: VecDeque<char> = VecDeque::new();
        tokio::select! {
            Ok(v) = serial_com::receive_byte(&mut port) => {
                msg_event = sequencer_msg_rcv(
                    v,
                    &mut track_info,
                    &mut logs,
                    &mut state,
                    &mut port,
                ).await;
            }
            Some(Ok(key))  = event_stream.next() => {
                if let event::Event::Key(key_event) = key {
                    if dbg!(key_event).kind == event::KeyEventKind::Press {
                        // exit command
                        if key_event.code == event::KeyCode::Char('c') && key_event.modifiers == event::KeyModifiers::CONTROL {
                            break;
                        }
                        // normal key events
                        match key_event.code {
                            event::KeyCode::Enter => {
                                let  _ = dbg!(is_fired.insert(KeyCommand::from(dbg!(input_chars.as_str()))));
                                input_chars.clear();
                                cursor_pos = 0;
                            }
                            event::KeyCode::Esc => {
                                is_fired = Some(KeyCommand::GoMonitor);
                            }
                            event::KeyCode::Tab => {
                                is_fired = Some(KeyCommand::InterPolation);
                            }
                            event::KeyCode::Char(c) => {
                                // key_event_buffer.push_back(c);
                                if cursor_pos == input_chars.len() {
                                    input_chars.push(c);
                                } else {
                                    input_chars.insert(cursor_pos, c);
                                }
                                cursor_pos += 1;
                                // input_chars.push(c);
                                dbg!(&input_chars);
                            }
                            event::KeyCode::Backspace => {
                                if cursor_pos > 0 {
                                    cursor_pos -= 1;
                                    input_chars.remove(cursor_pos);
                                }
                            }
                            event::KeyCode::Delete => {
                                if cursor_pos < input_chars.len() {
                                    input_chars.remove(cursor_pos);
                                }
                            }
                            event::KeyCode::Left => {
                                if cursor_pos > 0 {
                                    cursor_pos -= 1;
                                    execute!(stdout, MoveLeft(1)).unwrap();
                                }
                            }
                            event::KeyCode::Right => {
                                if cursor_pos < input_chars.len() {
                                    cursor_pos += 1;
                                    execute!(stdout, MoveRight(1)).unwrap();
                                }
                            }

                            _ => {}
                        }
                    }
                }
            }
        }
        // 状態遷移 / データ操作
        if let Some(key_event) = &is_fired {
            match key_event {
                KeyCommand::GoMonitor => {
                    ui_state = UiState::Monitor;
                    msg_event = EventInfo::UpdateAll;
                    eprintln!("change to monitor");
                }
                // 入力に対する補間処理
                KeyCommand::InterPolation => {
                    input_chars = interpolation_path(input_chars.as_str());
                }
                KeyCommand::DialogOpen => {
                    ui_state = UiState::FileDialog;
                    is_error_msg_update = true;
                }
                KeyCommand::Quit => {
                    break;
                }
                KeyCommand::Other(input) => {
                    if ui_state == UiState::FileDialog {
                        (ui_state, maybe_title) = try_send_midi(&mut port, Some(input)).await;
                        if let Ok(t) = maybe_title {
                            title = t;
                            msg_event = EventInfo::UpdateAll;
                            dialog_msg = None;
                            ui_state = UiState::Monitor;
                        } else {
                            let _ = dialog_msg.insert(maybe_title.unwrap_err());
                            is_error_msg_update = true;
                        }
                    } else {
                        eprintln!("fired other");
                        input_chars.clear();
                    }
                }
            }
        }
        is_fired = None;

        // 表示更新
        match ui_state {
            UiState::Monitor => {
                match msg_event {
                    EventInfo::None => {}
                    EventInfo::Tempo(data) => {
                        tempo = data;
                        draw_tempo(tempo)?;
                    }
                    EventInfo::ChInfo((ch, is_update_keyboard)) => {
                        // let track = track_info.get_mut(ch as usize).unwrap();
                        draw_table_at(track_info.as_slice(), ch as usize)?;
                        if is_update_keyboard {
                            draw_keyboard(&state)?;
                        }
                    }
                    EventInfo::Title | EventInfo::UpdateAll => {
                        draw_display(title.as_str(), tempo, &track_info, &state, &logs)?;
                    }
                }
                draw_logs(&logs)?;
                stdout.flush()?;
            }
            UiState::FileDialog => {
                if is_error_msg_update {
                    file_dialog(dialog_msg.clone())?;
                    is_error_msg_update = false;
                }
                update_file_path(&mut input_chars, cursor_pos)?;
                stdout.flush()?;
            }
        }
    }
    // post processing
    stdout.queue(MoveTop)?.execute(Clear(ClearType::All))?;
    Ok(())
}

async fn try_send_midi(
    port: &mut SerialPort,
    path: Option<impl AsRef<str>>,
) -> (UiState, Result<String, String>) {
    let maybe_info = crate::file_ctrl::file_check(path.as_ref());
    if let Ok(maybe_info) = maybe_info {
        if let Some((info, raw_data)) = maybe_info {
            serial_com::send_midi_file(port, &raw_data).await.unwrap();
            (
                UiState::Monitor,
                Ok(if let Some(t) = get_title(&info) {
                    t
                } else {
                    path.unwrap().as_ref().to_string()
                }),
            )
        } else {
            (UiState::FileDialog, Err(String::new()))
        }
    } else {
        (UiState::FileDialog, Err(maybe_info.err().unwrap().to_msg()))
    }
}

async fn sequencer_msg_rcv(
    first_byte: u8,
    track_info: &mut [TrackInfo],
    log: &mut PlayingLog,
    keyboard_state: &mut [Set<u8, 8>],
    port: &mut SerialPort,
) -> EventInfo {
    let mut is_update_keyboard = false;
    // let mut ch_update: Option<u8> = None;
    if let Some(serial_com::Message::Sequence(msg)) =
        serial_com::receive_sequence_msg(first_byte, port).await
    {
        log.add(msg.to_string());
        if let Some(ch) = msg.get_channel() {
            // パラメータ設定
            if ch > MAX_CHANNEL {
                return EventInfo::None;
            }
            let track = track_info.get_mut(ch as usize).unwrap();
            // ch_update = Some(ch);
            match msg.get_event_name() {
                SequenceEventFlag::ProgramChange => {
                    track.set_inst(crate::char_code_lut::string_from_raw(
                        msg.get_data().unwrap(),
                    ));
                }
                SequenceEventFlag::KeyEvent => {
                    let data = msg.get_data().unwrap();
                    let note = data[0] as usize;
                    if data[1] == 0 {
                        track.set_key_state(None);
                        if (24..96).contains(&note) {
                            keyboard_state[note - 24].remove(&ch);
                            is_update_keyboard = true;
                        }
                    } else {
                        track.set_key_state(Some(data[0]));
                        if (24..96).contains(&note) {
                            keyboard_state[note - 24].insert(ch);
                            is_update_keyboard = true;
                        }
                    }
                }
                SequenceEventFlag::Expression => {
                    let data = msg.get_data().unwrap();
                    track.set_expression(data[0]);
                }
                SequenceEventFlag::PitchBend => {
                    let data = msg.get_data().unwrap();
                    track.set_pitch_bend(((data[0] as i32) | ((data[1] as i32) << 8)) - 8192);
                }
                _ => {}
            }
            EventInfo::ChInfo((ch, is_update_keyboard))
        } else if msg.is_tempo() {
            EventInfo::Tempo(u32_from_le(msg.get_data().unwrap()))
        } else {
            EventInfo::None
        }
    } else {
        // TODO: Error実装を追加
        EventInfo::None
    }
}

fn draw_display(
    title: impl AsRef<str>,
    tempo: u32,
    track_info: &[TrackInfo],
    state: &[Set<u8, 8>],
    logs: &PlayingLog,
) -> std::io::Result<()> {
    let mut stdout = stdout();
    stdout
        .queue(Clear(ClearType::All))?
        .queue(MoveTo(0, 0))?
        .queue(style::Print(format!("Title: {}", title.as_ref())))?
        .queue(MoveTo(0, 1))?
        .queue(style::Print(format!("TEMPO: {}", tempo)))?
        .queue(MoveTo(0, 2))?
        .queue(style::Print(
            "  Ch  [Inst   ]  Key State   PitchBend  Expression",
        ))?
        .queue(MoveTo(0, 3))?
        .queue(style::Print(
            "  ------------------------------------------------",
        ))?
        .queue(MoveTo(0, MAX_CHANNEL as u16 + TABLE_TOP + 3))?
        .queue(style::Print("Received Messages:"))?;
    for ch in 0..track_info.len() {
        draw_table_at(track_info, ch)?;
    }
    draw_logs(logs)?;
    draw_keyboard(state)?;
    Ok(())
}

fn draw_tempo(tempo: u32) -> std::io::Result<()> {
    let mut stdout = stdout();
    stdout
        .queue(MoveTo(0, 1))?
        .queue(Clear(ClearType::CurrentLine))?
        .queue(style::Print(format!("TEMPO: {}", tempo)))?;
    Ok(())
}

fn draw_table_at(track_info: &[TrackInfo], ch: usize) -> std::io::Result<()> {
    let info = &track_info[ch];
    let mut stdout = stdout();
    stdout
        .queue(MoveTo(0, TABLE_TOP + ch as u16))?
        .queue(Clear(ClearType::CurrentLine))?
        .queue(style::SetForegroundColor(CH_COLOR[ch]))?
        .queue(style::Print(format!("  Ch{ch} [{}]", info.get_inst())))?
        .queue(style::ResetColor)?
        // // .queue(MoveToColumn(4))?
        // .queue(style::Print(info.get_inst()))?
        .queue(MoveToColumn(17))?
        .queue(style::Print(info.get_key_state()))?
        .queue(MoveToColumn(29))?
        .queue(style::Print(info.get_pitch_bend()))?
        .queue(MoveToColumn(40))?
        .queue(style::Print(info.get_expression()))?;
    // .flush()
    Ok(())
}

fn draw_logs(logs: &PlayingLog) -> std::io::Result<()> {
    let mut stdout = stdout();
    let s = logs.get_logs().iter().fold(String::new(), |s, log| {
        s + format!("{:8} - {}\n", log.get_id(), log.get_msg()).as_str()
    });
    stdout
        .queue(MoveTo(0, MAX_CHANNEL as u16 + TABLE_TOP + 4))?
        .queue(Clear(ClearType::FromCursorDown))?
        .queue(style::Print(s))?;
    Ok(())
}
