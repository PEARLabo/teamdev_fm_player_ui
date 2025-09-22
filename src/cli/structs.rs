use std::collections::VecDeque;

#[derive(Debug, Default, Clone)]
pub struct TrackInfo {
    inst: String,
    key_state: Option<u8>,
    pitch_bend: i32,
    expression: u8,
}

impl TrackInfo {
    #[inline]
    pub const fn get_inst(&self) -> &str {
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
    pub const fn get_pitch_bend(&self) -> i32 {
        self.pitch_bend
    }
    #[inline]
    pub const fn get_expression(&self) -> u8 {
        self.expression
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
