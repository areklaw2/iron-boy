pub mod length_timer;
pub mod sweep;
pub mod volume_envelope;

pub trait Channel {
    fn cycle(&mut self, cycles: u32);
    fn length_timer_cycle(&mut self);
    fn volume_envelope_cycle(&mut self);
    fn trigger(&mut self);
    fn reset(&mut self);
    fn enabled(&self) -> bool;
    fn sample(&self) -> u8;
}

pub struct ChannelBase {
    pub sample: u8,
    pub enabled: bool,
    pub dac_enabled: bool,
    pub triggered: bool,
    pub timer: i32,
}

impl ChannelBase {
    pub fn new() -> Self {
        Self {
            sample: 0,
            enabled: false,
            dac_enabled: false,
            triggered: false,
            timer: 0,
        }
    }

    pub fn sample(&self) -> u8 {
        if self.enabled && self.dac_enabled { self.sample } else { 0 }
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.dac_enabled = false;
        self.sample = 0;
        self.timer = 0;
        self.triggered = false;
    }
}
