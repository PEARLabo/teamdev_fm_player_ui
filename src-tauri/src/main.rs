// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod char_code_lut;
mod cli;
// mod commands;
mod sequence_msg;
mod serial_com;
mod utils;
use clap::Parser;

// use commands::*;

use serial2_tokio::SerialPort;
// use tauri::Manager;
use tokio::sync::{mpsc, Mutex};
#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    disable_gui: bool,
    #[arg(short, long)]
    input: Option<String>,
    #[arg(short, long)]
    list: bool,
    #[arg(short, long, default_value_t = 0)]
    port: usize,
    #[arg(long)]
    port_name: Option<String>,
}
// #[derive(Default)]
// struct AppState {
//     inner: Mutex<mpsc::Sender<(InternalCommand, String)>>,
//     srec_file: Mutex<Option<String>>,
//     file_data: Mutex<Option<Vec<u8>>>,
// }
// #[derive(serde::Serialize, Clone)]
struct ToFrontMsg {
    msg: String,
    id: Option<u16>,
}
// impl<'a> From<&'a str> for ToFrontMsg {
//     fn from(msg: &'a str) -> Self {
//         return Self {
//             msg: msg.to_string(),
//             id: None,
//         };
//     }
// }
// impl ToFrontMsg {
//     fn port_opened() -> Self {
//         return Self {
//             msg: "serial port opened".to_string(),
//             id: Some(1),
//         };
//     }
//     fn port_closed() -> Self {
//         return Self {
//             msg: "serial port closed".to_string(),
//             id: Some(2),
//         };
//     }
// }
// エラーメッセージを格納する構造体
// #[derive(serde::Serialize)]
struct ErrorMessage {
    error: String,
}

// ファイル情報を格納する構造体
// #[derive(serde::Serialize)]
struct FileInfo {
    size: usize,
    is_midi: bool,
}

// アプリケーションのエントリーポイント
fn main() {
    const BAUD_RATE: u32 = 115200;
    let args = Args::parse();
    // ignore proxy
    // let proxy_env_value = match std::env::var("http_proxy") {
    //     Ok(val) => {
    //         std::env::set_var("http_proxy", "");
    //         std::env::set_var("https_proxy", "");
    //         val
    //     }
    //     Err(_e) => String::from("proxy setting error"),
    // };
    if args.list {
        // Print the list of available ports
        if let Some(list) = utils::get_serial_port_list() {
            if list.is_empty() {
                println!("No serial port found");
            } else {
                list.iter().enumerate().for_each(|(i, port)| {
                    println!("{}: {}", i, port);
                });
            }
        } else {
            println!("No serial port found");
        }
    } else {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(cli::run(args))
    }
}

// Asyncの世界とのやり取り
// async fn async_process_model(
//     mut input_rx: mpsc::Receiver<(InternalCommand, String)>,
//     output_tx: mpsc::Sender<(InternalCommand, String)>,
// ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
//     while let Some(input) = input_rx.recv().await {
//         let output = input;
//         output_tx.send(output).await?;
//     }
//     Ok(())
// }
