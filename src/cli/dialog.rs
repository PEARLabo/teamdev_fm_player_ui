use crossterm::{
    QueueableCommand,
    cursor::{MoveTo, MoveToColumn},
    style,
    terminal::{Clear, ClearType},
};

pub fn file_dialog(msg: Option<String>) -> std::io::Result<()> {
    let mut stdout = std::io::stdout();
    stdout
        .queue(Clear(crossterm::terminal::ClearType::All))?
        .queue(MoveTo(0, 2))?
        .queue(style::Print("============= Send File ============="))?
        .queue(MoveTo(0, 3))?
        .queue(style::Print(msg.unwrap_or_default()))?
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
// TODO:　補間機能は後で実装
pub fn interpolation_path(input: impl AsRef<str>) -> String {
    let input = input.as_ref();
    return input.to_string();
}
