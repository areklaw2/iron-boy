use crate::bus::Memory;

use super::Channel;

#[derive(Clone, Copy)]
pub enum Level {
    Mute = 0,
    Full = 1,
    Halved = 2,
    Quartered = 3,
}

impl Level {
    pub fn write(field: u8) -> Level {
        match field {
            1 => Level::Full,
            2 => Level::Halved,
            3 => Level::Quartered,
            _ => Level::Mute,
        }
    }

    pub fn read(self) -> u8 {
        self as u8
    }

    fn process(self, data: u8) -> u8 {
        match self {
            Level::Mute => 0,
            Level::Full => data,
            Level::Halved => data / 2,
            Level::Quartered => data / 4,
        }
    }
}

pub struct WaveChannel {
    running: bool,
    enabled: bool,
    remaining: u32,
    output_level: Level,
    period: u16,
    counter: u16,
    length_enable: bool,
    samples: [u8; 32],
    current_sample: u8,
}

impl WaveChannel {
    pub fn new() -> WaveChannel {
        WaveChannel {
            running: false,
            enabled: false,
            remaining: 0x100 * 0x4000,
            output_level: Level::write(0),
            period: 0,
            counter: 0,
            length_enable: false,
            samples: [0; 32],
            current_sample: 0,
        }
    }

    pub fn with_ram(other: &WaveChannel) -> WaveChannel {
        let mut wave_channel = WaveChannel::new();
        wave_channel.samples = other.samples;
        wave_channel
    }
}

impl Memory for WaveChannel {
    fn mem_read(&mut self, address: u16) -> u8 {
        match address {
            0xFF1A => ((self.enabled as u8) << 7) | 0x7F,
            0xFF1B => 0xFF,
            0xFF1C => (self.output_level.read() << 5) | 0x9F,
            0xFF1D => 0xFF,
            0xFF1E => (self.length_enable as u8) << 6 | 0xBF,
            0xFF30..=0xFF3F => {
                let address = (address - 0xFF30) * 2;
                let byte0 = self.samples[address as usize] as u8;
                let byte1 = self.samples[address as usize + 1] as u8;
                byte0 << 4 | byte1
            }
            _ => 0xFF,
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        match address {
            0xFF1A => {
                self.enabled = data & 0x80 == 0x80;
                if !self.enabled {
                    self.running = false;
                }
            }
            0xFF1B => self.remaining = (256 - (data as u32)) * 0x4000,
            0xFF1C => self.output_level = Level::write((data >> 5) & 0b11),
            0xFF1D => {
                let mut period = self.period;
                period &= 0x0700;
                period |= data as u16;
                if period >= 2048 {
                    panic!("divider out of range: {:04x}", period);
                }
                self.period = period;
            }
            0xFF1E => {
                let mut period = self.period;

                period &= 0xff;
                period |= ((data & 0x07) as u16) << 8;
                if period >= 2048 {
                    panic!("period value out of range: {:04x}", period);
                }
                self.period = period;

                self.length_enable = data & 0x40 == 0x40;
                if data & 0x80 == 0x80 {
                    self.start();
                }
            }
            0xFF30..=0xFF3F => {
                let address = (address - 0xFF30) * 2;
                let byte0 = (data >> 4) as u8;
                let byte1 = (data & 0x0F) as u8;
                self.samples[address as usize] = byte0;
                self.samples[address as usize + 1] = byte1;
            }
            _ => {}
        }
    }
}

impl Channel for WaveChannel {
    fn step(&mut self) {
        if self.length_enable {
            if self.remaining == 0 {
                self.running = false;
                self.remaining = 256 * 0x4000;
                return;
            }

            self.remaining -= 1;
        }

        if !self.running {
            return;
        }

        if self.counter == 0 {
            self.counter = 2 * (0x0800 - self.period);
            self.current_sample = (self.current_sample + 1) % self.samples.len() as u8;
        }
        self.counter -= 1;
    }

    fn sample(&self) -> u8 {
        if !self.running {
            return 0;
        }

        let sample = self.samples[self.current_sample as usize];
        self.output_level.process(sample)
    }

    fn running(&self) -> bool {
        self.running
    }

    fn start(&mut self) {
        self.running = self.enabled;
    }
}
