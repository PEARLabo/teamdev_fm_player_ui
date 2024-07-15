use tauri::Manager;
use std::io::{self, Write, Read};
use serialport::{SerialPortSettings, DataBits, FlowControl, Parity, StopBits};
use std::time::Duration;
//イベント表示をライブラリとして使用できるようにする場合
//use Playback_Information::playback_info::process_event; //[Check!](ライブラリのパスの設定)


// ファイル情報を格納する構造体
#[derive(serde::Serialize)]
struct FileInfo {
    size: usize,
    is_midi: bool,
}

#[derive(Debug)]
struct U24(u32);

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

//24bit整数を扱うための
impl U24 {
    fn from_be_bytes(high: u8, mid: u8, low: u8) -> Self {
        Self(((high as u32) << 16) | ((mid as u32) << 8) | (low as u32))
    }

    fn value(&self) -> u32 {
        self.0
    }
}

// ファイルサイズをシリアル通信で送信するTauriコマンド
#[tauri::command]
async fn send_file_size(contents: Vec<u8>, port_name: String) -> Result<(), String> {
    // ファイル情報を取得
    let file_info = read_file(contents.clone()).await?;

    // シリアルポートの設定
    let settings = SerialPortSettings {
        baud_rate: 115200,
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

// //イベント情報をシリアル通信でやり取りするコマンド
#[tauri::command]
async fn process_event(port_name: String) -> Result<(), String> {
    // シリアルポートの設定
    let settings = SerialPortSettings {
        baud_rate: 115200,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: Duration::from_millis(1500),
    };

    // シリアルポートを開く
    let mut port = serialport::open_with_settings(&port_name, &settings)
        .map_err(|e| format!("Failed to open serial port: {}", e))?;
    
    // 音楽再生情報を受信するためのバッファ
    let mut buffer = [0; 5]; // 最大5バイトのバッファ
    
    /*
    フラッシュ表示用の機能[flash]
     */
    // // keyとvelocity初期表示を設定
    // let mut stdout = stdout();
    // stdout.execute(terminal::Clear(terminal::ClearType::All)).unwrap();
    // stdout.execute(cursor::MoveTo(0, 0)).unwrap();
    // println!("Tempo: ");
    // stdout.execute(cursor::MoveTo(0, 1)).unwrap();
    // println!("Chanel: ");
    // stdout.execute(cursor::MoveTo(0, 2)).unwrap();
    // println!("Key: ");
    // stdout.execute(cursor::MoveTo(0, 3)).unwrap();
    // println!("Velocity: ");
    // stdout.flush().unwrap();
    
    loop {
        // データを読み込む
        match port.read_exact(&mut buffer) {
            Ok(_) => {
                // 受信したデータを16進数でログに表示
                println!("Received playback info (hex): {:02x?}", buffer);
    
                //let data_width = u8::from_le(buffer[0] & 0x0F);
                let flag_a = u8::from_le((buffer[1] >> 4) & 0x0F);
                let chanel = u8::from_le(buffer[1] & 0x0F);
                let event_data = buffer;

                //flag_aの判定
                match flag_a {
                    //key event
                    0 => {
                        // Little Endianであるため、bufferからkeyとvelocityを取り出す
                        let key = u8::from_le(buffer[3]);
                        let velocity = u8::from_le(buffer[4]);

                        if velocity == 0{
                            //[flash]
                            // // カーソルを移動して値を上書き
                            // stdout.execute(cursor::MoveTo(8, 1)).unwrap(); // "Key: "の後に移動
                            // print!("{:2}, chanel");
                            // stdout.execute(cursor::MoveTo(5, 2)).unwrap(); // "Key: "の後に移動
                            // print!("{:6}", key); // 5桁の幅を確保して上書き
                            // stdout.execute(cursor::MoveTo(10, 3)).unwrap(); // "Velocity:"の後に移動
                            // print!("Noteoff"); // 10桁の幅を確保して上書き
                            // // バッファをフラッシュして表示を更新
                            // stdout.flush().unwrap();
                            println!("chanel: {}({:2}), key: {}({:6}), velocity: noteoff",
                                chanel, chanel, key, key);
                        }else if velocity != 0{
                            //[flash]
                            // // カーソルを移動して値を上書き
                            // stdout.execute(cursor::MoveTo(8, 1)).unwrap(); // "Key: "の後に移動
                            // print!("{:2}", chanel);
                            // stdout.execute(cursor::MoveTo(5, 2)).unwrap(); // "Key: "の後に移動
                            // print!("{:6}", key); // 5桁の幅を確保して上書き
                            // stdout.execute(cursor::MoveTo(10, 3)).unwrap(); // "Velocity:"の後に移動
                            // print!("{:11}", velocity); // 3桁の幅を確保して上書き
                            // // バッファをフラッシュして表示を更新
                            // stdout.flush().unwrap();
                            println!("chanel: {}({:2}), key: {}({:6}), velocity: {}({:11})",
                                chanel, chanel, key, key, velocity, velocity);
                        }
                    },
                    //tempo event
                    1 => {
                        let tempo = U24::from_be_bytes(buffer[2], buffer[3], buffer[4]);
                        let bpm = 1000000 / tempo.value();

                        //tempo情報を表示
                        // stdout.execute(cursor::MoveTo(7, 0)).unwrap(); // "Tempo: "の後に移動
                        // print!("{:?}", tempo);
                        // stdout.flush().unwrap();
                        println!("tempo: {:?}({:?})[μsec/四分音符], BPM: {}", tempo.value(), tempo, bpm);
                    },
                    //end event
                    2 => {
                        // 既存のchanel, key, velocity情報をクリア
                        // stdout.execute(cursor::MoveTo(7, 0)).unwrap(); // "Tempo: "の後に移動
                        // print!("   ");
                        // stdout.execute(cursor::MoveTo(8, 1)).unwrap(); // "Chanel: "の後に移動
                        // print!("   ");
                        // stdout.execute(cursor::MoveTo(5, 2)).unwrap(); // "Key: "の後に移動
                        // print!("   ");
                        // stdout.execute(cursor::MoveTo(10, 3)).unwrap(); // "Velocity:"の後に移動
                        // print!("   ");
                        println!("End");
                    },
                    //nop event
                    3 => {
                        // No operation
                    },
                    //param event
                    4 => {
                        let event = u8::from_le((buffer[2] >> 4) & 0x0F);
                        let slot = u8::from_le(buffer[2] & 0x0F);
                        let param_data = u8::from_be(buffer[3]);

                        match event {
                            0 => println!("Slot: {}({:6}), change param: {}({:11})", slot, slot, param_data, param_data),
                            1 => println!("Detune/Multiple: {}({:6}), change param: {}({:11})", slot, slot, param_data, param_data),
                            2 => println!("TotalLevel: {}({:6}), change param: {}({:11})", slot, slot, param_data, param_data),
                            3 => println!("KeyScale/AttackRate: {}({:6}), change param: {}({:11})", slot, slot, param_data, param_data),
                            4 => println!("DecayRate: {}({:6}), change param: {}({:11})", slot, slot, param_data, param_data),
                            5 => println!("SustainRate: {}({:6}), change param: {}({:11})", slot, slot, param_data, param_data),
                            6 => println!("SustainLevel/ReleaseRate: {}({:6}), change param: {}({:11})", slot, slot, param_data, param_data),
                            7 => println!("FeedBack/Connection: {}({:6}), change param: {}({:11})", slot, slot, param_data, param_data),
                            _ => println!("Invalid event: {}", event),
                        }
                    },
                    5 => println!("FlagA is 5: Skip to next track."),
                    _ => println!("FlagA is invalid: {}", flag_a),
                }
            },
            Err(e) => {
                println!("Failed to read from serial port: {}", e);
            }
        }
    }
}


// アプリケーションのエントリーポイント
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_file, send_file_size, process_event])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    // let port_name = "COM3".to_string();
    // process_event(port_name);
}