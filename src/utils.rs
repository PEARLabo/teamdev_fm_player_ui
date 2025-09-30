use std::{
    ffi::OsStr,
    fs::{DirEntry, read_dir},
    io::BufRead,
    path::{Path, PathBuf},
    str::FromStr,
};

use serial2_tokio::SerialPort;

use crate::midi::{MidiError, MidiInfo};

pub fn open_serial_port(port: impl AsRef<str>) -> Result<SerialPort, String> {
    let baud_rate = 115200;
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

pub fn validation_midi_file(data: &[u8]) -> Result<MidiInfo, MidiError> {
    let info = MidiInfo::try_from(data)?;
    if info.get_header().format() == crate::midi::Format::Format2 {
        return Err(MidiError::Custom(
            "Unsupported MIDI format detected (must be Format 0 or 1).",
        ));
    }
    if info
        .get_tracks()
        .iter()
        .any(|events| events.iter().any(|event| event.ch() > 6))
    {
        return Err(MidiError::Custom(
            "Unsupported MIDI channel detected (must be 1-6).",
        ));
    }
    Ok(info)
}
// MIDIファイルからタイトル情報を取得する
pub fn get_title(info: &MidiInfo) -> Option<String> {
    return None;
    // Note: トラック名を取得してしまうので、一度無効化
    // 曲タイトルとトラック名を区別することができれば...
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
    let (dir, prefix) = if input_str.ends_with(".") {
        // `.`スタートのファイル名がうまく取れないので、特別扱い
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
            // 入力と完全一致ディレクトリがある場合で、そのほかの　ファイル名も存在する場合
            // ディレクトリ列挙になるのを防ぐ
            let name = path.file_name().and_then(|s| s.to_str());
            let entries = if let Ok(entries) = read_dir(parent) {
                entries
                    .filter(|item| {
                        item.as_ref()
                            .unwrap()
                            .path()
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .starts_with(name.unwrap())
                    })
                    .count()
            } else {
                0
            };
            // 部分一致のファイルor ディレクトリが存在するときは親ディレクトリを対象に
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

    let entries = if let Ok(read_dir) = std::fs::read_dir(dbg!(&dir)) {
        read_dir
            .filter_map(Result::ok)
            .map(DirItem::from)
            .collect::<Vec<_>>()
    } else {
        eprintln!("no files...");
        return (UpdateResult::not_updated(input_str.to_string()), None);
    };
    dbg!(&entries);
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
        _ => {
            let common_fname = get_common_filename_prefix(&filtered_entries);
            if let Some(common_fname) = dbg!(common_fname) {
                (
                    UpdateResult::updated(format!("{}/{}", dir.to_str().unwrap(), common_fname)),
                    Some(filtered_entries),
                )
            } else {
                (
                    UpdateResult::not_updated(input_str.to_string()),
                    Some(filtered_entries),
                )
            }
        }
    }
}

pub fn get_common_filename_prefix(items: &[DirItem]) -> Option<String> {
    if items.is_empty() {
        return None;
    }

    let names: Vec<Option<&str>> = items.iter().map(|item| item.get_file_name()).collect();
    if names.iter().any(|name| name.is_none()) {
        return None;
    }
    let names: Vec<&str> = names.into_iter().map(|name| name.unwrap()).collect();

    let mut prefix = String::from(names[0]);

    for name in names.iter().skip(1) {
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
    }

    if prefix.is_empty() {
        None
    } else {
        Some(prefix)
    }
}
