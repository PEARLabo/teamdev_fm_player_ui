// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
mod commands;
use commands::{disconnect_serial_port, set_serial_port};
use magical_global as magical;
use serialport::SerialPort;
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use tauri::{State, Window};
use ymodem_send_rs::YmodemSender;
mod cli;
mod send_msg;

#[cfg(unix)]
type Port = serialport::TTYPort;
#[cfg(windows)]
type Port = serialport::COMPort;
//use crate::AppState;
#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    disable_gui: bool,
    #[arg(short, long)]
    input: Option<String>,
}
//MISI形式のファイルか判定する関数
fn check_midi_format(contents: &[u8]) -> bool {
    contents.starts_with(b"MThd")
}

#[derive(Default)]
struct AppState {
    //port: Option<Arc<Mutex<Box<dyn serialport::SerialPort>>>>,[Check 元]
    port: Option<Arc<Mutex<dyn serialport::SerialPort>>>,
}

// エラーメッセージを格納する構造体
#[derive(serde::Serialize)]
struct ErrorMessage {
    error: String,
}

// ファイル情報を格納する構造体
#[derive(serde::Serialize)]
struct FileInfo {
    size: usize,
    is_midi: bool,
}

#[derive(Debug)]
struct U24(u32);

//24bit整数を扱うための
impl U24 {
    fn from_be_bytes(high: u8, mid: u8, low: u8) -> Self {
        Self(((high as u32) << 16) | ((mid as u32) << 8) | (low as u32))
    }

    fn value(&self) -> u32 {
        self.0
    }
}

