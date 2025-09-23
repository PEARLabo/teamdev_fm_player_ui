use std::io::{Write, stdout};

use crossterm::{
    QueueableCommand,
    cursor::MoveTo,
    style::{self, Color, Stylize},
};
use micromap::Set;

use crate::cli::view::CH_COLOR;

pub fn draw_keyboard(state: &[Set<u8, 8>]) -> std::io::Result<()> {
    const KEYBOARD_TOP: u16 = crate::cli::view::TABLE_TOP + crate::cli::MAX_CHANNEL as u16 + 1;
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
    stdout.queue(MoveTo(0, KEYBOARD_TOP))?;
    // 描画設定
    upper_key.into_iter().for_each(|s| {
        let _ = stdout.queue(s);
    });
    stdout.queue(MoveTo(0, KEYBOARD_TOP + 1))?;
    lower_key.into_iter().for_each(|s| {
        let _ = stdout.queue(s);
    });
    Ok(())
}

#[inline]
const fn is_natural_note(n: u8) -> bool {
    let n = n % 12;
    n == 0 || n == 2 || n == 4 || n == 5 || n == 7 || n == 9 || n == 11
}
