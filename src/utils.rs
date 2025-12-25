use std::{
    ffi::OsStr,
    fs::{DirEntry, read_dir},
    io::BufRead,
    path::{Path, PathBuf},
    str::FromStr,
};

use serial2_tokio::SerialPort;

use crate::midi::{MidiError, MidiInfo};
#[derive(Debug, Default)]
pub struct ValidationConf {
    // enable Ch7, Ch8, Ch9 and Ch10 (FM and Rhythm)
    // adpcm ch is unsupported
    pub ym2608: bool,
    // more options ...
}

pub fn open_serial_port(port: impl AsRef<str>, baud_rate: u32) -> Result<SerialPort, String> {
    // let  = 115200;
    let port_setting = SerialPort::open(port.as_ref(), baud_rate);
    if port_setting.is_err() {
        return Err("failed to open serial port".to_string());
    }

    Ok(port_setting.unwrap())
}

#[inline]
pub fn u32_from(data: &[u8]) -> u32 {
    (data[0] as u32) << 24 | (data[1] as u32) << 16 | (data[2] as u32) << 8 | (data[3] as u32)
}

#[inline]
pub fn u32_from_le(data: &[u8]) -> u32 {
    (data[0] as u32) | (data[1] as u32) << 8 | (data[2] as u32) << 16 | (data[3] as u32) << 24
}

