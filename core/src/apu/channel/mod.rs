pub mod length_timer;
pub mod sweep;
pub mod volume_envelope;

pub trait Channel {
    fn cycle(&mut self, ticks: u32);
    fn trigger(&mut self);
    fn reset(&mut self);
}

pub struct ChannelBase {
    pub enabled: bool,
    pub dac_enabled: bool,
    pub output: u8,
    pub timer: i16,
    pub triggered: bool,
}

impl ChannelBase {
    pub fn new() -> Self {
        Self {
            enabled: false,
            dac_enabled: false,
            output: 0,
            timer: 0,
            triggered: false,
        }
    }

    pub fn get_output(&self) -> u8 {
        if self.enabled && self.dac_enabled {
            self.output
        } else {
            0
        }
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.dac_enabled = false;
        self.output = 0;
        self.timer = 0;
        self.triggered = false;
    }
}
