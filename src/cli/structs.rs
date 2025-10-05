use std::collections::VecDeque;
const PANPOT_CENTER: u8 = 64;
const PITCHBEND_CENTER: i32 = 0;
#[derive(Debug, Clone)]
pub struct TrackInfo {
    inst: String,
    key_state: Option<u8>,
    pitch_bend: i32,
    expression: u8,
    pan_pot: u8,
}
impl Default for TrackInfo {
    fn default() -> Self {
        Self {
            inst: String::new(),
            key_state: None,
            pitch_bend: PITCHBEND_CENTER,
            expression: 127,
            pan_pot: PANPOT_CENTER,
        }
    }
}

impl TrackInfo {
    #[inline]
    pub fn get_inst(&self) -> &str {
        self.inst.as_str()
    }
    pub fn get_key_state(&self) -> String {
        if let Some(n) = self.key_state {
            format!("Key On  {n:<3}")
        } else {
            String::from("Key Off    ")
        }
    }
    #[inline]
    pub fn get_pitch_bend(&self) -> i32 {
        self.pitch_bend
    }
    #[inline]
    pub fn get_expression(&self) -> u8 {
        self.expression
    }
    #[inline]
    pub fn get_pan_pot(&self) -> u8 {
        self.pan_pot
    }
    pub fn get_pan_pot_str(&self) -> &'static str {
        let l = self.pan_pot < 86;
        let r = self.pan_pot > 43;
        if l && r {
            "C"
        } else if l {
            "L"
        } else if r {
            "R"
        } else {
            " "
        }
    }
    #[inline]
    pub fn set_inst(&mut self, inst: String) {
        self.inst = inst;
    }
    #[inline]
    pub(crate) fn set_key_state(&mut self, data: Option<u8>) {
        self.key_state = data;
    }

    pub fn set_expression(&mut self, expression: u8) {
        self.expression = expression;
    }
    pub fn set_pitch_bend(&mut self, bend: i32) {
        self.pitch_bend = bend;
    }
    #[inline]
    pub fn set_pan_pot(&mut self, pan_pot: u8) {
        self.pan_pot = pan_pot;
    }

    pub(crate) fn clear(&mut self) {
        self.inst.clear();
        self.key_state = None;
        self.pitch_bend = PITCHBEND_CENTER;
        self.expression = 127;
        self.pan_pot = PANPOT_CENTER;
    }
}
#[derive(Debug, Default)]
pub struct PlayingLog {
    current_id: u64,
    logs: VecDeque<LogItem>,
}
impl PlayingLog {
    pub const MAX_LOG_SIZE: usize = 10;
    pub fn add(&mut self, msg: String) {
        while self.logs.len() >= Self::MAX_LOG_SIZE {
            self.logs.pop_back();
        }
        self.logs.push_front(LogItem::new(self.current_id, msg));
        dbg!(self.logs.len());
        self.current_id += 1;
    }
    pub fn get_logs(&self) -> &VecDeque<LogItem> {
        &self.logs
    }
}
#[derive(Debug)]
pub struct LogItem {
    id: u64,
    msg: String,
}
impl LogItem {
    #[inline]
    pub fn new(id: u64, msg: String) -> Self {
        Self { id, msg }
    }
    #[inline]
    pub const fn get_id(&self) -> u64 {
        self.id
    }
    #[inline]
    pub const fn get_msg(&self) -> &str {
        self.msg.as_str()
    }
}
