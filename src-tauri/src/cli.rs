use crate::{serial_com, utils::check_midi_format, Args};
// use serial2::SerialPort;
use serial2_tokio::SerialPort;
use std::fs::File;
use std::io::Read;
pub async fn run(args: Args) {
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
    if let Some(path) = args.input {
        let mut file = File::open(path).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        if check_midi_format(&buf) {
            println!("Send File Size");
            serial_com::send_midi_file(&mut port, &buf).await.unwrap();
        } else {
            println!("Not a midi format");
        }
    } else {
        println!("No input path");
    }
}
fn open_serial_port(port: impl AsRef<str>) -> Result<SerialPort, String> {
    let baud_rate = 115200;
    let port_setting = SerialPort::open(port.as_ref(), baud_rate);
    if port_setting.is_err() {
        return Err("failed to open serial port".to_string());
    }

    Ok(port_setting.unwrap())
}
