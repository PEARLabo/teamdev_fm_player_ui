#[derive(Debug)]
pub struct U24(u32);

//24bit整数を扱うための
impl U24 {
    fn from_be_bytes(high: u8, mid: u8, low: u8) -> Self {
        Self(((high as u32) << 16) | ((mid as u32) << 8) | (low as u32))
    }

    fn value(&self) -> u32 {
        self.0
    }
}

//MISI形式のファイルか判定する関数
pub fn check_midi_format(contents: &[u8]) -> bool {
    contents.starts_with(b"MThd")
}
