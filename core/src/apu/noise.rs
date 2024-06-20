use crate::bus::Memory;

use super::{envelope::Envelope, lsfr::Lfsr, Channel};

pub struct NoiseChannel {
    running: bool,
    lfsr: Lfsr,
    start_envelope: Envelope,
    envelope: Envelope,
    length_enable: bool,
    remaining: u32,
}

impl NoiseChannel {
    pub fn new() -> NoiseChannel {
        NoiseChannel {
            lfsr: Lfsr::write(0),
            start_envelope: Envelope::write(0),
            envelope: Envelope::write(0),
            remaining: 64 * 0x4000,
            length_enable: false,
            running: false,
        }
    }
}

impl Memory for NoiseChannel {
    fn mem_read(&mut self, address: u16) -> u8 {
        match address {
            0xFF20 => 0xFF,
            0xFF21 => self.start_envelope.read(),
            0xFF22 => self.lfsr.read(),
            0xFF23 => (self.length_enable as u8) << 6 | 0xBF,
            _ => 0xFF,
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        match address {
            0xFF20 => {
                let length = data & 0x3F;
                if length >= 64 {
                    panic!("length out of range: {}", length);
                }
                self.remaining = (64 - (length as u32)) * 0x4000;
            }
            0xFF21 => {
                self.start_envelope = Envelope::write(data);
                if !self.start_envelope.dac_enabled() {
                    self.running = false;
                }
            }
            0xFF22 => self.lfsr = Lfsr::write(data),
            0xFF23 => {
                self.length_enable = data & 0x40 == 0x40;
                if data & 0x80 != 0 {
                    self.start();
                }
            }
            _ => {}
        }
    }
}

impl Channel for NoiseChannel {
    fn step(&mut self) {
        if self.length_enable {
            if self.remaining == 0 {
                self.running = false;
                self.remaining = 64 * 0x4000;
                return;
            }

            self.remaining -= 1;
        }

        if !self.running {
            return;
        }

        self.envelope.step();
        self.lfsr.step();
    }

    fn sample(&self) -> u8 {
        if !self.running {
            return 0;
        }

        if self.lfsr.high() {
            self.envelope.read_volume()
        } else {
            0
        }
    }

    fn running(&self) -> bool {
        self.running
    }

    fn start(&mut self) {
        self.envelope = self.start_envelope;
        self.running = self.envelope.dac_enabled();
    }
}
