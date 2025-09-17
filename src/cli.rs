use crate::midi::MidiError;
use crate::sequence_msg::SequenceEventFlag;
use crate::utils::get_title;
use crate::{Args, serial_com, utils::check_midi_format};
use micromap::Set;
use serial2_tokio::SerialPort;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{Read, Write, stdout};
use tokio::io::AsyncBufReadExt;

type Stdin = tokio::io::Lines<tokio::io::BufReader<tokio::io::Stdin>>;

#[derive(Clone)]
enum Error {
    FileOpen(String),
    Format(String),
    FileNotExist(String),
    MidiError(MidiError),
}
impl Error {
    fn to_msg(&self) -> String {
        match self {
            Self::FileOpen(path) => format!("Failed to open file {path}."),
            Self::Format(path) => format!("File format Error: {path} is not MIDI Format 0."),
            Self::FileNotExist(path) => format!("File Not Exist: {path}"),
            Self::MidiError(e) => e.to_string(),
        }
    }
}
// dbg_mode: MIDIのI/O動作テスト
pub async fn run(args: Args) {
    let stdin = tokio::io::stdin();
    let mut input_lines = tokio::io::BufReader::new(stdin).lines();
    let mut port = if let Ok(port) = if let Some(port_name) = args.port_name {
        open_serial_port(port_name)
    } else if let Ok(port_info) = SerialPort::available_ports() {
        open_serial_port(port_info[args.port].to_str().unwrap())
    } else {
        panic!("No ports");
    } {
        port
    } else {
        panic!("Could not open port");
    };
    serial_com::clear_buffer(&mut port);
    if let Some(title) = load_midi_and_send(&mut input_lines, &mut port, args.input).await {
        init_display(title);
    } else {
        return;
    }
    let mut inst_names: [String; 6] = [
        String::from("unknown"),
        String::from("unknown"),
        String::from("unknown"),
        String::from("unknown"),
        String::from("unknown"),
        String::from("unknown"),
    ];
    let mut key_state: [Option<u8>; 6] = [None, None, None, None, None, None];
    let mut pitch_bend: [i32; 6] = [0; 6];
    let mut expression: [u8; 6] = [0; 6];
    let mut keyboad_state: Vec<Set<u8, 8>> = (0..72).map(|_| Set::default()).collect::<Vec<_>>();
    let mut is_update_keyboad = true;
    let mut log: VecDeque<String> = VecDeque::new();
    let mut num = 0;
    const FONT_COLOR: [&str; 6] = [
        "\x1b[33m", "\x1b[36m", "\x1b[32m", "\x1b[35m", "\x1b[31m", "\x1b[34m",
    ];
    loop {
        tokio::select!(
          Ok(v) = serial_com::receive_byte(&mut port) => {
            // Sequencerとの独自プロトコルの通信
            if let Some(serial_com::Message::Sequence(msg)) = serial_com::receive_sequence_msg(v, &mut port).await {
                  if let Some(ch) = msg.get_channel() {
                    // パラメータ設定
                    match  msg.get_event_name() {
                      SequenceEventFlag::ProgramChange => {
                        inst_names[ch as usize] = crate::char_code_lut::string_from_raw(msg.get_data().unwrap());
                      }
                      SequenceEventFlag::KeyEvent => {
                        let data = msg.get_data().unwrap();
                        let note = data[0] as usize;
                        if data[1] == 0 {
                          key_state[ch as usize] = None;
                          if (24..96).contains(&note) {
                            keyboad_state[note - 24].remove(&ch);
                            is_update_keyboad = true;
                          }

                        } else {
                          key_state[ch as usize] = Some(data[0]);
                          if (24..96).contains(&note) {
                            keyboad_state[note - 24].insert(ch);
                            is_update_keyboad = true;
                          }
                        }
                      }
                      SequenceEventFlag::Expression => {
                        let data = msg.get_data().unwrap();
                        expression[ch as usize] = data[0];
                      }
                      SequenceEventFlag::PitchBend => {
                        let data = msg.get_data().unwrap();
                        pitch_bend[ch as usize] =((data[0] as i32) | ((data[1] as i32)<< 8)) - 8192;
                      }
                      _=>{}
                    }
                    // 表示
                    print!("\x1b[{}B",ch+2);
                    print!("\x1b[2K");
                    println!("  {}Ch{ch} [{:<7}]\x1b[39m: {} {:<8}   {:<3}",FONT_COLOR[ch as usize],inst_names[ch as usize],
                    if let Some(n) = key_state[ch as usize].as_ref() {
                      format!("Key On  {n:<3}")
                    } else {
                      String::from("Key Off    ")
                    },
                    pitch_bend[ch as usize],
                    expression[ch as usize]
                  );
                    print!("\x1b[{}A",ch+3);
                    if is_update_keyboad {
                      print!("\x1b[9B");
                      print_keyboad(&keyboad_state);
                      print!("\x1b[11A");
                      is_update_keyboad = false;
                    }
                  } else if msg.is_tempo() {
                    let data = msg.get_data().unwrap();
                    print!("\x1b[1F");
                    println!("TEMPO: {}", unsafe {
                      *(data.as_ptr() as *const u32)
                    });
                  }
                  log.push_front(format!("{num:>5}: {msg}"));
                  num += 1;
                  if log.len() > 15 {
                    log.pop_back();
                  }
                  print!("\x1b[13B\x1b[0J");
                  log.iter().for_each(|s| println!("{s}"));
                  print!("\x1b[{}A",13+ log.len());
            }
          }
          Ok(line) = input_lines.next_line() => {
            let line = line.unwrap();
            if line == "q" {
              serial_com::clear_buffer(&mut port);
              break;
            } else if line == "o" {
              if let Some(title) = load_midi_and_send(&mut input_lines, &mut port, None::<String>).await {
                  init_display(title)
              } else {
                serial_com::clear_buffer(&mut port);
                return;
              }
            }
          }
        )
    }
}
fn init_display(title: impl AsRef<str>) {
    print!("\x1b[2J");
    print!("\x1b[1;1H");
    println!("Title: {}", title.as_ref());
    print!("\x1b[1B");
    println!("  Ch  [Inst   ]  Key State   PitchBend  Expression");
    println!("  ------------------------------------------------");
    print!("\x1b[2A");
    print!("\x1b[12B");
    println!("Received Messages:");
    print!("\x1b[13A");
    stdout().flush().unwrap();
}
async fn send_file(
    path: impl AsRef<std::path::Path>,
    port: &mut SerialPort,
) -> Result<String, Error> {
    let path = path.as_ref();
    let mut file = if let Ok(f) = File::open(path) {
        f
    } else {
        return Err(Error::FileOpen(path.to_str().unwrap().to_string()));
    };
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    let midi_info = crate::utils::validation_midi_file(&buf);
    if let Ok(midi_info) = midi_info {
        // TODO: midi_infoからタイトル情報を取得可能に (midi_infoは解析情報)
        let title = if let Some(t) = get_title(&buf) {
            t
        } else {
            path.file_name().unwrap().to_str().unwrap().to_string()
        };
        println!("Send File Size");
        serial_com::send_midi_file(port, &buf).await.unwrap();
        Ok(title)
    } else {
        // Err(Error::Format(path.to_str().unwrap().to_string()))
        Err(Error::MidiError(midi_info.unwrap_err()))
    }
}
// MIDIファイルを読みデータを送る (有効なファイルが入力されるまで聞く)
async fn load_midi_and_send(
    stdin: &mut Stdin,
    port: &mut SerialPort,
    path: Option<impl AsRef<str>>,
) -> Option<String> {
    let mut p = if let Some(p) = path {
        p.as_ref().to_string()
    } else {
        file_dialog(stdin, None).await.unwrap_or(String::from(""))
    };
    let mut path = std::path::PathBuf::from(&p);
    let mut e;
    loop {
        // control by input
        if p.is_empty() {
            continue;
        } else {
            if path.exists() {
                let res = send_file(path, port).await;
                if let Ok(title) = res {
                    return Some(title);
                } else {
                    e = res.err().unwrap();
                }
            } else {
                e = Error::FileNotExist(p.clone())
            }
            p = file_dialog(stdin, Some(e.to_msg()))
                .await
                .unwrap_or(String::from(""));
            path = std::path::PathBuf::from(&p);
        }
    }
}
async fn file_dialog(input: &mut Stdin, msg: Option<String>) -> Option<String> {
    println!("\x1b[2J");
    print!("\x1b[5;20H============= Send File ============");
    print!("\x1b[6;20H {}", msg.unwrap_or_default());
    print!("\x1b[8;20H");
    print!("======== PRESS ENTER TO SEND =======");
    print!("\x1b[7;20H file name > ");
    stdout().flush().unwrap();
    (input.next_line().await).unwrap_or_default()
}
fn open_serial_port(port: impl AsRef<str>) -> Result<SerialPort, String> {
    let baud_rate = 115200;
    let port_setting = SerialPort::open(port.as_ref(), baud_rate);
    if port_setting.is_err() {
        return Err("failed to open serial port".to_string());
    }

    Ok(port_setting.unwrap())
}

fn print_keyboad(state: &[Set<u8, 8>]) {
    let mut prev_state = "\x1b[47m";
    let mut upper = String::new();
    let mut lower = String::new();
    const COLOR_LUT: [&str; 6] = [
        "\x1b[43m", "\x1b[46m", "\x1b[42m", "\x1b[45m", "\x1b[41m", "\x1b[44m",
    ];
    state.iter().enumerate().for_each(|(n, s)| {
        let is_natural_tone = is_natural_note(n as u8);
        let color = if let Some(&ch) = s.iter().next() {
            COLOR_LUT[ch as usize]
        } else if is_natural_tone {
            "\x1b[47m"
        } else {
            "\x1b[40m"
        };
        upper += &format!("{color} ");
        if is_natural_tone {
            // 白鍵
            prev_state = color;
            lower += &format!("{color} ");
        } else {
            // 黒鍵
            lower += &format!("{prev_state} ");
        }
    });
    upper += "\x1b[49m";
    lower += "\x1b[49m";
    println!("{upper}");
    println!("{lower}");
}
fn is_natural_note(n: u8) -> bool {
    let n = n % 12;
    n == 0 || n == 2 || n == 4 || n == 5 || n == 7 || n == 9 || n == 11
}
