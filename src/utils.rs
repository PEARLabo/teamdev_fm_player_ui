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
