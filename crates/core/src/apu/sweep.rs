use Direction::*;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Increasing,
    Decreasing,
}

impl From<bool> for Direction {
    fn from(value: bool) -> Self {
        match value {
            false => Increasing,
            true => Decreasing,
        }
    }
}

pub struct Sweep {
    pace: u8,
    direction: Direction,
    step: u8,
    timer: u8,
    enabled: bool,
    shadow_period: u16,
    period_calculated: bool,
}

impl Sweep {
    pub fn new() -> Self {
        Self {
            pace: 0,
            direction: Increasing,
            step: 0,
            timer: 0,
            enabled: false,
            shadow_period: 0,
            period_calculated: false,
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

            if self.enabled && self.pace > 0 {
                let new_period = self.calculate_period(channel_enabled);
                if new_period > 0x07FF || self.step == 0 {
                    return;
                }
                self.shadow_period = new_period;
                *period = new_period;
                self.calculate_period(channel_enabled);
            } else {
                self.period_calculated = false;
            }
        }
    }

    pub fn write(&mut self, value: u8, channel_enabled: &mut bool) {
        self.pace = (value & 0x70) >> 4;

        let new_direction = ((value & 0x08) != 0).into();
        if self.direction == Decreasing && new_direction == Increasing && self.period_calculated {
            *channel_enabled = false;
        }
        self.direction = new_direction;

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
        self.direction = Increasing;
        self.step = 0;
        self.timer = 0;
        self.enabled = false;
        self.shadow_period = 0;
        self.period_calculated = false;
    }

    fn calculate_period(&mut self, channel_enabled: &mut bool) -> u16 {
        let delta = self.shadow_period >> self.step;
        let new_period = match self.direction {
            Increasing => self.shadow_period.saturating_add(delta),
            Decreasing => self.shadow_period.saturating_sub(delta),
        };

        match new_period > 0x07FF {
            true => *channel_enabled = false,
            false => self.period_calculated = true,
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
        } else {
            self.period_calculated = false;
        }
    }
}
