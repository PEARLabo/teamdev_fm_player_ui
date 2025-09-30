use std::{
    collections::{HashMap, VecDeque},
    io::Write,
};

use crate::utils::u32_from;
const fn to_u16(h: u8, l: u8) -> u16 {
    ((h as u16) << 8) | (l as u16)
}
#[derive(Debug, Clone, Copy)]
pub enum MidiError {
    InvalidFileFormat(u8), // 0: HEADER / 1: Track
    InvalidHeader,
    UnknownFormat,
    LengthError,
    UnknownEventFormat(u8),
    UnknownEvent(u8),
    Custom(&'static str),
}
impl std::fmt::Display for MidiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFileFormat(s) => write!(
                f,
                "Invalid File Format: {} is broken",
                if *s == 0 {
                    "Header"
                } else if *s == 1 {
                    "Track"
                } else {
                    "unknown"
                }
            ),
            Self::InvalidHeader => write!(f, "Invalid Header"),
            Self::UnknownFormat => write!(f, "Unknown Format"),
            Self::LengthError => write!(f, "Length Error"),
            Self::UnknownEventFormat(e) => write!(f, "Unknown Event Format: {:#x}", e),
            Self::UnknownEvent(e) => write!(f, "Unknown Event: {:#x}", e),
            Self::Custom(s) => write!(f, "{s}"),
        }
    }
}
#[derive(Debug, Default)]
pub struct MidiConfig {
    // SysExの形式をDominoフォーマットへ変更する
    pub ignore_text: bool,
    // SysExメッセージを除去する
    pub sysex_ignore: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Format {
    Format0,
    Format1,
    Format2,
}
impl TryFrom<u16> for Format {
    type Error = MidiError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Format0),
            1 => Ok(Self::Format1),
            2 => Ok(Self::Format2),
            _ => Err(MidiError::UnknownFormat),
        }
    }
}
#[derive(Debug)]
pub struct MidiInfo {
    header: MidiHeader,
    // track: u32,
    tracks: Vec<Vec<MidiEvent>>,
}

impl MidiInfo {
    pub fn get_header(&self) -> &MidiHeader {
        &self.header
    }
    pub fn get_tracks(&self) -> &[Vec<MidiEvent>] {
        &self.tracks
    }
    pub fn construct(&self, config: &MidiConfig) -> Vec<u8> {
        let header = self.header.to_binary();
        let track = self
            .tracks
            .iter()
            .flat_map(|events| {
                // 5はマジックナンバー(Geminiの答えるイベントの平均バイト数)
                let mut data: Vec<u8> = Vec::with_capacity(events.len() * 5 + 8);
                data.extend([0x4d, 0x54, 0x72, 0x6b]);
                let track_body = events
                    .iter()
                    .filter_map(|event| {
                        if config.sysex_ignore && event.is_sysex() {
                            None
                        } else if config.ignore_text && event.is_meta() && event.data()[0] < 0x0A {
                            None
                        } else {
                            Some(event.clone())
                        }
                    })
                    .flat_map(|event| event.to_binary())
                    .collect::<Vec<u8>>();
                let len = track_body.len();
                data.extend([
                    (len >> 24) as u8,
                    (len >> 16) as u8,
                    (len >> 8) as u8,
                    len as u8,
                ]);
                data.extend(track_body);
                data
            })
            .collect();
        let data = [header, track].concat();
        // DEBUG OUT
        // {
        //     let file = std::fs::File::create("converted.mid").unwrap();
        //     let mut writer = std::io::BufWriter::new(file);
        //     writer.write_all(&data).unwrap();
        //     writer.flush().unwrap();
        // }
        data
    }
    pub fn convert_to_format0(&self) -> Self {
        let header = MidiHeader {
            format: Format::Format0,
            time_unit: self.header.time_unit,
            tracks: 1,
        };
        let mut track = Vec::new();
        let mut tick = 0;
        let mut event_queues = self
            .get_tracks()
            .iter()
            .map(|events| VecDeque::from(events.to_vec()))
            .collect::<Vec<_>>();
        let mut next_ticks = (0..self.tracks.len())
            .map(|i| (i, event_queues[i].front().as_ref().unwrap().delta_time))
            .collect::<HashMap<usize, usize>>();
        let is_all_empty =
            |list: &[VecDeque<MidiEvent>]| -> bool { list.iter().all(|queue| queue.is_empty()) };
        while !is_all_empty(&event_queues) {
            let (idx, delta_time) = {
                let (i, d) = next_ticks
                    .iter()
                    .min_by_key(|&(_, delta_time)| delta_time)
                    .unwrap();
                (*i, *d)
            };
            // イベントの登録
            let mut event = event_queues[dbg!(idx)].pop_front().unwrap();

            event.delta_time = delta_time;
            tick += delta_time;
            if !event.is_end_of_track() {
                track.push(event);
            }
            // delta timeの更新
            next_ticks.iter_mut().for_each(|(_, d)| {
                *d = d.saturating_sub(delta_time);
            });
            // 探索対処言うから削除 / 次の待ち時間を登録
            if event_queues[idx].is_empty() {
                next_ticks.remove(&idx);
            } else {
                next_ticks.insert(idx, event_queues[idx].front().unwrap().delta_time);
            }
        }
        Self {
            header,
            tracks: vec![track],
        }
    }
}

