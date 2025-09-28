pub struct LengthTimer {
    enabled: bool,
    time: u16,
}

impl LengthTimer {
    pub fn new() -> Self {
        Self { enabled: false, time: 0 }
    }

    pub fn cycle(&mut self, channel_enabled: &mut bool) {
        if !self.enabled || self.time == 0 {
            return;
        }

        self.time = self.time.saturating_sub(1);
        if self.time == 0 {
            *channel_enabled = false
        }
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.time = 0;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, value: bool, channel_enabled: &mut bool) {
        let old_enabled = self.enabled;
        self.enabled = value;
        if !old_enabled && self.enabled {
            self.cycle(channel_enabled);
        }
    }

    pub fn time(&self) -> u16 {
        self.time
    }

    pub fn set_time(&mut self, value: u16) {
        self.time = value
    }
}
