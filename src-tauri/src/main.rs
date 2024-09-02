// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cli;
mod commands;
mod sequence_msg;
mod serial_com;
mod utils;
use clap::Parser;

use commands::*;

use serial2_tokio::SerialPort;
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
struct AppState {
    inner: Mutex<mpsc::Sender<(InternalCommand, String)>>,
    file_data: Mutex<Option<Vec<u8>>>,
}

// エラーメッセージを格納する構造体
#[derive(serde::Serialize)]
struct ErrorMessage {
    error: String,
}

// ファイル情報を格納する構造体
#[derive(serde::Serialize)]
struct FileInfo {
    size: usize,
    is_midi: bool,
}

// アプリケーションのエントリーポイント
fn main() {
    const BAUD_RATE: u32 = 115200;
    let args = Args::parse();
    // ignore proxy
    let proxy_env_value = match std::env::var("http_proxy") {
        Ok(val) => {
            std::env::set_var("http_proxy", "");
            std::env::set_var("https_proxy", "");
            val
        }
        Err(_e) => String::from("proxy setting error"),
    };
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
    } else if args.disable_gui {
        // Run CLI Tool
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(cli::run(args))
    } else {
        let (async_proc_input_tx, async_proc_input_rx) = mpsc::channel(1);
        let (async_proc_output_tx, mut async_proc_output_rx) = mpsc::channel(1);
        tauri::Builder::default()
            .manage(AppState {
              inner: Mutex::new(async_proc_input_tx),
              file_data: Mutex::new(None),
            })
            .setup(|app| {
              tauri::async_runtime::spawn(async move {
                async_process_model(async_proc_input_rx, async_proc_output_tx).await
              });
              let app_handle = app.handle();
              tauri::async_runtime::spawn(async move {
                  loop {
                      if let Some(output) = async_proc_output_rx.recv().await {
                        if output.0 == InternalCommand::Open {
                          if let Ok(mut port) = SerialPort::open(output.1, BAUD_RATE) {
                            // Todo: フロントへの接続成功通知の実装
                            println!("Connect Success.");
                            serial_com::clear_buffer(&mut port);
                            loop {
                              tokio::select!(
                                Some(output) = async_proc_output_rx.recv() => {
                                  // フロントからのイベント
                                  if handle_internal_control(output.0,&mut port,&app_handle).await {
                                    serial_com::clear_buffer(&mut port);
                                    break;
                                  }
                                }
                                Ok(v) = serial_com::receive_byte(&mut port) => {
                                  // Sequencerとの独自プロトコルの通信
                                  if let Some(sq_msg) = serial_com::receive_sequence_msg(v, &mut port).await {
                                    handle_sequence_msg(sq_msg, &app_handle);
                                  }
                                }
                              );
                            }
                          } else {
                            // Todo: フロントへの接続失敗通知の実装
                            println!("faild open port");
                          }
                        }
                      }
                  }
              });
              Ok(())
            })
            .invoke_handler(tauri::generate_handler![
                read_file,
                // process_event,
                send_file_size, // 本番用
                open_file,
                set_serial_port,
                disconnect_serial_port,
            ])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
    // reset env proxy
    if !proxy_env_value.is_empty() {
        std::env::set_var("http_proxy", proxy_env_value.as_str());
        std::env::set_var("https_proxy", proxy_env_value.as_str());
    }
}

// Asyncの世界とのやり取り
async fn async_process_model(
    mut input_rx: mpsc::Receiver<(InternalCommand, String)>,
    output_tx: mpsc::Sender<(InternalCommand, String)>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    while let Some(input) = input_rx.recv().await {
        let output = input;
        output_tx.send(output).await?;
    }
    Ok(())
}