#[derive(Debug)]
pub struct MidiHeader {
    format: Format,
    time_unit: u16,
    tracks: u16,
}
impl MidiHeader {
    fn to_binary(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(14);
        data.extend([0x4D, 0x54, 0x68, 0x64]);
        data.extend([0x00, 0x00, 0x00, 0x06]);
        data.extend([0x00, self.format as u8]);
        data.extend([(self.tracks >> 8) as u8, self.tracks as u8]);
        data.extend([(self.time_unit >> 8) as u8, self.time_unit as u8]);
        data
    }

    pub(crate) fn format(&self) -> Format {
        self.format
    }
}
#[derive(Debug, Clone)]
pub struct MidiEvent {
    delta_time: usize,
    ch: u8,
    status_byte: u8,
    data: Vec<u8>,
}

impl MidiEvent {
    pub fn ch(&self) -> u8 {
        self.ch
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    pub fn to_binary(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(self.data.len() + 5);
        data.extend(convert_variable_value(self.delta_time));
        data.push(if self.status_byte & 0xf0 == 0xf0 {
            self.status_byte
        } else {
            self.status_byte | self.ch
        });
        data.extend_from_slice(&self.data);
        data
    }

    fn is_end_of_track(&self) -> bool {
        self.status_byte == 0xff && self.data[0] == 0x2f && self.data[1] == 0x00
    }

    pub fn status_byte(&self) -> u8 {
        self.status_byte
    }
    pub fn new_with(&self, data: &[u8]) -> Self {
        Self {
            delta_time: self.delta_time,
            ch: self.ch,
            status_byte: self.status_byte,
            data: data.to_vec(),
        }
    }
    fn is_sysex(&self) -> bool {
        self.status_byte == 0xf0 || self.status_byte == 0xf7
    }

    fn is_meta(&self) -> bool {
        self.status_byte == 0xff
    }
}

impl TryFrom<&[u8]> for MidiHeader {
    type Error = MidiError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        if data[0..4] != [0x4d, 0x54, 0x68, 0x64] {
            return Err(MidiError::InvalidFileFormat(0));
        }
        if u32_from(&data[4..]) != 6 {
            return Err(MidiError::InvalidHeader);
        }
        let format = Format::try_from(to_u16(data[8], data[9]))?;
        let time_unit = to_u16(data[12], data[13]);
        let tracks = to_u16(data[10], data[11]);
        Ok(Self {
            format,
            time_unit,
            tracks,
        })
    }
}

impl TryFrom<&[u8]> for MidiInfo {
    type Error = MidiError;
    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let header = MidiHeader::try_from(data)?;
        // let mut events = Vec::new();
        let mut track_idx = 0;
        let mut track_data = &data[14..];
        eprintln!("track data len  = {}", track_data.len());
        let mut tracks = Vec::with_capacity(header.tracks as usize);
        while dbg!(track_idx) < header.tracks {
            if track_data[0..4] != [0x4d, 0x54, 0x72, 0x6b] {
                eprintln!("{:?}", &track_data[0..4]);
                return Err(MidiError::InvalidFileFormat(1));
            }
            let track_size = u32_from(&track_data[4..]) as usize;
            let raw_events = &track_data[8..8 + track_size];
            tracks.push(parse_track(raw_events)?);
            // let track_size = u32_from(&track_data[4..]) as usize;)
            track_data = &track_data[8 + track_size..];
            track_idx += 1;
        }

