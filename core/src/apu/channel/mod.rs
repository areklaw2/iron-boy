pub mod length_timer;
pub mod sweep;
pub mod volume_envelope;

pub trait Channel {
    fn cycle(&mut self, ticks: u32);
    fn trigger(&mut self);
    fn reset(&mut self);
    fn enabled(&self) -> bool;
    fn output(&self) -> u8;
    fn length_timer_cycle(&mut self);
    fn volume_envelope_cycle(&mut self);
}

pub struct ChannelBase {
    pub output: u8,
    pub enabled: bool,
    pub dac_enabled: bool,
    pub triggered: bool,
    pub timer: i16,
}

impl ChannelBase {
    pub fn new() -> Self {
        Self {
            output: 0,
            enabled: false,
            dac_enabled: false,
            triggered: false,
            timer: 0,
        }
    }

    pub fn output(&self) -> u8 {
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
