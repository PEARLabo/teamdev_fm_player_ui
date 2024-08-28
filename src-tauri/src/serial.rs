use serial2_tokio::SerialPort;
use tokio::io::AsyncReadExt;
pub async fn read_one_byte(port: &mut SerialPort) -> u8 {
    let mut buf = [0; 1];
    port.read_exact(&mut buf).await.unwrap();
    buf[0]
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