pub fn validation_midi_file(data: &[u8], conf: &ValidationConf) -> Result<MidiInfo, MidiError> {
    let max_ch = if conf.ym2608 { 10 } else { 6 };
    let info = MidiInfo::try_from(data)?;
    if info.get_header().format() == crate::midi::Format::Format2 {
        return Err(MidiError::Custom(String::from(
            "Unsupported MIDI format detected (must be Format 0 or 1).",
        )));
    }
    if info
        .get_tracks()
        .iter()
        .any(|events| events.iter().any(|event| event.ch() > max_ch))
    {
        return Err(MidiError::Custom(format!(
            "Unsupported MIDI channel detected (must be 1-{max_ch})."
        )));
    }
    Ok(info)
}
// MIDIファイルからタイトル情報を取得する
pub fn get_title(info: &MidiInfo) -> Option<String> {
    // Note: トラック名を取得してしまうので、一度無効化
    // 曲タイトルとトラック名を区別することができれば...
    if false {
        // Conductor or track 1
        let track = info.get_tracks().first().unwrap();
        for event in track {
            if event.status_byte() == 0xff {
                let data = event.data();
                if data[0] == 0x03 && data[1] != 0 {
                    let mut i = 1;
                    while data[i] & 0x80 != 0 {
                        i += 1;
                    }
                    i += 1;
                    return Some(String::from_utf8_lossy(&data[i..]).to_string());
                } else {
                    break;
                }
            }
        }
        None
    } else {
        None
    }
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
#[derive(Debug)]
pub struct DirItem {
    path: PathBuf,
}
impl DirItem {
    pub fn path(&self) -> &Path {
        &self.path
    }
    pub fn as_str(&self) -> Option<&str> {
        self.path.to_str()
    }
    pub fn get_file_name(&self) -> Option<&str> {
        self.path.file_name()?.to_str()
    }
}
impl From<DirEntry> for DirItem {
    fn from(entry: DirEntry) -> Self {
        Self { path: entry.path() }
    }
}
pub struct UpdateResult<T> {
    pub content: T,
    pub changed: bool,
}
impl<T> UpdateResult<T> {
    fn updated(data: T) -> Self {
        Self {
            content: data,
            changed: true,
        }
    }
    fn not_updated(data: T) -> Self {
        Self {
            content: data,
            changed: false,
        }
    }
    pub fn is_update(&self) -> bool {
        self.changed
    }
    pub fn get_content(&self) -> &T {
        &self.content
    }
}
pub fn generate_suggestion(input: impl AsRef<str>) -> (UpdateResult<String>, Option<Vec<DirItem>>) {
    let input_str = input.as_ref();
    let path = if input_str.is_empty() {
        PathBuf::from_str("./").unwrap()
    } else if !(input_str.starts_with("./")
        || input_str.starts_with("../")
        || input_str.starts_with("/"))
    {
        Path::new("./").join(input_str)
    } else {
        PathBuf::from_str(input_str).unwrap()
    };
    // 1. ディレクトリとプレフィクスに分離
    // e.g. `./hoge/fuga` -> `./hoge`, `fuga`
    let (dir, prefix) = if input_str.ends_with(".") {
        // ./.を'./'と'.'に分離する
        // Rustが'./.'をカレントディレクトリと読んでしまう
        let mut dir = path.parent().unwrap_or(Path::new("."));
        if dir.to_str().unwrap().is_empty() {
            dir = Path::new(".");
        }
        let fname = path
            .file_name()
            .unwrap_or(OsStr::new("."))
            .to_str()
            .unwrap();
        (dir.to_path_buf(), Some(fname))
    } else if path.is_dir() {
        if input_str.ends_with("/") {
            (path.to_path_buf(), None)
        } else if let Some(parent) = path.parent() {
            // 入力がディレクトリと完全一致時にほかの候補があるか確認する
            // e.g ./.git -> ./.git/ or ./.gitconfig
            let name = path.file_name().and_then(|s| s.to_str());
            let entries = if let Ok(entries) = read_dir(parent) {
                let name_prefix = name.unwrap_or("");
                entries
                    .filter_map(Result::ok) // Convert iterator of Results to iterator of DirEntry
                    .filter(|entry| entry.file_name().to_string_lossy().starts_with(name_prefix))
                    .count()
            } else {
                0
            };
            // 部分一致のファイル or ディレクトリが存在するときは親ディレクトリを対象に
            if entries > 1 {
                (parent.to_path_buf(), name)
            } else {
                (path.to_path_buf(), None)
            }
        } else {
            (path.to_path_buf(), None)
        }
    } else if let (Some(parent), Some(name)) =
        (path.parent(), path.file_name().and_then(|s| s.to_str()))
    {
        (parent.to_path_buf(), Some(name))
    } else {
        return (UpdateResult::not_updated(input_str.to_string()), None);
    };
    // 2. ディレクトリからリストを取得
    let entries = if let Ok(read_dir) = std::fs::read_dir(&dir) {
        read_dir
            .filter_map(Result::ok)
            .map(DirItem::from)
            .collect::<Vec<_>>()
    } else {
        eprintln!("no files...");
        return (UpdateResult::not_updated(input_str.to_string()), None);
    };
    // 3. ファイル名でフィルタリング
    let filtered_entries: Vec<DirItem> = if let Some(prefix) = prefix {
        entries
            .into_iter()
            .filter(|item| {
                item.get_file_name()
                    .is_some_and(|name| name.starts_with(prefix))
            })
            .collect()
    } else {
        entries
    };
    // 4. 候補のリストと、補間結果を返す
    match filtered_entries.len() {
        0 => (UpdateResult::not_updated(input_str.to_string()), None),
        1 => {
            let item = filtered_entries.into_iter().next().unwrap();
            let is_dir = item.path().is_dir();
            item.as_str()
                .map(|p_str| {
                    (
                        UpdateResult::updated(p_str.to_string() + if is_dir { "/" } else { "" }),
                        None,
                    )
                })
                .unwrap_or_else(|| (UpdateResult::not_updated(input_str.to_string()), None))
        }
        _ => match get_common_filename_prefix(&filtered_entries) {
            // 全候補における完全一致部分までを補完する
            Some(common) if !common.is_empty() && Some(common.as_str()) != prefix => {
                let path = dir.join(common);
                (
                    UpdateResult::updated(path.to_string_lossy().to_string()),
                    Some(filtered_entries),
                )
            }
            _ => (
                UpdateResult::not_updated(input_str.to_string()),
                Some(filtered_entries),
            ),
        },
    }
}
// ファイルリストでファイル名の先頭からの一致部分を返す
// e.g
//   list: teamdev_fm_player_ui/ teamdev_fm_sequencer/
//   output: teamdev_fm_
pub fn get_common_filename_prefix(items: &[DirItem]) -> Option<String> {
    let mut names = items.iter().map(|item| item.get_file_name());

    let first = names.next().flatten()?.to_string();

    let final_prefix = names.try_fold(first, |mut prefix, name| {
        let name = name?;
        let common_len = prefix
            .chars()
            .zip(name.chars())
            .take_while(|(pc, nc)| pc == nc)
            .count();
        prefix.truncate(
            prefix
                .char_indices()
                .nth(common_len)
                .map_or(prefix.len(), |(idx, _)| idx),
        );
        Some(prefix)
    })?;

    if final_prefix.is_empty() {
        None
    } else {
        Some(final_prefix)
    }
}

// (IP Addr, Port)
type SocketAddrInfo = (Option<String>, Option<String>);
pub struct SocketInfo {
    pub tx_addr: String,
    pub rx_addr: String,
}
// 受信アドレス/ポート情報は自動生成可能
// 送信アドレス/ポートは必須
pub fn get_udp_addrinfo(tx_info: SocketAddrInfo, rx_info: SocketAddrInfo) -> Option<SocketInfo> {
    if let Some(tx_port) = tx_info.0.as_ref().and(tx_info.1) {
        let tx_addr = tx_info.0.unwrap() + ":" + &tx_port;
        // RX Addrのデフォルトは0.0.0.0
        let rx_addr = rx_info.0.unwrap_or(String::from("0.0.0.0"));
        // Portの指定がなければ、tx portの次
        Some(SocketInfo {
            tx_addr,
            rx_addr: rx_addr
                + ":"
                + &rx_info
                    .1
                    .unwrap_or((tx_port.parse::<usize>().unwrap() + 1).to_string()),
        })
    } else {
        None
    }
}
