use crate::{check_midi_format, send_msg, Args, Port};
use magical_global as magical;
use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::fs::File;
use std::io::Read;

pub fn run(args: Args) {
    if if let Some(port_name) = args.port_name {
        open_serial_port(port_name)
    } else {
        if let Ok(port_info) = serialport::available_ports() {
            open_serial_port(port_info[args.port].port_name.as_str())
        } else {
            panic!("No ports");
        }
    }
    .is_err()
    {
        panic!("Could not open port");
    }

    if let Some(path) = args.input {
        let mut file = File::open(path).unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        if check_midi_format(&buf) {
            println!("Send File Size");
            send_midi_file(buf);
        } else {
            println!("Not a midi format");
        }
    } else {
        println!("No input path");
    }
}

fn send_midi_file(buf: Vec<u8>) {
    let mut port = magical::get_at_mut::<Box<dyn serialport::SerialPort>>(0).unwrap();
    send_msg::file_size(&mut port, &buf);
    // Ymodemによるファイル転送(受信可能の場合)
    let msg_flag = send_msg::receive_byte(&mut port).unwrap() & 0xf;
    if msg_flag == 0xe {
        send_msg::file_data(&mut port, &buf);
    } else {
        panic!("");
    }
    let msg_flag = send_msg::receive_byte(&mut port).unwrap() & 0xf;
    if msg_flag == 0xc {
        println!("failed to send  midi file");
    } else if msg_flag == 0xd {
        println!("success to send  midi file");
    } else {
        unreachable!();
    }
}
fn open_serial_port(port: impl AsRef<str>) -> Result<(), String> {
    let baud_rate = 115200;
    let port_setting = serialport::new(port.as_ref().to_string(), baud_rate)
        .data_bits(DataBits::Eight)
        .parity(Parity::None)
        .stop_bits(StopBits::One)
        .flow_control(FlowControl::None)
        .timeout(std::time::Duration::from_millis(1500))
        .open();
    if port_setting.is_err() {
        return Err("failed to open serial port".to_string());
    }
    // `magical::set_at` には `Box<dyn SerialPort>` を渡す
    if magical::set_at(Box::new(port_setting), 0).is_err() {
        println!("failed to set data");
    }
    Ok(())
}
