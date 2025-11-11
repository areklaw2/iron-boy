use getset::{CopyGetters, Setters};

#[derive(Debug, CopyGetters, Setters)]
#[getset(get_copy = "pub", set = "pub")]
pub struct VolumeEnvelope {
    initial_volume: u8,
    direction: bool,
    pace: u8,
    volume: u8,
    timer: u8,
}

impl VolumeEnvelope {
    pub fn new() -> Self {
        VolumeEnvelope {
            initial_volume: 0,
            direction: false,
            pace: 0,
            volume: 0,
            timer: 0,
        }
    }

    pub fn cycle(&mut self) {
        if self.pace != 0 {
            if self.timer > 0 {
                self.timer -= 1
            }

            if self.timer == 0 {
                self.timer = self.pace;

                if (self.volume < 0xF && self.direction) || (self.volume > 0x0 && !self.direction) {
                    match self.direction {
                        true => self.volume += 1,
                        false => self.volume -= 1,
                    }
                }
            }
        }
    }

    pub fn reset(&mut self) {
        self.timer = self.pace;
        self.volume = self.initial_volume;
    }

    pub fn disable_dac(&self) -> bool {
        self.initial_volume == 0 && !self.direction
    }

    pub fn write(&mut self, value: u8) {
        self.initial_volume = value >> 4;
        self.direction = ((value & 0x08) != 0).into();
        self.pace = value & 0x07;
    }

    pub fn read(&self) -> u8 {
        let initial_volume = self.initial_volume << 4;
        let direction = (self.direction as u8) << 3;
        let pace = self.pace & 0x07;
        initial_volume | direction | pace
    }
}
