use tokio::io::AsyncReadExt;
enum SequenceEventFlag {
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
    fn into_u8(self) -> u8 {
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
enum ParamChangeFlag {
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
            Self::Slot => write!(f, "slot"),
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
    fn into_u8(self) -> u8 {
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
// TODO: フロントへのイベント発行の実装
pub async fn sequence_msg<R: tauri::Runtime>(
    first_byte: u8,
    port: &mut serial2_tokio::SerialPort,
    app_handle: &impl tauri::Manager<R>,
) {
    let msg_flag = first_byte & 0xf;
    let len = (first_byte >> 4) as usize;
    if len == 0 && msg_flag == 1 {
        // End Event
        println!("ALL: Play End");
        return;
    }
    let mut buf = vec![0; len];
    port.read_exact(&mut buf).await.unwrap();
    // println!("{:#02x} {:#02x?}",first_byte, buf);
    if msg_flag != 1 {
        println!("receive: {:#02x}", msg_flag);
    }
    let event_flag = SequenceEventFlag::from(buf[0] & 0xf);
    let ch = (buf[0] >> 4) & 0xf;
    match event_flag {
        SequenceEventFlag::KeyEvent => {
            let key = buf[1];
            let vel = buf[2];
            if vel == 0 {
                println!("Ch{ch}: Key Off {}", key);
            } else {
                println!("Ch{ch}: Key On  {}", key);
            }
        }
        SequenceEventFlag::Tempo => {
            let tempo = ((buf[1] as u32) << 16) | ((buf[2] as u32) << 8) | (buf[3] as u32);
            println!("Ch{ch}: TEMPO   {}", tempo);
        }
        SequenceEventFlag::End => {
            println!("ALL: Play End");
        }
        SequenceEventFlag::Nop => {}
        SequenceEventFlag::Param => msg_param_change(ch, &buf[1..], app_handle),
        SequenceEventFlag::ProgramChange => {
            let name = cvt_string(&buf[1..]);
            println!("Ch{ch}: PC      {}", name);
        }
        SequenceEventFlag::Expression => {}
        _ => {}
    }
}
// TODO: フロントへのイベント発行の実装
fn msg_param_change<R: tauri::Runtime>(ch: u8, buf: &[u8], app_handle: &impl tauri::Manager<R>) {
    let param_flag = ParamChangeFlag::from(buf[0] & 0xf);
    match param_flag {
        ParamChangeFlag::Slot => {}
        ParamChangeFlag::DtMul => {}
        ParamChangeFlag::Tl => {}
        ParamChangeFlag::KsAr => {}
        ParamChangeFlag::Dr => {}
        ParamChangeFlag::Sr => {}
        ParamChangeFlag::SlRr => {}
        ParamChangeFlag::FbCon => {}
        ParamChangeFlag::Other => {
            println!("Ch{ch}: unknown param");
        }
    }
}
fn cvt_string(buf: &[u8]) -> String {
    let tmp = Vec::from(buf);
    String::from_utf8(tmp).unwrap_or(String::from("unknown"))
}
