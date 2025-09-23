use std::{fs::File, io::Read};

use crate::midi::{MidiError, MidiInfo};
#[derive(Clone)]
pub enum Error {
    FileOpen(String),
    Format(String),
    FileNotExist(String),
    MidiError(MidiError),
}
impl Error {
    pub fn to_msg(&self) -> String {
        match self {
            Self::FileOpen(path) => format!("Failed to open file {path}."),
            Self::Format(path) => format!("File format Error: {path} is not MIDI Format 0."),
            Self::FileNotExist(path) => format!("File Not Exist: {path}"),
            Self::MidiError(e) => e.to_string(),
        }
    }
}

/**
 *  Return MidiInfo and raw data if a valid file path is provided.
 *  return error when file is not exist
 */
pub fn file_check(path: Option<impl AsRef<str>>) -> Result<Option<(MidiInfo, Vec<u8>)>, Error> {
    if let Some(path) = path {
        let path = path.as_ref();
        let mut file = if let Ok(f) = File::open(path) {
            Ok(f)
        } else {
            Err(Error::FileOpen(path.to_string()))
        }?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        let midi_info = crate::utils::validation_midi_file(&buf);
        if let Ok(midi_info) = midi_info {
            Ok(Some((midi_info, buf)))
        } else {
            Err(Error::MidiError(midi_info.unwrap_err()))
        }
    } else {
        // ファイルパスが与えられていない
        Ok(None)
    }
}
