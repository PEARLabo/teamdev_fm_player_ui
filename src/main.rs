mod char_code_lut;
mod cli;

mod midi;
mod sequence_msg;
mod serial_com;
mod utils;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    // #[arg(long)]
    // disable_gui: bool,
    #[arg(short, long)]
    input: Option<String>,
    #[arg(short, long)]
    list: bool,
    #[arg(short, long, default_value_t = 0)]
    port: usize,
    #[arg(long)]
    port_name: Option<String>,
}

struct ToFrontMsg {
    msg: String,
    id: Option<u16>,
}

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
    if args.list {
        // Print the list of available ports
        if let Some(list) = utils::get_serial_port_list() {
            if list.is_empty() {
                println!("No serial port found");
            } else {
                list.iter().enumerate().for_each(|(i, port)| {
                    println!("{i}: {port}");
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
