use getset::{CopyGetters, Setters};

#[derive(Debug, CopyGetters, Setters)]
#[getset(get_copy = "pub", set = "pub")]
pub struct Length {
    initial_time: u8,
    time: u16,
    #[getset(skip)]
    max_length: u16,
}

pub const WAVE_MAX_LENGTH: u16 = 256;
pub const DEFAULT_MAX_LENGTH: u16 = 64;

impl Length {
    pub fn new(max_length: u16) -> Self {
        Length {
            initial_time: 0,
            time: 0,
            max_length,
        }
    }

    pub fn reset(&mut self) {
        self.initial_time = 0
    }

    pub fn cycle(&mut self) {
        if self.time > 0 {
            self.time -= 1;
        }
    }

    pub fn initialize(&mut self) {
        let initial_length: u16 = match self.max_length == WAVE_MAX_LENGTH {
            true => self.initial_time as u16,
            false => (self.initial_time & 0x3F) as u16,
        };
        self.time = self.max_length - initial_length;
    }

    pub fn reload(&mut self) {
        if self.time == 0 {
            self.time = self.max_length;
        }
    }

    pub fn expired(&self) -> bool {
        self.time == 0
    }

    pub fn maxxed(&self) -> bool {
        self.time == self.max_length
    }
}
