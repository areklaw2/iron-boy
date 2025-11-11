use crate::apu::period::Period;
use getset::CopyGetters;

#[derive(Debug, CopyGetters)]
pub struct Sweep {
    pace: u8,
    direction: bool,
    step: u8,
    enabled: bool,
    shadow_period: u16,
    timer: u8,
    period_calculated: bool,
    #[getset(get_copy = "pub")]
    disable_channel: bool,
}

impl Sweep {
    pub fn new() -> Self {
        Sweep {
            pace: 0,
            direction: false,
            step: 0,
            enabled: false,
            shadow_period: 0,
            timer: 0,
            period_calculated: false,
            disable_channel: false,
        }
    }

    pub fn cycle(&mut self, period: &mut Period) {
        if self.timer > 0 {
            self.timer -= 1;
        }

        if self.timer == 0 {
            match self.pace > 0 {
                true => self.timer = self.pace,
                false => self.timer = 8,
            }

            if self.enabled && self.pace > 0 {
                let new_period = self.calculate_period();

                if new_period <= 2047 && self.step > 0 {
                    self.shadow_period = new_period;
                    let low = (new_period & 0xFF) as u8;
                    let high = ((new_period & 0x700) >> 8) as u8;
                    period.set_low(low);
                    period.set_high((period.high() & 0xF8) | high);
                    self.calculate_period();
                }
            } else {
                self.period_calculated = false;
            }
        }
    }

    pub fn calculate_period(&mut self) -> u16 {
        let sweep_shift = self.step;
        let mut new_period = self.shadow_period >> sweep_shift;

        new_period = match self.direction {
            true => self.shadow_period - new_period,
            false => self.shadow_period + new_period,
        };

        match new_period > 2047 {
            true => self.disable_channel = true,
            false => self.period_calculated = true,
        }
        new_period
    }

    pub fn read(&self) -> u8 {
        let pace = (self.pace & 0x07) << 4;
        let direction = (self.direction as u8) << 3;
        let shift = self.step & 0x07;
        pace | direction | shift
    }

    pub fn write(&mut self, value: u8) {
        let sweep_value = self.read();
        self.pace = (value & 0x70) >> 4;
        self.direction = (value & 0x08) != 0;
        self.step = value & 0x07;

        let was_decrementing = (sweep_value & (1 << 3)) != 0;
        let is_decrementing = self.direction;
        let exiting_negate_mode = was_decrementing && !is_decrementing;
        if exiting_negate_mode && self.period_calculated {
            self.disable_channel = true;
        }
    }

    pub fn trigger(&mut self, period_value: u16) {
        self.shadow_period = period_value;

        match self.pace > 0 {
            true => self.timer = self.pace,
            false => self.timer = 8,
        }

        self.enabled = self.pace > 0 || self.step > 0;
        self.disable_channel = false;

        if self.step > 0 {
            self.calculate_period();
        } else {
            self.period_calculated = false;
        }
    }
}
