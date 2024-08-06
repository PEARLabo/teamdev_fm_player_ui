// // Ymodem関連の定数
// const SOH: u8 = 0x01; // Start Of Header
// const STX: u8 = 0x02; // Start Of Text
// const EOT: u8 = 0x04; // End Of Transmission
// const ACK: u8 = 0x06; // Acknowledge
// const NAK: u8 = 0x15; // Negative Acknowledge
// const C: u8 = 0x43; // 'C' for CRC mode

// /// ファイルをYmodemプロトコルで送信する関数
// ///
// /// # Arguments
// ///
// /// * `contents` - 送信するファイルのバイト列
// /// * `settings` - シリアルポートの設定
// /// * `port_name` - シリアルポートの名前
// ///
// /// # Returns
// ///
// /// `io::Result<()>` - 送信が成功した場合はOk(()), エラーが発生した場合はエラーを返す。
// fn ymodem_file_send(
//     contents: &[u8],
//     _settings: &SerialPortSettings,
//     port: &mut Box<dyn SerialPort> ,
// ) -> io::Result<()> {
//     // シリアルポートを開く
//     // let mut port = serialport::open_with_settings(port_name, settings)?;

//     // 受信側からの 'C' 信号を待つ
//     let mut response = [0; 1];
//     loop {
//         port.read_exact(&mut response)?;
//         if response[0] == C {
//             break;
//         }
//     }

//     // ファイルヘッダの送信
//     let file_header = create_file_header("example.mid", contents.len() as u64)?;
//     port.write_all(&file_header)?;

//     // ACKを待つ
//     wait_for_ack(&mut *port)?;

//     // ファイルデータの送信
//     let mut block_number = 0; // ブロック番号は0から開始
//     for chunk in contents.chunks(128) {
//         let data_block = create_data_block(chunk, block_number +1)?;
//         port.write_all(&data_block)?;

//         // ACKを待つ
//         wait_for_ack(&mut *port)?;

//         block_number += 1;
//     }

//     // EOTの送信
//     port.write_all(&[EOT])?;
//     wait_for_ack(&mut *port)?;
//     let data_block = create_data_block(&vec![0;128], 0)?;
//     port.write_all(&data_block)?;
//     // 最後のACKを待つ
//     wait_for_ack(&mut *port)?;
//     println!("YMODEM PASS!");
//     Ok(())
// }

// /// ファイルのファイルヘッダを作成する関数
// ///
// /// # Arguments
// ///
// /// * `filename` - ファイル名
// /// * `filesize` - ファイルのサイズ
// ///
// /// # Returns
// ///
// /// `io::Result<Vec<u8>>` - ファイルヘッダのバイト列を含む結果。エラーが発生した場合はエラーを返す。
// fn create_file_header(filename: &str, filesize: u64) -> io::Result<Vec<u8>> {
//     let mut header = vec![SOH, 0, 255];
//     let mut file_info = Vec::new();
//     file_info.extend_from_slice(filename.as_bytes());
//     file_info.push(0); // null terminator
//     file_info.extend_from_slice(filesize.to_string().as_bytes());
//     file_info.push(0); // null terminator

//     let mut block = vec![0u8; 128];
//     block[..file_info.len()].copy_from_slice(&file_info);
//     header.extend_from_slice(&block);
//     let crc_value = crc16_ccitt(&block);
//     header.push((crc_value >> 8) as u8);
//     header.push((crc_value & 0xFF) as u8);

//     Ok(header)
// }

// /// データブロックを作成する関数
// ///
// /// # Arguments
// ///
// /// * `chunk` - 送信するデータのバイト列
// /// * `block_number` - データブロックの番号
// ///
// /// # Returns
// ///
// /// `io::Result<Vec<u8>>` - データブロックのバイト列を含む結果。エラーが発生した場合はエラーを返す。
// fn create_data_block(chunk: &[u8], block_number: u8) -> io::Result<Vec<u8>> {
//     let mut block = vec![ SOH/*STX*/, dbg!(block_number), !block_number];
//     let mut data = vec![0u8; 128];
//     data[..chunk.len()].copy_from_slice(chunk);
//     block.extend_from_slice(&data);

//     // Convert CRC value to little-endian
//     let crc_value = crc16_ccitt(&data);
//     let crc_bytes = crc_value.to_le_bytes();

//     block.push((crc_value >> 8) as u8);
//     block.push((crc_value & 0xFF) as u8);

//     Ok(block)
// }

// /// ACKを待つ関数
// ///
// /// # Arguments
// ///
// /// * `port` - シリアルポート
// ///
// /// # Returns
// ///
// /// `io::Result<()>` - ACKを受信した場合はOk(()), エラーが発生した場合はエラーを返す。
// fn wait_for_ack(port:&mut Box<dyn SerialPort>) -> io::Result<()> {
//     let mut response = [0; 1];
//     loop {
//         port.read_exact(&mut response)?;
//         if response[0] == ACK {
//             break;
//         }
//     }
//     Ok(())
// }

// /// CRC-16-CCITTを計算する関数
// ///
// /// # Arguments
// ///
// /// * `data` - CRCを計算するデータのバイト列
// ///
// /// # Returns
// ///
// /// `u16` - 計算されたCRC値
// fn crc16_ccitt(data: &[u8]) -> u16 {
//     let mut crc = 0u16;
//     for &byte in data {
//         crc ^= (byte as u16) << 8;
//         for _ in 0..8 {
//             if (crc & 0x8000) != 0 {
//                 crc = (crc << 1) ^ 0x1021;
//             } else {
//                 crc <<= 1;
//             }
//         }
//     }
//     crc
// }