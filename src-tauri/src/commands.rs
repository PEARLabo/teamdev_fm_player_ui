// src/commands.rs
use crate::{utils::check_midi_format, AppState, FileInfo};
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
// //ファイルサイズと形式を判定するtauriコマンド
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
pub async fn send_file_size(
    data: Vec<u8>,
    state: tauri::State<'_, AppState>,
) -> Result<(), String> {
    println!("call send fn");
    let mut dst = state.file_data.lock().await;
    *dst = Some(data);
    let async_proc_input_tx = state.inner.lock().await;
    async_proc_input_tx
        .send((InternalCommand::Send, String::from("")))
        .await
        .map_err(|e| e.to_string())
}
