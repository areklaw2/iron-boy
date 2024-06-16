use crate::bus::Memory;

pub struct VolumeEnvelope {
    initial_volume: u8,
    direction: bool,
    sweep_pace: u8,
    delay: u8,
    pub volume: u8,
}

impl Memory for VolumeEnvelope {
    fn mem_read(&mut self, address: u16) -> u8 {
        match address {
            0xFF12 | 0xFF17 | 0xFF21 => {
                let mut data = 0;
                data |= (self.initial_volume & 0xF) << 4;
                data |= (self.direction as u8) << 3;
                data |= self.sweep_pace & 0x7;
                data
            }
            _ => 0xFF,
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        match address {
            0xFF12 | 0xFF17 | 0xFF21 => {
                self.initial_volume = data >> 4;
                self.direction = data & 0x8 == 0x8;
                self.sweep_pace = data & 0x7;
                self.volume = self.initial_volume;
            }
            0xFF14 | 0xFF19 | 0xFF23 => {
                if data & 0x80 == 0x80 {
                    self.volume = self.initial_volume;
                    self.delay = self.sweep_pace;
                }
            }
            _ => {}
        }
    }
}

impl VolumeEnvelope {
    pub fn new() -> VolumeEnvelope {
        VolumeEnvelope {
            sweep_pace: 0,
            direction: false,
            delay: 0,
            initial_volume: 0,
            volume: 0,
        }
    }

    pub fn step(&mut self) {
        if self.delay > 1 {
            self.delay -= 1;
        } else if self.delay == 1 {
            self.delay = self.sweep_pace;
            if self.direction && self.volume < 15 {
                self.volume += 1;
            } else if !self.direction && self.volume > 0 {
                self.volume -= 1;
            }
        }
    }
}
