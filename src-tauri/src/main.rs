use tauri::Manager;
use std::io::{self, Write, Read};
use serialport::{SerialPortSettings, DataBits, FlowControl, Parity, StopBits};
use std::time::Duration;
use Playback_Information::playback_info::process_event; //[Check!](ライブラリのパスの設定)


// #[derive(Debug)]
// struct Bitfield {
//     value: u8,
// }


// ファイル情報を格納する構造体
#[derive(serde::Serialize)]
struct FileInfo {
    size: usize,
    is_midi: bool,
}


// //4bit単位のデータを扱うようにビットフィールドを作成
// impl Bitfield {
//     // 2つの4-bit値で新しいBitfieldを作成
//     fn new(high: u8, low: u8) -> Self {
//         assert!(high <= 0x0F && low <= 0x0F, "Values must be 4-bit integers");
//         Self {
//             value: (high << 4) | (low & 0x0F),
//         }
//     }

//     // 高位4-bit値を取得
//     fn high(&self) -> u8 {
//         (self.value >> 4) & 0x0F
//     }

//     // 低位4-bit値を取得
//     fn low(&self) -> u8 {
//         self.value & 0x0F
//     }
// }


// ファイルの内容を受け取り、情報を返すTauriコマンド
#[tauri::command]
async fn read_file(contents: Vec<u8>) -> Result<FileInfo, String> {
    println!("Reading file with contents of length: {}", contents.len());  // デバッグ用ログ

    // MIDIファイルかどうかを確認
    let is_midi = contents.len() >= 4 && &contents[..4] == b"MThd";
    println!("File size: {}, Is MIDI: {}", contents.len(), is_midi);  // デバッグ用ログ
    Ok(FileInfo {
        size: contents.len(),
        is_midi,
    })
}


// ファイルサイズをシリアル通信で送信するTauriコマンド
#[tauri::command]
async fn send_file_size(contents: Vec<u8>, port_name: String) -> Result<(), String> {
    // ファイル情報を取得
    let file_info = read_file(contents.clone()).await?;

    // シリアルポートの設定
    let settings = SerialPortSettings {
        baud_rate: 9600,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(1500),
    };

    // シリアルポートを開く
    let mut port = serialport::open_with_settings(&port_name, &settings)
        .map_err(|e| format!("Failed to open serial port: {}", e))?;

    // ファイルサイズをリトルエンディアンでバイト配列に変換
    let size_bytes = file_info.size.to_le_bytes();
    println!("file byte size: {:?}", size_bytes);

    //let bit4_header = Bitfield::new(0x0F, 0x02);
    let bit4_header = 0xF2;//リトルエンディアンに対応させる
    let all_data: [u8; 3] = [bit4_header, size_bytes[0], size_bytes[1]];

    // シリアルポートにデータを書き込む
    port.write_all(&all_data)
        .map_err(|e| format!("Failed to write to serial port: {}", e))?;

    // シーケンサからの受信可能メッセージを待機
    let mut response = [0; 1];
    match port.read_exact(&mut response) {
        Ok(_) => {
            println!("Received response byte: {:02x}", response[0]);
            
            let high_resp = (response[0] >> 4) & 0x0F;
            let low_resp = response[0] & 0x0F;
            println!("High nibble: {:x}, Low nibble: {:x}", high_resp, low_resp);

            if high_resp == 0xE && low_resp == 0x0 {
                // ファイルデータをシリアルポートに書き込む
                port.write_all(&contents)
                    .map_err(|e| format!("Failed to send file data: {}", e))?;

                // シーケンサからの受信完了メッセージを待機
                let mut ack = [0; 1];
                match port.read_exact(&mut ack) {
                    Ok(_) => {
                        println!("Received ack byte: {:02x}", ack[0]);

                        let ack_high_nibble = (ack[0] >> 4) & 0x0F;
                        let ack_low_nibble = ack[0] & 0x0F;
                        let ack_OK: [u8; 1] = [0xB0];
                        let ack_ERR: [u8; 1] = [0xA0];

                        //受信完了メッセージのヘッダ情報かつチェックサムの内容が一致しているか
                        if ack_high_nibble == 0xD && ack_low_nibble == 0x0 {
                            //データ転送終了メッセ
                            println!("File transfer successful, checksum: {:?}", ack[1]);
                            port.write_all(&ack_OK)
                                .map_err(|e| format!("Failed to write to serial port: {}", e))?;
                        } else if ack_low_nibble == 0xC {
                            //異常終了メッセ
                            port.write_all(&ack_ERR)
                                .map_err(|e| format!("Failed to write to serial port: {}", e))?;
                            return Err("Incomplete file transfer".into());
                        }
                    },
                    Err(e) => {
                        println!("Failed to read ack from serial port: {}", e);
                        // タイムアウト後の処理として未完了メッセージを送信する
                        port.write_all(&[0xC0])
                            .map_err(|e| format!("Failed to send incomplete message: {}", e))?;
                    }
                }
            } else {
                return Err("Sequencer not ready".into());
            }
        },
        Err(e) => {
            println!("Failed to read from serial port: {}", e);
            // タイムアウト後の処理として未完了メッセージを送信する
            port.write_all(&[0xC0])
                .map_err(|e| format!("Failed to send incomplete message: {}", e))?;
        }
    }

    Ok(())
}

// アプリケーションのエントリーポイント
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_file, send_file_size])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    let port_name = "COM3".to_string();
    process_event(port_name);
}