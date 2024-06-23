pub struct VolumeEnvelope {
    pub enabled: bool,
    pub counter: u8,
    pub pace: u8,
    pub direction: bool,
    pub volume: u8,
}

impl VolumeEnvelope {
    pub fn new() -> Self {
        Self {
            enabled: false,
            counter: 0,
            pace: 0,
            direction: true,
            volume: 0,
        }
    }

    pub fn cycle(&mut self, channel_enabled: &bool) {
        if !self.enabled || !channel_enabled {
            return;
        }

        self.counter += 1;
        if self.counter < self.pace {
            return;
        }

        self.volume = match self.direction {
            true => self.volume.saturating_add(1),
            false => self.volume.saturating_sub(1),
        };

        if self.volume == 0 || self.volume == 15 {
            self.enabled = false;
        }

        self.counter = 0;
    }

    pub fn write(&mut self, data: u8) {
        self.pace = data & 0x07;
        self.direction = data & 0x08 == 0x08;
        self.volume = (data & 0xF0) >> 4;
        self.enabled = self.pace > 0;
        self.counter = 0;
    }

    pub fn read(&self) -> u8 {
        let volume = (self.volume & 0x0F) << 4;
        let direction = (self.direction as u8) << 3;
        let pace = self.pace & 0x07;
        volume | direction | pace
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.counter = 0;
        self.pace = 0;
        self.direction = true;
        self.volume = 0;
    }
}
