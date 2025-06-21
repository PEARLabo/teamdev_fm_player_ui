use serial2_tokio::SerialPort;
//MISI形式のファイルか判定する関数
pub fn check_midi_format(contents: &[u8]) -> bool {
    contents.starts_with(b"MThd")
}
// MIDIファイルからタイトル情報を取得する
pub fn get_title(data: &[u8]) -> Option<String> {
    // Note: not implemented
    return None;
    let (hd, body) = data.split_at(14);
    // println!("{:x?}",hd);
    let (name, body) = body.split_at(4);
    let (data_len, events) = body.split_at(4);
    let mut i = 0;
    // convert to usize from big endian bytes
    let data_len = data_len
        .iter()
        .fold(0usize, |acc, &v| (acc << 8) + (v as usize));
    while i < data_len {
        // skip delta time
        while events[i] & 0x80 != 0 {
            i += 1;
        }
        i += 1;
        // parse event data and skipping
        match events[i] & 0xf0 {
            0x80 | 0x90 | 0xA0 | 0xB0 | 0xE0 => i += 3,
            0xC0 | 0xD0 => i += 2,
            0xf0 => match events[i] {
                0xf0 => {
                    while events[i] == 0xf7 {
                        i += 1
                    }
                    i += 1
                }
                0xf7 => unimplemented!(),
                0xff => {
                    // NOTE: ここを実装すれば、MIDIファイルからタイトルを取得できるはず
                    todo!()
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    None
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
