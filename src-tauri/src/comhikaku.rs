// // src/commands.rs
// use tauri::{Window, State};
// use serialport::{SerialPort, DataBits, Parity, StopBits, FlowControl};
// use std::sync::{Arc, Mutex};
// use magical_global as magical;
// use crate::AppState;

// #[tauri::command]
// pub async fn set_serial_port(window: Window, port_name: String, _state: State<'_, Arc<Mutex<AppState>>>) -> Result<(), String> {
//     let baud_rate = 115200;

//     let port_setting = serialport::new(port_name.clone(), baud_rate)
//         .data_bits(DataBits::Eight)
//         .parity(Parity::None)
//         .stop_bits(StopBits::One)
//         .flow_control(FlowControl::None)
//         .timeout(std::time::Duration::from_millis(1500))
//         .open()
//         .map_err(|e| format!("Failed to open serial port: {}", e))?;
//     // if magical::set_at(Box::new(Box::new(port_setting) as Box<dyn SerialPort>), 0).is_err() {
//     if magical::set_at(Box::new(port_setting), 0).is_err() {
//         println!("failed to set data");
//     };
//     window.emit("playback_info", &format!("Serial port opened: {}", port_name)).unwrap();

//     Ok(())
// }

// #[tauri::command]
// pub async fn disconnect_serial_port(window: Window, _state: State<'_, Arc<Mutex<AppState>>>) -> Result<(), String> {
//     // グローバルからポート設定を取得して閉じる
//     //if let Ok(mut port_setting) = magical::get_at_mut::<Box<dyn SerialPort>>(0) {
//     if let Some(mut port_setting) = magical::get_at_mut::<Box<dyn SerialPort>>(0) {
//         // ポートをクローズ
//         port_setting.as_mut().unwrap().clear(serialport::ClearBuffer::All).map_err(|e| format!("Failed to clear serial port: {}", e))?;
//         magical::set_at(Box::new(None), 0).map_err(|e| format!("Failed to clear global port setting: {:?}", e))?;
//     } else {
//         return Err("No port setting found".into());
//     }

//     window.emit("playback_info", "Serial port disconnected")
//         .map_err(|e| format!("Failed to emit playback info: {}", e))?;

//     Ok(())
// }
