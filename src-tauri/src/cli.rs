use crate::{check_midi_format, send_msg, Args, Port};
use magical_global as magical;
use serialport::{DataBits, FlowControl, Parity, StopBits};
use std::fs::File;
use std::io::Read;

pub fn run(args: Args) {
    if let Err(e) = open_serial_port(args.port) {
        panic!("{}", e);
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
fn open_serial_port(target: usize) -> Result<(), String> {
    if let Ok(ports_info) = serialport::available_ports() {
        let baud_rate = 115200;
        let port_setting = serialport::new(ports_info[target].port_name.clone(), baud_rate)
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
    } else {
        Err("failed to get available ports".to_string())
    }
}
