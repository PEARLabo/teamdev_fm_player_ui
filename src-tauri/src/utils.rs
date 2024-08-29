use serial2_tokio::SerialPort;
//MISI形式のファイルか判定する関数
pub fn check_midi_format(contents: &[u8]) -> bool {
    contents.starts_with(b"MThd")
}

pub fn get_serial_port_list() -> Option<Vec<String>> {
    if let Ok(ports_info) = SerialPort::available_ports() {
        Some(
            ports_info
                .into_iter()
                .map(|info| info.to_str().unwrap().to_string())
                .collect::<Vec<String>>(),
        )
    } else {
        None
    }
}
