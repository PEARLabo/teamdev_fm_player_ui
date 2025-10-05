mod char_code_lut;
mod cli;
mod midi;
mod sequence_msg;
mod serial_com;
mod utils;
use clap::Parser;
mod file_ctrl;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    input: Option<String>,
    #[arg(short, long)]
    list: bool,
    #[arg(short, long, default_value_t = 0)]
    port: usize,
    #[arg(long)]
    port_name: Option<String>,
    // ignore text message
    #[arg(long)]
    ignore_text: bool,
    // Remove SysExEvents
    #[arg(long)]
    ignore_sysex: bool,
    #[arg(long)]
    ym2608: bool,
    #[arg(long, default_value_t = 115200)]
    baud_rate: u32,
}

// アプリケーションのエントリーポイント
fn main() {
    const _BAUD_RATE: u32 = 115200;
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
            .unwrap();
    }
}
