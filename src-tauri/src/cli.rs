use crate::{send_msg, utils::check_midi_format, Args};
use serial2::SerialPort;
use std::fs::File;
use std::io::Read;

pub fn run(args: Args) {
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

    if let Some(path) = args.input {
        let mut file = File::open(path).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        if check_midi_format(&buf) {
            println!("Send File Size");
            send_midi_file(&mut port, buf);
        } else {
            println!("Not a midi format");
        }
    } else {
        println!("No input path");
    }
}

fn send_midi_file(port: &mut SerialPort, buf: Vec<u8>) {
    send_msg::file_size(port, &buf).unwrap();
    // Ymodemによるファイル転送(受信可能の場合)
    let msg_flag = send_msg::receive_byte(port).unwrap() & 0xf;
    if msg_flag == 0xe {
        send_msg::file_data(port, &buf);
    } else {
        panic!("Communication partner is not accepting.");
    }
    let msg_flag = send_msg::receive_byte(port).unwrap() & 0xf;
    if msg_flag == 0xc {
        println!("failed to send  midi file");
    } else if msg_flag == 0xd {
        println!("success to send  midi file");
    } else {
        println!("received: {:#01X}", msg_flag);
        unreachable!();
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
