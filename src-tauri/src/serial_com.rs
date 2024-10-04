use crate::sequence_msg::{SequenceEventFlag, SequenceMsg};
use serial2_tokio::SerialPort;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use ymodem_send_rs::{YmodemAsyncSend, YmodemSender};
pub enum Message {
    Sequence(SequenceMsg),
    Printf(String),
}
impl From<SequenceMsg> for Message {
    fn from(value: SequenceMsg) -> Self {
        Message::Sequence(value)
    }
}
impl From<String> for Message {
    fn from(value: String) -> Self {
        Message::Printf(value)
    }
}
pub async fn file_size(port: &mut SerialPort, buf: &[u8]) -> Result<(), String> {
    let f_size = buf.len().to_le_bytes();
    let bit4_header = 0x2F; //リトルエンディアンに対応させる
    let all_data: [u8; 4] = [bit4_header, f_size[0], f_size[1], f_size[2]];

    // シリアルポートにデータを書き込む
    port.write_all(&all_data)
        .await
        .map_err(|e| format!("Failed to write to serial port: {}", e))?;
    println!("File size sent!");
    Ok(())
}

pub async fn file_data(port: &mut SerialPort, data: &[u8]) {
    println!("Start Send MIDI FIle by Ymodem");
    let fname = "example.mid";
    let sender = YmodemSender::new(fname, data);
    sender.send(port).await.unwrap();
    println!("Maybe File sent!");
}
// Receive only one byte
pub async fn receive_byte(port: &mut SerialPort) -> Result<u8, String> {
    let mut response = [0; 1];
    match port.read_exact(&mut response).await {
        Ok(_) => Ok(response[0]),
        Err(e) => {
            println!("Failed to read from serial port:\n  {:?}", e);
            Err(e.to_string())
        }
    }
}

pub async fn send_midi_file(port: &mut SerialPort, buf: &[u8]) -> Result<(), String> {
    file_size(port, buf).await.unwrap();
    // Ymodemによるファイル転送(受信可能の場合)
    let msg_flag = receive_byte(port).await.unwrap() & 0xf;
    if msg_flag == 0xe {
        file_data(port, buf).await;
    } else {
        println!("Communication partner is not accepting.");
        return Err(String::from("Communication partner is not accepting."));
    }
    let msg_flag = receive_byte(port).await.unwrap() & 0xf;
    if msg_flag == 0xc {
        println!("failed to send  midi file");
        Err(String::from("failed to send  midi file"))
    } else if msg_flag == 0xd {
        println!("success to send  midi file");
        Ok(())
    } else {
        println!("received: {:#01X}", msg_flag);
        unreachable!();
    }
}

pub async fn receive_sequence_msg(
    first_byte: u8,
    port: &mut serial2_tokio::SerialPort,
) -> Option<Message> {
    if first_byte == 0x0 {
      println!("Loader start wait.");
      return None;
    } else if first_byte == 0xd {
      println!("load success");
      return None;
    } else if first_byte == 0xa {
      println!("load failed");
      return None;
    } else if first_byte == 0xE {
      println!("Sequencer ready");
      return None;
    }
    let msg_flag = first_byte & 0xf;
    let len = (first_byte >> 4) as usize;
    if len == 0 && msg_flag == 1 {
        // End Event(継続するデータなし)
        println!("NOP OR END");
        return Some(Message::from(SequenceMsg::new(
            0,
            SequenceEventFlag::End,
            None,
        )));
    } else if msg_flag != 0x1 && msg_flag != 0x07 {
      // 読み捨てコード
      // let mut buf = vec![0u8; len];
      // port.read_exact(&mut buf).await.unwrap();
      println!("receive: {:#02x}", msg_flag);
      return None
    }
    let len = if (msg_flag & 0xf) == 0x7 {
        // Printf protocol length
        println!("!printf protocol!");
        let low_byte = crate::serial_com::receive_byte(port).await.unwrap();
        let high_byte = crate::serial_com::receive_byte(port).await.unwrap();
        ((high_byte as usize) << 8) | (low_byte as usize)
    } else {
      // Sequence msg Protocol
        let high_byte = crate::serial_com::receive_byte(port).await.unwrap();
        len | ((high_byte as usize) << 4)
    };
    println!("len: {len}");
    let mut buf = vec![0; len];
    port.read_exact(&mut buf).await.unwrap();
    if msg_flag == 0x7 {
        // Printfのメッセージ
        let str = std::str::from_utf8(&buf).unwrap().to_string();
        return Some(Message::from(str));
    } else if msg_flag != 1 {
        println!("receive: {:#02x}", msg_flag);
        return None;
    }

    Some(Message::from(SequenceMsg::from(buf.as_slice())))
}

pub fn clear_buffer(port: &mut SerialPort) {
    port.discard_input_buffer().unwrap();
    port.discard_output_buffer().unwrap();
}
async fn send_text(port: &mut SerialPort, text: &str) {
    port.write_all(text.as_bytes()).await.unwrap()
}
pub async fn send_raw_text_file(port: &mut SerialPort, fname: impl AsRef<std::path::Path>) {
    let file = std::fs::read_to_string(fname).unwrap();
    send_text(port, &file).await;
}
