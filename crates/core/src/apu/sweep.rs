pub struct Sweep {
    pace: u8,
    direction: bool,
    step: u8,
    timer: u8,
}

impl Sweep {
    pub fn new() -> Self {
        Self {
            pace: 0,
            direction: false,
            step: 0,
            timer: 0,
        }
    }

    pub fn cycle(&mut self, frequency: &mut u16, channel_enabled: &mut bool) {
        if self.pace == 0 {
            return;
        }

        self.timer += 1;
        if self.timer >= self.pace {
            let delta = *frequency >> self.step;

            *frequency = match self.direction {
                false => frequency.saturating_add(delta),
                true => frequency.saturating_sub(delta),
            };

            if *frequency > 0x07FF {
                *channel_enabled = false;
                *frequency = 0x07FF;
            }
            self.timer = 0;
        }
    }

    pub fn reset_timer(&mut self) {
        self.timer = 0
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
    }
}
