use std::{
    fs::DirEntry,
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
    if !info.get_header().is_format0() {
        return Err(MidiError::Custom(
            "Unsupported MIDI format detected (must be Format 0).",
        ));
    }
    if info
        .get_tracks()
        .first()
        .unwrap()
        .iter()
        .any(|event| event.get_ch() > 6)
    {
        return Err(MidiError::Custom(
            "Unsupported MIDI channel detected (must be 1-6).",
        ));
    }
    Ok(info)
}
// MIDIファイルからタイトル情報を取得する
pub fn get_title(data: &MidiInfo) -> Option<String> {
    // Note: not implemented
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

pub fn generate_suggestion(input: impl AsRef<str>) -> (String, Option<Vec<DirItem>>) {
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
    let (dir, prefix) = if path.is_dir() {
        (path.to_path_buf(), None)
    } else if let (Some(parent), Some(name)) =
        (path.parent(), path.file_name().and_then(|s| s.to_str()))
    {
        (parent.to_path_buf(), Some(name))
    } else {
        return (input_str.to_string(), None);
    };

    let entries = if let Ok(read_dir) = std::fs::read_dir(dir) {
        read_dir
            .filter_map(Result::ok)
            .map(DirItem::from)
            .collect::<Vec<_>>()
    } else {
        eprintln!("no files...");
        return (input_str.to_string(), None);
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
        0 => (input_str.to_string(), None),
        1 => {
            let item = filtered_entries.into_iter().next().unwrap();
            let is_dir = item.path().is_dir();
            item.as_str()
                .map(|p_str| (p_str.to_string() + if is_dir { "/" } else { "" }, None))
                .unwrap_or_else(|| (input_str.to_string(), None))
        }
        _ => (input_str.to_string(), Some(filtered_entries)),
    }
}
