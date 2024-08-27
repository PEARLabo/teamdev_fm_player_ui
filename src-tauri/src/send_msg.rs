use std::io::{Read, Write};
use ymodem_send_rs::{YmodemError, YmodemSender};
type Port = Box<dyn serialport::SerialPort>;

pub fn file_size(port: &mut Port, buf: &[u8]) -> Result<(), String> {
    let f_size = buf.len().to_le_bytes();
    let bit4_header = 0x2F; //リトルエンディアンに対応させる
    let all_data: [u8; 4] = [bit4_header, f_size[0], f_size[1], f_size[2]];

    // シリアルポートにデータを書き込む
    port.write_all(&all_data)
        .map_err(|e| format!("Failed to write to serial port: {}", e))?;
    println!("File size sent!");
    Ok(())
}

pub fn file_data(port: &mut Port, data: &[u8]) {
    println!("Start Send MIDI FIle by Ymodem");
    let mut fname = "example.mid";
    let mut sender = YmodemSender::new(fname, data);
    sender.send(port).unwrap();
    println!("Maybe File sent!");
}

pub fn receive_byte(port: &mut Port) -> Result<u8, String> {
    let mut response = [0; 1];
    match port.read_exact(&mut response) {
        Ok(()) => Ok(response[0]),
        Err(e) => {
            println!("Failed to read from serial port:\n  {:?}", e);
            Err(e.to_string())
        }
    }
}

pub mod r#async {
  use kioto_serial::SerialStream;
  use tokio::io::{AsyncReadExt,AsyncWriteExt};
  use ymodem_send_rs::{YmodemError, YmodemSender};
pub async fn file_size(port: &mut SerialStream, buf: &[u8]) -> Result<(), String> {
  let f_size = buf.len().to_le_bytes();
  let bit4_header = 0x2F; //リトルエンディアンに対応させる
  let all_data: [u8; 4] = [bit4_header, f_size[0], f_size[1], f_size[2]];

  // シリアルポートにデータを書き込む
  port.write_all(&all_data).await
      .map_err(|e| format!("Failed to write to serial port: {}", e))?;
  println!("File size sent!");
  Ok(())
}

pub async fn file_data(port: &mut SerialStream, data: &[u8]) {
  println!("Start Send MIDI FIle by Ymodem");
  let fname = "example.mid";
  let sender = YmodemSender::new(fname, data);
  sender.send_async(port).await.unwrap();
  println!("Maybe File sent!");
}

pub async fn receive_byte(port: &mut SerialStream) -> Result<u8, String> {
  let mut response = [0; 1];
  match port.read_exact(&mut response).await {
      Ok(_) => Ok(response[0]),
      Err(e) => {
          println!("Failed to read from serial port:\n  {:?}", e);
          Err(e.to_string())
      }
  }
}
pub async fn send_midi_file_async(port: &mut SerialStream, buf: &[u8]) -> Result<(),String> {
  file_size(port, &buf);
  // Ymodemによるファイル転送(受信可能の場合)
  let msg_flag = receive_byte(port).await.unwrap() & 0xf;
  if msg_flag == 0xe {
      file_data(port, &buf);
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
}