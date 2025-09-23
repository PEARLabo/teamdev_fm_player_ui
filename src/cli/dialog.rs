use crossterm::{
    QueueableCommand,
    cursor::{MoveTo, MoveToColumn},
    style,
    terminal::{Clear, ClearType},
};

use crate::utils::DirItem;

pub fn file_dialog(msg: Option<impl AsRef<str>>) -> std::io::Result<()> {
    let mut stdout = std::io::stdout();
    let e_message = if let Some(msg) = msg {
        msg.as_ref().to_string()
    } else {
        String::new()
    };
    stdout
        .queue(Clear(crossterm::terminal::ClearType::All))?
        .queue(MoveTo(0, 2))?
        .queue(style::Print("============= Send File ============="))?
        .queue(MoveTo(0, 3))?
        .queue(style::Print(e_message))?
        .queue(MoveTo(0, 5))?
        .queue(style::Print("======== PRESS ENTER TO SEND ========"))?
        .queue(MoveTo(0, 4))?
        .queue(style::Print("file name > "))
        .map(|_| ())
}
pub fn update_file_path(path: impl AsRef<str>, cursor_pos: usize) -> std::io::Result<()> {
    let mut stdout = std::io::stdout();
    stdout
        .queue(MoveTo(0, 4))?
        .queue(Clear(ClearType::CurrentLine))?
        .queue(style::Print(format!("file name > {}", path.as_ref())))?
        .queue(MoveToColumn(cursor_pos as u16 + 12))?;

    Ok(())
}
pub fn draw_suggest(entries: &[DirItem]) -> std::io::Result<()> {
    const MAX_LENGTH: usize = 80;
    let mut stdout = std::io::stdout();
    let mut lines = Vec::new();
    let mut tmp = String::new();

    for entry in entries {
        if tmp.len() > MAX_LENGTH {
            lines.push(tmp);
            tmp = String::new();
        }
        tmp += entry.get_file_name().unwrap();
    }
    if !tmp.is_empty() {
        lines.push(tmp);
    }
    let mut n = 0;
    stdout
        .queue(MoveTo(0, 5))?
        .queue(Clear(ClearType::FromCursorDown))?;
    lines.into_iter().for_each(|str| {
        unsafe {
            let _ = stdout
                .queue(MoveTo(0, n + 5))
                .unwrap_unchecked()
                .queue(style::Print(str));
        }
        n += 1;
    });
    stdout
        .queue(MoveTo(0, n + 5))?
        .queue(style::Print("======== PRESS ENTER TO SEND ========"))?;
    Ok(())
}
