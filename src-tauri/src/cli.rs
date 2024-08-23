use crate::{check_midi_format, send_msg, Args, Port};
use magical_global as magical;
use std::fs::File;
use std::io::Read;

pub fn run(args: Args) {
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