        Ok(Self { header, tracks })
    }
}
fn parse_track(raw_events: &[u8]) -> Result<Vec<MidiEvent>, MidiError> {
    let mut i = 0;
    let mut events = Vec::new();
    let mut before_status = 0;
    while i < raw_events.len() {
        let (delta_time, len) = get_variable_value(&raw_events[i..])?;
        i += len;
        let ch = raw_events[i] & 0x0f;
        let status_byte = raw_events[i] & 0xf0;
        // this switch process is generated by Gemini
        match status_byte {
            0x80 | 0x90 | 0xA0 | 0xB0 | 0xE0 => {
                before_status = raw_events[i];
                let mut event_data = vec![0; 2];
                event_data[0] = raw_events[i + 1];
                event_data[1] = raw_events[i + 2];
                i += 3;
                events.push(MidiEvent {
                    delta_time,
                    ch,
                    status_byte,
                    data: event_data,
                });
            }
            0xC0 | 0xD0 => {
                before_status = raw_events[i];
                let mut event_data = vec![0; 1];
                event_data[0] = raw_events[i + 1];
                i += 2;
                events.push(MidiEvent {
                    delta_time,
                    ch,
                    status_byte,
                    data: event_data,
                });
            }
            0xf0 => {
                before_status = 0;
                let status_byte = raw_events[i];
                match raw_events[i] {
                    0xf7 => {
                        let mut event_data = Vec::new();
                        i += 1;
                        let (meta_len, len_bytes) = get_variable_value(&raw_events[i + 2..])?;
                        event_data.extend_from_slice(&raw_events[i..(i + len_bytes + meta_len)]);
                        i += len_bytes + meta_len;
                    }
                    0xf0 => {
                        let mut event_data = Vec::new();
                        i += 1;
                        while raw_events[i] != 0xf7 {
                            event_data.push(raw_events[i]);
                            i += 1;
                        }
                        event_data.push(0xf7);
                        events.push(MidiEvent {
                            delta_time,
                            ch: 0,
                            status_byte,
                            data: event_data,
                        });
                        i += 1; // 0xf7をスキップ
                    }
                    0xff => {
                        // Meta Event
                        let meta_type = raw_events[i + 1];
                        let (meta_len, len_bytes) = get_variable_value(&raw_events[i + 2..])?;
                        let mut event_data = Vec::with_capacity(2 + meta_len + len_bytes);
                        event_data.push(meta_type);
                        event_data
                            .extend_from_slice(&raw_events[i + 2..(i + 2 + len_bytes + meta_len)]);
                        i += 2 + len_bytes + meta_len;
                        events.push(MidiEvent {
                            delta_time,
                            // Non Channel
                            ch: 0,
                            status_byte,
                            data: event_data,
                        });
                    }
                    e => return Err(MidiError::UnknownEventFormat(e)),
                }
            }
            e if e < 0x80 => {
                // Running Status
                // if before_status != 0 {
                let ch = before_status & 0x0f;
                let event = before_status & 0xf0;
                match event {
                    0x80 | 0x90 | 0xA0 | 0xB0 | 0xE0 => {
                        let mut event_data = vec![0; 2];
                        event_data[0] = raw_events[i];
                        event_data[1] = raw_events[i + 1];
                        i += 2;
                        events.push(MidiEvent {
                            delta_time,
                            ch,
                            status_byte: event,
                            data: event_data,
                        });
                    }
                    0xC0 | 0xD0 => {
                        let mut event_data = vec![0; 1];
                        event_data[0] = raw_events[i];
                        i += 1;
                        events.push(MidiEvent {
                            delta_time,
                            ch,
                            status_byte: event,
                            data: event_data,
                        });
                    }
                    _ => return Err(MidiError::UnknownEventFormat(before_status)),
                }
            }
            e => {
                eprintln!("{:?} - delta_time:{} - ch: {:#x}", events, delta_time, ch);
                return Err(MidiError::UnknownEvent(e));
            }
        }
    }
    Ok(events)
}
// return: (data, length)
fn get_variable_value(data: &[u8]) -> Result<(usize, usize), MidiError> {
    let mut value = 0;
    let mut len = 0;
    loop {
        // データが足りない場合のエラーハンドリング
        if len >= data.len() {
            return Err(MidiError::LengthError); // もしくはより適切なエラー
        }
        // 4バイトを超えるVLQは不正
        if len > 4 {
            return Err(MidiError::LengthError);
        }

        let byte = data[len] as usize;
        len += 1;

        // 既存の値を7ビット左シフトし、現在のバイトの下位7ビットを合成する
        value = (value << 7) | (byte & 0x7F);

        // 現在のバイトのMSBが0であれば、これが最後のバイト
        if byte & 0x80 == 0 {
            break;
        }
    }
    Ok((value, len))
}
fn convert_variable_value(value: usize) -> Vec<u8> {
    let mut data = Vec::new();
    let mut v = value;
    let mut flag = 0;
    loop {
        let byte = v & 0x7F;
        v >>= 7;
        data.push(byte as u8 | flag);
        flag = 0x80;
        if v == 0 {
            break;
        }
    }
    data.reverse();
    data
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_variable_value() {
        assert_eq!(vec![1], convert_variable_value(1));
        assert_eq!(vec![0x7f], convert_variable_value(0x7f));
        assert_eq!(vec![0x81, 0x00], convert_variable_value(0x80));
    }
}
