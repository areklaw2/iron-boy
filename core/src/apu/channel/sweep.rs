pub struct Sweep {
    step: u8,
    direction: bool,
    pace: u8,
    pub sequence: u8,
}

impl Sweep {
    pub fn new() -> Self {
        Self {
            step: 0,
            direction: true,
            pace: 0,
            sequence: 0,
        }
    }

    pub fn cycle(&mut self, frequency: &mut u16, channel_enabled: &mut bool) {
        if self.pace == 0 {
            return;
        }

        self.sequence += 1;
        if self.sequence >= self.pace {
            let delta = *frequency >> self.step;

            *frequency = match self.direction {
                true => frequency.saturating_add(delta),
                false => frequency.saturating_sub(delta),
            };

            if *frequency > 0x07FF {
                *channel_enabled = false;
                *frequency = 0x07FF;
            }
            self.sequence = 0;
        }
    }

    pub fn write(&mut self, data: u8) {
        self.step = data & 0x07;
        self.direction = (data & 0x08) == 0x00;
        self.pace = (data & 0x70) >> 4;
    }

    pub fn read(&self) -> u8 {
        let pace = (self.pace & 0x07) << 4;
        let direction = (self.direction as u8) << 3;
        let shift = self.step & 0x07;
        0x80 | pace | direction | shift
    }

    pub fn reset(&mut self) {
        self.step = 0;
        self.direction = true;
        self.pace = 0;
        self.sequence = 0;
    }
}
