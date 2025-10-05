use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    style::{self, Color, Stylize},
    terminal::{Clear, ClearType},
};
use micromap::Set;
use std::io::{self, stdout};

use crate::cli::{
    MAX_CHANNEL,
    view::{CH_COLOR, Rhythm},
};
pub fn draw_keyboard(state: &[Set<u8, 8>]) -> std::io::Result<()> {
    // MAX_CHANNEL is a const in view.rs, so we can use it directly
    let keyboard_top: u16 = crate::cli::view::TABLE_TOP
        + MAX_CHANNEL.load(std::sync::atomic::Ordering::Relaxed) as u16
        + 1;
    let mut stdout = stdout();
    let mut prev_state = Color::White;
    // let
    let [upper_key, lower_key]: [Vec<style::PrintStyledContent<_>>; 2] = state
        .iter()
        .enumerate()
        .fold([Vec::new(), Vec::new()], |mut acc, (n, s)| {
            let is_natural_tone = is_natural_note(n as u8);
            let upper_color = if let Some(&ch) = s.iter().next() {
                CH_COLOR[ch as usize]
            } else if is_natural_tone {
                Color::White
            } else {
                Color::Black
            };
            if is_natural_tone {
                // 黒鍵の下の対応
                prev_state = upper_color;
            }
            acc[0].push(style::PrintStyledContent(" ".on(upper_color)));
            acc[1].push(style::PrintStyledContent(" ".on(prev_state)));
            acc
        });
    stdout.queue(MoveTo(0, keyboard_top))?;
    // 描画設定
    upper_key.into_iter().for_each(|s| {
        let _ = stdout.queue(s);
    });
    stdout.queue(MoveTo(0, keyboard_top + 1))?;
    lower_key.into_iter().for_each(|s| {
        let _ = stdout.queue(s);
    });
    Ok(())
}

pub fn draw_rhythm(state: &[u8; 6]) -> io::Result<()> {
    // キーボードの1行下
    let row = crate::cli::view::TABLE_TOP
        + MAX_CHANNEL.load(std::sync::atomic::Ordering::Relaxed) as u16
        + 1
        + 3;
    let mut stdout = stdout();

    stdout
        .queue(MoveTo(2, row))?
        .queue(Clear(ClearType::CurrentLine))?
        .queue(style::Print("Rhythm:"))?;
    state.iter().enumerate().for_each(|(i, s)| {
        let _ = stdout.queue(MoveTo(i as u16 * 5 + 11, row));
        if *s != 0 {
            let _ = stdout.queue(style::PrintStyledContent(
                Rhythm::cvt_name_by_name(i).with(CH_COLOR[i]),
            ));
        } else {
            let _ = stdout.queue(style::PrintStyledContent(
                Rhythm::cvt_name_by_name(i).with(Color::Grey),
            ));
        };
    });
    Ok(())
}
#[inline]
const fn is_natural_note(n: u8) -> bool {
    let n = n % 12;
    n == 0 || n == 2 || n == 4 || n == 5 || n == 7 || n == 9 || n == 11
}
