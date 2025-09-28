pub struct Sweep {
    pace: u8,
    direction: bool,
    step: u8,
    timer: u8,
    enabled: bool,
    shadow_period: u16,
}

impl Sweep {
    pub fn new() -> Self {
        Self {
            pace: 0,
            direction: false,
            step: 0,
            timer: 0,
            enabled: false,
            shadow_period: 0,
        }
    }

    pub fn cycle(&mut self, period: &mut u16, channel_enabled: &mut bool) {
        if self.timer > 0 {
            self.timer -= 1;
        }

        if self.timer == 0 {
            match self.pace > 0 {
                true => self.timer = self.pace,
                false => self.timer = 8,
            }

            if !self.enabled || self.pace == 0 {
                return;
            }

            let new_period = self.calculate_period(channel_enabled);
            if new_period <= 0x07FF && self.step > 0 {
                self.shadow_period = new_period;
                *period = new_period;
                self.calculate_period(channel_enabled);
            }
        }
    }

    pub fn write(&mut self, value: u8) {
        self.pace = (value & 0x70) >> 4;
        self.direction = (value & 0x08) != 0;
        self.step = value & 0x07;
    }

    pub fn read(&self) -> u8 {
        let pace = (self.pace & 0x07) << 4;
        let direction = (self.direction as u8) << 3;
        let shift = self.step & 0x07;
        0x80 | pace | direction | shift
    }

    pub fn reset(&mut self) {
        self.pace = 0;
        self.direction = false;
        self.step = 0;
        self.timer = 0;
        self.enabled = false;
        self.shadow_period = 0;
    }

    fn calculate_period(&self, channel_enabled: &mut bool) -> u16 {
        let delta = self.shadow_period >> self.step;
        let new_period = match self.direction {
            false => self.shadow_period.saturating_add(delta),
            true => self.shadow_period.saturating_sub(delta),
        };

        if new_period > 0x07FF {
            *channel_enabled = false;
        }
        new_period
    }

    pub fn trigger(&mut self, period: u16, channel_enabled: &mut bool) {
        self.shadow_period = period;
        match self.pace > 0 {
            true => self.timer = self.pace,
            false => self.timer = 8,
        }

        self.enabled = self.pace > 0 || self.step > 0;

        if self.step > 0 {
            self.calculate_period(channel_enabled);
        }
    }
}
