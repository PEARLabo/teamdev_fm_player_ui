use crate::cli::keyboard::draw_keyboard;
use crate::cli::structs::{PlayingLog, TrackInfo};
use crossterm::{
    QueueableCommand,
    cursor::{MoveTo, MoveToColumn},
    style::{self, Color},
    terminal::{Clear, ClearType},
};
use micromap::Set;
use std::io::stdout;

pub const TABLE_TOP: u16 = 4;
// YM2203の場合
pub const MAX_CHANNEL: u8 = 6;

pub static CH_COLOR: &[style::Color; MAX_CHANNEL as usize] = &[
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Cyan,
];

pub fn draw_display(
    title: impl AsRef<str>,
    tempo: u32,
    track_info: &[TrackInfo],
    state: &[Set<u8, 8>],
    logs: &PlayingLog,
) -> std::io::Result<()> {
    let mut stdout = stdout();
    stdout
        .queue(Clear(ClearType::All))?
        .queue(MoveTo(0, 0))?
        .queue(style::Print(format!("Title: {}", title.as_ref())))?
        .queue(MoveTo(0, 1))?
        .queue(style::Print(format!("TEMPO: {}", tempo)))?
        .queue(MoveTo(0, 2))?
        .queue(style::Print(
            "  Ch  [Inst   ]  Key State   PitchBend  Expression",
        ))?
        .queue(MoveTo(0, 3))?
        .queue(style::Print(
            "  ------------------------------------------------",
        ))?
        .queue(MoveTo(0, MAX_CHANNEL as u16 + TABLE_TOP + 3))?
        .queue(style::Print("Received Messages:"))?;
    for ch in 0..track_info.len() {
        draw_table_at(track_info, ch)?;
    }
    draw_logs(logs)?;
    draw_keyboard(state)?;
    Ok(())
}

pub fn draw_tempo(tempo: u32) -> std::io::Result<()> {
    let mut stdout = stdout();
    stdout
        .queue(MoveTo(0, 1))?
        .queue(Clear(ClearType::CurrentLine))?
        .queue(style::Print(format!("TEMPO: {}", tempo)))?;
    Ok(())
}

pub fn draw_table_at(track_info: &[TrackInfo], ch: usize) -> std::io::Result<()> {
    let info = &track_info[ch];
    let mut stdout = stdout();
    stdout
        .queue(MoveTo(0, TABLE_TOP + ch as u16))?
        .queue(Clear(ClearType::CurrentLine))?
        .queue(style::SetForegroundColor(CH_COLOR[ch]))?
        .queue(style::Print(format!("  Ch{} [{}]", ch, info.get_inst())))?
        .queue(style::ResetColor)?
        .queue(MoveToColumn(17))?
        .queue(style::Print(info.get_key_state()))?
        .queue(MoveToColumn(29))?
        .queue(style::Print(info.get_pitch_bend()))?
        .queue(MoveToColumn(40))?
        .queue(style::Print(info.get_expression()))?;
    Ok(())
}

pub fn draw_logs(logs: &PlayingLog) -> std::io::Result<()> {
    let mut stdout = stdout();
    let top = TABLE_TOP + MAX_CHANNEL as u16 + 4;
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
