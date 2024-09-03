// src/commands.rs
use crate::{sequence_msg::SequenceMsg, serial_com, utils::check_midi_format, AppState, FileInfo};
use std::{fs::File, io::Read};
use tauri::State;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum InternalCommand {
    Open,
    Close,
    Send,
}
impl std::fmt::Display for InternalCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "Port Open"),
            Self::Close => write!(f, "Port Close"),
            Self::Send => write!(f, "Send File"),
        }
    }
}

#[tauri::command]
pub async fn open_file(path: String, state: State<'_, AppState>) -> Result<(), String> {
    let mut file = File::open(&path).map_err(|e| e.to_string())?;
    let mut buf = Vec::with_capacity(file.metadata().unwrap().len() as usize);
    let _ = file.read_to_end(&mut buf).map_err(|e| e.to_string())?;
    let is_midi = check_midi_format(&buf);
    if is_midi {
        // Set File Data
        let mut dst = state.file_data.lock().await;
        *dst = Some(buf);
        Ok(())
    } else {
      Err(format!("Invalid File Format: {path} is not Standard MIDI Format."))
    }
}
//ファイルサイズと形式を判定するtauriコマンド
// Maybe unused?
#[tauri::command]
pub fn read_file(contents: Vec<u8>, _state: State<'_, AppState>) -> Result<FileInfo, String> {
    println!("Reading file with contents of length: {}", contents.len()); // デバッグ用ログ

    let size = contents.len();
    let is_midi = check_midi_format(&contents);

    Ok(FileInfo { size, is_midi })
}

#[tauri::command]
pub async fn set_serial_port(
    port_name: String,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    let async_proc_input_tx = state.inner.lock().await;
    async_proc_input_tx
        .send((InternalCommand::Open, port_name))
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn disconnect_serial_port(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let async_proc_input_tx = state.inner.lock().await;
    async_proc_input_tx
        .send((InternalCommand::Close, String::from("")))
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn send_midi_file(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let async_proc_input_tx = state.inner.lock().await;
    async_proc_input_tx
        .send((InternalCommand::Send, String::from("")))
        .await
        .map_err(|e| e.to_string())
}

// JSの世界からのイベント分岐(trueを返すとシリアル通信を閉じる)
// TODO: フロントへのイベント発行の実装/関数名をいい感じに
pub async fn handle_internal_control<R: tauri::Runtime>(
    control: InternalCommand,
    port: &mut serial2_tokio::SerialPort,
    manager: &impl tauri::Manager<R>,
) -> bool {
    let state = manager.state::<AppState>();
    match control {
        InternalCommand::Send => {
            println!("start send file");
            match serial_com::send_midi_file(port, state.file_data.lock().await.as_ref().unwrap())
                .await
            {
                Ok(_) => {
                    println!("Success: Send File Data");
                }
                Err(msg) => {
                    println!("Error: {}", msg);
                }
            }
            false
        }
        InternalCommand::Close => true,
        _ => false,
    }
}
// TODO: フロントへの送信を実装
// シーケンサからの演奏情報受け取り時に実行する関数
pub fn handle_sequence_msg<R: tauri::Runtime>(msg: SequenceMsg, manager: &impl tauri::Manager<R>) {
    println!("{}", msg);
    println!("hogehoge");
    manager.emit_all("sequencer-msg", msg).unwrap();
}
