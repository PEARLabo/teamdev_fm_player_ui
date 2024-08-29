use tokio::io::AsyncReadExt;

fn convert_to_bpm(data: &[u8]) -> u32 {
    let usec_per_beat =
        ((data[0] as u32) | ((data[1] as u32) << 8) | ((data[2] as u32) << 16)) as usize;
    (60000000usize / usec_per_beat) as u32
}

#[derive(serde::Serialize)]
pub struct SequenceMsg {
    channel: u8,
    sq_event: SequenceEventFlag,
    param_change: Option<ParamChangeFlag>,
    data: Option<Vec<u8>>,
}
impl<'a> From<&'a [u8]> for SequenceMsg {
    fn from(data: &'a [u8]) -> Self {
        let event_flag = SequenceEventFlag::from(data[0] & 0xf);
        let ch = (data[0] >> 4) & 0xf;
        if event_flag == SequenceEventFlag::Param {
            Self::new_param_change(ch, ParamChangeFlag::from(data[1] & 0xf), data[2..].to_vec())
        } else {
            Self::new(ch, event_flag, Some(data[1..].to_vec()))
        }
    }
}
impl std::fmt::Display for SequenceMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = self.get_data().unwrap_or(&[0; 1]);
        match self.sq_event {
            SequenceEventFlag::KeyEvent => {
                if data[1] == 0 {
                    // Key Off
                    write!(f, "Ch{:2}: Key Off        {}", self.channel, data[0])
                } else {
                    // Key On
                    write!(f, "Ch{:2}: Key On         {}", self.channel, data[0])
                }
            }
            SequenceEventFlag::Tempo => {
                write!(f, "Tempo: {} BPM", convert_to_bpm(data))
            }
            SequenceEventFlag::End => write!(f, "End"),
            SequenceEventFlag::Nop => write!(f, "Ch{:2}: NOP", self.channel),
            SequenceEventFlag::Param => {
                write!(
                    f,
                    "Ch{:2}: Set {} {:#02X}",
                    self.channel,
                    self.param_change.unwrap(),
                    data[0]
                )
            }
            SequenceEventFlag::ProgramChange => {
                write!(
                    f,
                    "Ch{:2}: Program Change [{}]",
                    self.channel,
                    std::str::from_utf8(data).unwrap_or("unknown")
                )
            }
            SequenceEventFlag::Expression => {
                write!(f, "Ch{:2}: Expression     {}", self.channel, data[0])
            }

            _ => write!(f, ""),
        }
    }
}

impl SequenceMsg {
    pub fn new(channel: u8, event: SequenceEventFlag, data: Option<Vec<u8>>) -> Self {
        Self {
            channel,
            sq_event: event,
            param_change: None,
            data,
        }
    }
    fn new_param_change(channel: u8, param_change: ParamChangeFlag, data: Vec<u8>) -> Self {
        Self {
            channel,
            sq_event: SequenceEventFlag::Param,
            param_change: Some(param_change),
            data: Some(data),
        }
    }
    fn get_data(&self) -> Option<&[u8]> {
        if let Some(data) = &self.data {
            Some(data.as_slice())
        } else {
            None
        }
    }
}
#[derive(serde::Serialize, PartialEq)]
pub enum SequenceEventFlag {
    KeyEvent,
    Tempo,
    End,
    Nop,
    Param,
    ProgramChange,
    Expression,
    Other, // これhあ値不定。イベント追加で変動
}

impl From<u8> for SequenceEventFlag {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::KeyEvent,
            1 => Self::Tempo,
            2 => Self::End,
            3 => Self::Nop,
            4 => Self::Param,
            5 => Self::ProgramChange,
            6 => Self::Expression,
            _ => Self::Other,
        }
    }
}

impl SequenceEventFlag {
    pub fn into_u8(self) -> u8 {
        match self {
            Self::KeyEvent => 0,
            Self::Tempo => 1,
            Self::End => 2,
            Self::Nop => 3,
            Self::Param => 4,
            Self::ProgramChange => 5,
            Self::Expression => 6,
            _ => 0xff,
        }
    }
}
#[derive(serde::Serialize, Clone, Copy)]
pub enum ParamChangeFlag {
    Slot,
    DtMul,
    Tl,
    KsAr,
    Dr,
    Sr,
    SlRr,
    FbCon,
    Other,
}
impl std::fmt::Display for ParamChangeFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Slot => write!(f, "Slot"),
            Self::DtMul => write!(f, "Detune/Multiple"),
            Self::Tl => write!(f, "TotalLevel"),
            Self::KsAr => write!(f, "KeyScale/AttackRate"),
            Self::Dr => write!(f, "DecayRate"),
            Self::Sr => write!(f, "SustainRate"),
            Self::SlRr => write!(f, "SustainLevel/ReleaseRate"),
            Self::FbCon => write!(f, "FeedBack/Connection"),
            _ => write!(f, "other"),
        }
    }
}
impl From<u8> for ParamChangeFlag {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Slot,
            1 => Self::DtMul,
            2 => Self::Tl,
            3 => Self::KsAr,
            4 => Self::Dr,
            5 => Self::Sr,
            6 => Self::SlRr,
            7 => Self::FbCon,
            _ => Self::Other,
        }
    }
}
impl ParamChangeFlag {
    pub fn into_u8(self) -> u8 {
        match self {
            Self::Slot => 0,
            Self::DtMul => 1,
            Self::Tl => 2,
            Self::KsAr => 3,
            Self::Dr => 4,
            Self::Sr => 5,
            Self::SlRr => 6,
            Self::FbCon => 7,
            _ => 0xff,
        }
    }
}
