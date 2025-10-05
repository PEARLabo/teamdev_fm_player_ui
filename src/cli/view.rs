use crate::cli::keyboard::draw_keyboard;
use crate::cli::structs::{PlayingLog, TrackInfo};
use crate::cli::{AppState, MAX_CHANNEL, keyboard};
use crossterm::{
    QueueableCommand,
    cursor::{MoveTo, MoveToColumn},
    style::{self, Color},
    terminal::{Clear, ClearType},
};
use std::io::stdout;
use std::sync::atomic::Ordering;

pub const TABLE_TOP: u16 = 4;
// YM2203の場合
// pub const MAX_CHANNEL: u8 = 6;
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Rhythm {
    BD,
    SD,
    Top,
    HH,
    Tom,
    Rym,
}
impl Rhythm {
    pub fn cvt_name_by_name(code: usize) -> String {
        match code {
            0 => Self::BD,
            1 => Self::SD,
            2 => Self::Top,
            3 => Self::HH,
            4 => Self::Tom,
            5 => Self::Rym,
            _ => panic!("Invalid Rhythm Code"),
        }
        .to_string()
    }
}
impl TryFrom<u8> for Rhythm {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            40 => Ok(Self::Rym),
            47 => Ok(Self::Tom),
            42 => Ok(Self::HH),
            46 => Ok(Self::Top),
            38 => Ok(Self::SD),
            36 => Ok(Self::BD),
            _ => Err("Invalid Rhythm Code"),
        }
    }
}
impl std::fmt::Display for Rhythm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BD => write!(f, "BD"),
            Self::SD => write!(f, "SD"),
            Self::Top => write!(f, "TOP"),
            Self::HH => write!(f, "HH"),
            Self::Tom => write!(f, "TOM"),
            Self::Rym => write!(f, "RYM"),
        }
    }
}
pub static CH_COLOR: &[style::Color; super::MAX_CHANNELS_YM2608 as usize] = &[
    // 3bit colorに含まれる色（ANSI 0〜15）
    Color::AnsiValue(9),  // 赤
    Color::AnsiValue(10), // 黄
    Color::AnsiValue(11), // 緑
    Color::AnsiValue(12), // 青緑（シアン）
    Color::AnsiValue(13), // 青
    Color::AnsiValue(14), // マゼンタ
    Color::AnsiValue(1),
    Color::AnsiValue(2),
    Color::AnsiValue(3),
    Color::AnsiValue(4),
];

pub fn draw_display(
    app_state: &AppState,
    // tempo: u32,
    // track_info: &[TrackInfo],
    // state: &[Set<u8, 8>],
    // logs: &PlayingLog,
) -> std::io::Result<()> {
    let mut stdout = stdout();
    stdout
        .queue(Clear(ClearType::All))?
        .queue(MoveTo(0, 0))?
        .queue(style::Print(format!("Title: {}", &app_state.title)))?
        .queue(MoveTo(0, 1))?
        .queue(style::Print(format!("TEMPO: {}", app_state.tempo)))?
        .queue(MoveTo(0, 2))?
        .queue(style::Print(if app_state.ym2608 {
            "       Ch   [Inst   ]  Key State   PitchBend  Expression  PanPot"
        } else {
            "       Ch   [Inst   ]  Key State   PitchBend  Expression"
        }))?
        .queue(MoveTo(0, 3))?
        .queue(style::Print(
            "  -------------------------------------------------------",
        ))?
        .queue(MoveTo(
            0,
            MAX_CHANNEL.load(Ordering::Relaxed) as u16 + TABLE_TOP + 6,
        ))?
        .queue(style::Print("Received Messages:"))?;
    for ch in 0..app_state.track_info.len() {
        draw_table_at(&app_state.track_info, ch, app_state.ym2608)?;
    }
    if app_state.ym2608 {
        keyboard::draw_rhythm(&app_state.percussion_state)?;
    }
    draw_logs(&app_state.logs)?;
    draw_keyboard(&app_state.keyboard_state)?;
    Ok(())
}

pub fn draw_tempo(tempo: u32) -> std::io::Result<()> {
    let mut stdout = stdout();
    stdout
        .queue(MoveTo(0, 1))?
        .queue(Clear(ClearType::CurrentLine))?
        .queue(style::Print(format!("TEMPO: {tempo}")))?;
    Ok(())
}

pub fn draw_table_at(track_info: &[TrackInfo], ch: usize, is_ym2608: bool) -> std::io::Result<()> {
    let info = &track_info[ch];
    let mut stdout = stdout();
    stdout
        .queue(MoveTo(0, TABLE_TOP + ch as u16))?
        .queue(Clear(ClearType::CurrentLine))?
        .queue(style::SetForegroundColor(CH_COLOR[ch]))?
        .queue(style::Print(format!(
            "  {}  Ch{:2} [{}]",
            match ch {
                9 => "RHY",
                x if x < 3 || x >= 6 => "FM ",
                x if x < 6 => "SSG",
                _ => "   ",
            },
            ch + 1,
            info.get_inst()
        )))?
        .queue(style::ResetColor)?
        .queue(MoveToColumn(23))?
        .queue(style::Print(info.get_key_state()))?
        .queue(MoveToColumn(35))?
        .queue(style::Print(info.get_pitch_bend()))?
        .queue(MoveToColumn(46))?
        .queue(style::Print(info.get_expression()))?;
    if is_ym2608 {
        stdout.queue(MoveToColumn(58))?.queue(style::Print(format!(
            "{} ({:2})",
            info.get_pan_pot_str(),
            info.get_pan_pot()
        )))?;
    }
    Ok(())
}

pub fn draw_logs(logs: &PlayingLog) -> std::io::Result<()> {
    let mut stdout = stdout();
    let top = TABLE_TOP + MAX_CHANNEL.load(Ordering::Relaxed) as u16 + 7;
    stdout
        .queue(MoveTo(0, top))?
        .queue(Clear(ClearType::FromCursorDown))?;
    for (i, log) in logs.get_logs().iter().enumerate() {
        stdout
            .queue(MoveTo(0, top + i as u16))?
            .queue(style::Print(format!(
                "{:8} - {}",
                log.get_id(),
                log.get_msg()
            )))?;
    }
    Ok(())
}
