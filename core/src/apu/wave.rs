use crate::bus::Memory;

use super::{Channel, Mode, Sample};

const WAVE_INITIAL_DELAY: u32 = 4;

#[derive(Debug, Clone, Copy)]
pub enum OutputLevel {
    Mute = 0,
    Full = 1,
    Halved = 2,
    Quartered = 3,
}

impl OutputLevel {
    pub fn write(field: u8) -> OutputLevel {
        match field {
            1 => OutputLevel::Full,
            2 => OutputLevel::Halved,
            3 => OutputLevel::Quartered,
            _ => OutputLevel::Mute,
        }
    }

    pub fn read(self) -> u8 {
        self as u8
    }

    fn process(self, sample: Sample) -> Sample {
        match self {
            OutputLevel::Mute => 0,
            OutputLevel::Full => sample,
            OutputLevel::Halved => sample / 2,
            OutputLevel::Quartered => sample / 4,
        }
    }
}

pub struct WaveChannel {
    running: bool,
    enabled: bool,
    remaining: u32,
    output_level: OutputLevel,
    divider: u16,
    counter: u16,
    mode: Mode,
    samples: [Sample; 32],
    index: u8,
}

impl Memory for WaveChannel {
    fn mem_read(&mut self, address: u16) -> u8 {
        match address {
            0xFF1A => (self.enabled as u8) << 7 | 0x7f,
            0xFF1B => 0xFF,
            0xFF1C => self.output_level.read() << 5 | 0x9f,
            0xFF1D => 0xFF,
            0xFF1E => (self.mode as u8) << 6 | 0xBF,
            0xFF30..=0xFF3F => {
                let address = (address & 0xFF30) as usize * 2;
                let sample0 = self.samples[address] as u8;
                let sample2 = self.samples[address + 1] as u8;
                sample0 << 4 | sample2
            }
            _ => 0xFF,
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        match address {
            0xFF1A => self.set_enabled(data & 0x80 == 0x80),
            0xFF1B => self.set_length(data),
            0xFF1C => self.output_level = OutputLevel::write((data >> 5) & 3),
            0xFF1D => {
                let mut divider = self.divider;
                divider &= 0x700;
                divider |= data as u16;
                self.set_divider(divider);
            }
            0xFF1E => {
                let mut divider = self.divider;

                divider &= 0xFF;
                divider |= ((data & 7) as u16) << 8;
                self.set_divider(divider);

                self.mode = match data & 0x40 == 0x40 {
                    true => Mode::Counter,
                    false => Mode::Continuous,
                };

                if data & 0x80 == 0x80 {
                    self.start();
                }
            }
            0xFF30..=0xFF3F => {
                let address = (address & 0xFF30) as usize * 2;
                let sample0 = (data >> 4) as Sample;
                let sample1 = (data & 0xf) as Sample;
                self.samples[address] = sample0;
                self.samples[address + 1] = sample1;
            }
            _ => {}
        }
    }
}

impl Channel for WaveChannel {
    fn step(&mut self) {
        if self.mode == Mode::Counter {
            if self.remaining == 0 {
                self.running = false;
                self.remaining = 0x100 * 0x4000;
                return;
            }
            self.remaining -= 1;
        }

        if !self.running {
            return;
        }

        if self.counter == 0 {
            self.counter = 2 * (0x800 - self.divider);
            self.index = (self.index + 1) % self.samples.len() as u8;
        }

        self.counter -= 1;
    }

    fn sample(&self) -> Sample {
        if !self.running {
            return 0;
        }
        let sample = self.samples[self.index as usize];
        self.output_level.process(sample)
    }

    fn running(&self) -> bool {
        self.running
    }

    fn start(&mut self) {
        self.running = self.enabled;
    }
}

impl WaveChannel {
    pub fn new() -> WaveChannel {
        WaveChannel {
            running: false,
            enabled: false,
            remaining: 0x100 * 0x4000,
            output_level: OutputLevel::write(0),
            divider: 0,
            counter: 0,
            mode: Mode::Continuous,
            samples: [0; 32],
            index: 0,
        }
    }

    pub fn with_ram(other: &WaveChannel) -> WaveChannel {
        let mut wave_channel = WaveChannel::new();
        wave_channel.samples = other.samples;
        wave_channel
    }

    pub fn set_divider(&mut self, divider: u16) {
        if divider >= 0x800 {
            panic!("divider out of range: {:04x}", divider);
        }
        self.divider = divider;
    }

    pub fn set_length(&mut self, length: u8) {
        let len = length as u32;
        self.remaining = (0x100 - len) * 0x4000;
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        if !self.enabled {
            self.running = false;
        }
    }
}
