pub const LENGTH_TIMER_MAX: u16 = 64;

pub struct LengthTimer {
    pub enabled: bool,
    pub timer: u16,
}

impl LengthTimer {
    pub fn new() -> Self {
        Self { enabled: false, timer: 0 }
    }

    pub fn cycle(&mut self, channel_enabled: &mut bool) {
        if !self.enabled || self.timer == 0 {
            return;
        }

        self.timer = self.timer.saturating_sub(1);
        if self.timer != 0 {
            return;
        }
        *channel_enabled = false;
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.timer = 0;
    }
}
