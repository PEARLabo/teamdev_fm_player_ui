#[derive(Debug, Clone)]
pub enum KeyCommand {
    // o + enter
    DialogOpen,
    // esc
    GoMonitor,
    // q + enter
    Quit,
    // tab
    InterPolation,
    // some input + enter
    Other(String),
}

impl From<&str> for KeyCommand {
    fn from(value: &str) -> Self {
        if value.is_empty() {
            return Self::Other(String::new());
        }
        if value.len() == 1 {
            match value.chars().next().unwrap() {
                'o' => Self::DialogOpen,
                'q' => Self::Quit,
                _ => Self::Other(value.to_string()),
            }
        } else {
            Self::Other(value.to_string())
        }
    }
}
