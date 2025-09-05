pub struct VolumeEnvelope {
    enabled: bool,
    timer: u8,
    pace: u8,
    direction: bool,
    volume: u8,
}

impl VolumeEnvelope {
    pub fn new() -> Self {
        Self {
            enabled: false,
            timer: 0,
            pace: 0,
            direction: true,
            volume: 0,
        }
    }

    pub fn cycle(&mut self, channel_enabled: bool) {
        if !self.enabled || !channel_enabled {
            return;
        }

        self.timer += 1;
        if self.timer < self.pace {
            return;
        }

        self.volume = match self.direction {
            true => self.volume.saturating_add(1),
            false => self.volume.saturating_sub(1),
        };

        if self.volume == 0 || self.volume == 15 {
            self.enabled = false;
        }

        self.timer = 0;
    }

    pub fn reset_timer(&mut self) {
        self.timer = 0
    }

    pub fn volume(&self) -> u8 {
        self.volume
    }

    pub fn write(&mut self, value: u8) {
        self.volume = value >> 4;
        self.direction = (value & 0x08) != 0;
        self.pace = value & 0x07;
        self.enabled = self.pace > 0;
        self.timer = 0;
    }

    pub fn read(&self) -> u8 {
        let volume = self.volume << 4;
        let direction = (self.direction as u8) << 3;
        let pace = self.pace & 0x07;
        volume | direction | pace
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.timer = 0;
        self.volume = 0;
        self.direction = true;
        self.pace = 0;
    }
}