//ファイルサイズと形式を判定するtauriコマンド
#[tauri::command]
fn read_file(
    contents: Vec<u8>,
    _state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<FileInfo, String> {
    println!("Reading file with contents of length: {}", contents.len()); // デバッグ用ログ

    let size = contents.len();
    let is_midi = check_midi_format(&contents);

    Ok(FileInfo { size, is_midi })
}

// ファイルサイズをシリアル通信で送信するTauriコマンド
#[tauri::command]
async fn send_file_size<'a>(
    window: Window,
    contents: Vec<u8>,
    _port_name: String,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<(), String> {
    //ファイルがMIDI形式かどうかを確認
    if !check_midi_format(&contents) {
        //MIDI形式でない場合returnエラー
        return Err("You choosed not MIDI file".into());
    }
    let mut port = magical::get_at_mut::<Box<dyn SerialPort>>(0).unwrap();
    send_msg::file_size(port, &contents);
    // ファイル情報を取得
    // let file_info = read_file(contents.clone(), state)?;
    // // 既にポートが設定されているか確認
    // let mut port = magical::get_at_mut::<Box<dyn SerialPort>>(0).unwrap(); // `port` をミュータブル参照として取得

    // // ファイルサイズをリトルエンディアンでバイト配列に変換
    // let size_bytes = file_info.size.to_le_bytes();
    // println!("file byte size: {:?}", size_bytes);

    // let bit4_header = 0x2F; //リトルエンディアンに対応させる
    // let all_data: [u8; 4] = [bit4_header, size_bytes[0], size_bytes[1], size_bytes[2]];

    // // シリアルポートにデータを書き込む
    // port.write_all(&all_data)
    //     .map_err(|e| format!("Failed to write to serial port: {}", e))?;
    // println!("File size sent!");

    //データ送信が始まったことを知らせるイベント
    println!("Starting send file!");
    window
        .emit("playback_info", &"Starting send file!")
        .unwrap();

    // シーケンサからの受信可能メッセージを待機
    let mut response = [0; 1];
    match port.read_exact(&mut response) {
        Ok(_) => {
            let message = format!("Received response byte: {:02x}", response[0]);
            println!("{}", message);
            window.emit("playback_info", message).unwrap();

            let high_resp = (response[0] >> 4) & 0x0F;
            let low_resp = response[0] & 0x0F;
            println!("High nibble: {:x}, Low nibble: {:x}", high_resp, low_resp);

            if high_resp == 0x0 && low_resp == 0xE {
                //YmodemSenderのインスタンスを作成
                let mut fname = "example.mid";
                let mut sender = YmodemSender::new(fname, &contents);
                sender.send(&mut port);

                // シーケンサからの受信完了メッセージを待機
                let mut ack = [0; 1];
                match port.read_exact(&mut ack) {
                    Ok(_) => {
                        println!("Received ack byte: {:02x}", ack[0]);
                        window
                            .emit("playback_info", &("Received ack byte: {:02x}", ack[0]))
                            .unwrap();
                        window
                            .emit(
                                "playback_info",
                                &("File transfer successful, checksum: {:?}", ack[0]),
                            )
                            .unwrap();

                        let ack_high_nibble = (ack[0] >> 4) & 0x0F;
                        let ack_low_nibble = ack[0] & 0x0F;
                        let _ack_ok: [u8; 1] = [0xB0];
                        let ack_err: [u8; 1] = [0xA0];

                        //受信完了メッセージのヘッダ情報かつチェックサムの内容が一致しているか
                        if ack_high_nibble == 0xD && ack_low_nibble == 0x0 {
                            // データ転送終了メッセ
                            let ack_message = format!("Received ack byte: {:02x}", ack[0]);
                            println!("{}", ack_message);
                            window
                                .emit("playback_info", ack_message)
                                .map_err(|e| format!("Failed to emit playback info: {}", e))?;
                            Ok(())
                        } else if ack_low_nibble == 0xC {
                            // 異常終了メッセ
                            port.write_all(&[0xC0])
                                .map_err(|e| format!("Failed to write to serial port: {}", e))?;
                            window
                                .emit("playback_info", "Failed to write to serial port")
                                .map_err(|e| format!("Failed to emit playback info: {}", e))?;
                            Err("Incomplete file transfer".into())
                        } else {
                            Err("ack_nibble value error".to_string())
                        }
                    }
                    Err(e) => {
                        println!("Failed to read ack from serial port: {}", e);
                        // タイムアウト後の処理として未完了メッセージを送信する
                        port.write_all(&[0xC0])
                            .map_err(|e| format!("Failed to send incomplete message: {}", e))?;
                        window
                            .emit("playback_info", &"Failed to read ack from serial port")
                            .unwrap();
                        Err("Failed to read ack from serial port".into())
                    }
                }
            } else {
                return Err("Sequencer not ready".into());
            }
        }
        Err(e) => {
            println!("Failed to read from serial port: {}", e);
            // タイムアウト後の処理として未完了メッセージを送信する
            port.write_all(&[0xC0])
                .map_err(|e| format!("Failed to send incomplete message: {}", e))?;
            window
                .emit("playback_info", "Failed to read from serial port")
                .map_err(|e| format!("Failed to emit playback info: {}", e))?;
            Err("Failed to read from serial port".into())
        }
    }
}

#[tauri::command]
async fn process_event(
    window: Window,
    _state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<(), String> {
    // 音楽再生情報を受信するためのバッファ
    let mut buffer = [0; 5]; // 最大5バイトのバッファ
                             // 既にポートが設定されているか確認
    let port = magical::get_at_mut::<Box<dyn SerialPort>>(0).unwrap(); // `port` をミュータブル参照として取得

    // フロントへのメッセージ送信デモ
    window
        .emit("playback_info", &"Starting playback info")
        .unwrap();

    loop {
        let mut msg_combined = 0u64;
        match port.read_exact(&mut buffer[0..1]) {
            Ok(_) => {
                assert!(buffer[0] & 0x0F == 0x01);
                msg_combined = buffer[0] as u64;
            }
            Err(e) => {
                println!("Failed to read from serial port: {}", e);
                // エラーメッセージを作成
                let error_message = ErrorMessage {
                    error: format!("Failed to read from serial port: {}", e),
                };

                // JSON形式でフロントエンドにメッセージ送信
                window.emit("playback_info", &error_message).unwrap();
            }
        }
        let following_size = (buffer[0] >> 4) & 0xF;
        if following_size == 0 {
            let flaga_msg = "End".to_string();
            println!("{}", flaga_msg);
            window.emit("playback_info", &flaga_msg).unwrap();
            return Ok(());
            //break;
        }
        println!("first byte: {:02X}", buffer[0]);
        println!("size: {}", following_size);
        println!("size: {}", following_size);
        assert!(following_size < 5);
        let mut tmp_buffer = vec![0; following_size as usize];
        // データを読み込む
        match port.read_exact(&mut tmp_buffer) {
            Ok(_) => {
                for i in 0..(following_size as usize) {
                    buffer[i + 1] = tmp_buffer[i];
                    msg_combined |= (buffer[i + 1] as u64) << ((i + 1) * 8);
                }

                // 受信したデータを16進数でログに表示
                println!(
                    "Received playback info (hex): {:02x?}",
                    &buffer[0..(following_size as usize + 1)]
                );

                let flag_a = u8::from_le((buffer[1]) & 0x0F);
                let chanel = u8::from_le(buffer[1] >> 4 & 0x0F);

                let tauri_msg = [
                    (msg_combined & 0xFFFFFFFF) as u32,
                    (msg_combined >> 32) as u32,
                ];

                //flag_aの判定
                match flag_a {
                    //key event
                    0 => {
                        // Little Endianであるため、bufferからkeyとvelocityを取り出す
                        let key = u8::from_le(buffer[2]);
                        let velocity = u8::from_le(buffer[3]);

                        if velocity == 0 {
                            let flaga_msg = format!(
                                "chanel: {}({:2}), key: {}({:6}), velocity: 0({:11})",
                                chanel, chanel, key, key, 0
                            );
                            println!("{}", flaga_msg);
                            window.emit("playback_info", &tauri_msg).unwrap();
                        } else if velocity != 0 {
                            let flaga_msg = format!(
                                "chanel: {}({:2}), key: {}({:6}), velocity: {}({:11})",
                                chanel, chanel, key, key, velocity, velocity
                            );
                            println!("{}", flaga_msg);
                            window.emit("playback_info", &tauri_msg).unwrap();
                        }
                    }
                    //tempo event
                    1 => {
                        let tempo = U24::from_be_bytes(buffer[2], buffer[3], buffer[4]);
                        let bpm = 1000000 / tempo.value();

                        let flaga_msg = format!(
                            "tempo: {:?}({:?})[μsec/四分音符], BPM: {}",
                            tempo.value(),
                            tempo,
                            bpm
                        );
                        println!("{}", flaga_msg);
                        window.emit("playback_info", &tauri_msg).unwrap();
                    }
                    //end event
                    2 => {
                        unreachable!();
                    }
                    //nop event
                    3 => {
                        // No operation
                    }
                    //param event
                    4 => {
                        let event = u8::from_le((buffer[2] >> 4) & 0x0F);
                        let slot = u8::from_le(buffer[2] & 0x0F);
                        let param_data = u8::from_be(buffer[3]);

                        let flaga_msg = match event {
                            0 => format!(
                                "Slot: {}({:6}), change param: {}({:11})",
                                slot, slot, param_data, param_data
                            ),
                            1 => format!(
                                "Detune/Multiple: {}({:6}), change param: {}({:11})",
                                slot, slot, param_data, param_data
                            ),
                            2 => format!(
                                "TotalLevel: {}({:6}), change param: {}({:11})",
                                slot, slot, param_data, param_data
                            ),
                            3 => format!(
                                "KeyScale/AttackRate: {}({:6}), change param: {}({:11})",
                                slot, slot, param_data, param_data
                            ),
                            4 => format!(
                                "DecayRate: {}({:6}), change param: {}({:11})",
                                slot, slot, param_data, param_data
                            ),
                            5 => format!(
                                "SustainRate: {}({:6}), change param: {}({:11})",
                                slot, slot, param_data, param_data
                            ),
                            6 => format!(
                                "SustainLevel/ReleaseRate: {}({:6}), change param: {}({:11})",
                                slot, slot, param_data, param_data
                            ),
                            7 => format!(
                                "FeedBack/Connection: {}({:6}), change param: {}({:11})",
                                slot, slot, param_data, param_data
                            ),
                            _ => format!("Invalid event: {}", event),
                        };

                        println!("{}", flaga_msg);
                        // window.emit("playback_info", &tauri_msg).unwrap();
                    }
                    5 => {
                        let flaga_msg = "FlagA is 5: Skip to next track.".to_string();
                        println!("{}", flaga_msg);
                        window.emit("playback_info", &tauri_msg).unwrap();
                    }
                    _ => {
                        let flaga_msg = format!("FlagA is invalid: {}", flag_a);
                        println!("{}", flaga_msg);
                        window.emit("playback_info", &tauri_msg).unwrap();
                    }
                }
            }
            Err(e) => {
                println!("Failed to read from serial port: {}", e);
                // エラーメッセージを作成
                let error_message = ErrorMessage {
                    error: format!("Failed to read from serial port: {}", e),
                };

                // JSON形式でフロントエンドにメッセージ送信
                window.emit("playback_info", &error_message).unwrap();
            }
        }
    }
}

// アプリケーションのエントリーポイント
fn main() {
    let app_state = Arc::new(Mutex::new(AppState { port: None }));
    let args = Args::parse();
    // ignore proxy
    let proxy_env_value = match std::env::var("http_proxy") {
        Ok(val) => {
            std::env::set_var("http_proxy", "");
            std::env::set_var("https_proxy", "");
            val
        }
        Err(_e) => String::from("proxy setting error"),
    };
    if args.disable_gui {
        cli::run(args);
    } else {
        tauri::Builder::default()
            .manage(app_state)
            .invoke_handler(tauri::generate_handler![
                read_file,
                process_event,
                send_file_size, // 本番用
                //send_file_test  // テスト用
                set_serial_port,
                disconnect_serial_port,
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }

    // let port_name = "COM3".to_string();
    // process_event(port_name);
    // reset env proxy

    if !proxy_env_value.is_empty() {
        std::env::set_var("http_proxy", proxy_env_value.as_str());
        std::env::set_var("https_proxy", proxy_env_value.as_str());
    }
}
