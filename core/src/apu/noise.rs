use crate::bus::Memory;

use super::{envelope::Envelope, lsfr::Lfsr, Channel, Mode, Sample};

pub struct NoiseChannel {
    running: bool,
    lfsr: Lfsr,
    start_envelope: Envelope,
    envelope: Envelope,
    mode: Mode,
    remaining: u32,
}

impl Memory for NoiseChannel {
    fn mem_read(&mut self, address: u16) -> u8 {
        match address {
            0xFF20 => 0xFF,
            0xFF21 => self.envelope.read(),
            0xFF22 => self.lfsr.read(),
            0xFF23 => (self.mode as u8) << 6 | 0xBF,
            _ => 0xFF,
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        match address {
            0xFF20 => self.set_length(data & 0x3F),
            0xFF21 => self.set_envelope(Envelope::write(data)),
            0xFF22 => self.lfsr = Lfsr::write(data),
            0xFF23 => {
                self.mode = match data & 0x40 == 0x40 {
                    true => Mode::Counter,
                    false => Mode::Continuous,
                };

                if data & 0x80 == 0x80 {
                    self.start()
                }
            }
            _ => {}
        }
    }
}

impl Channel for NoiseChannel {
    fn step(&mut self) {
        if self.mode == Mode::Counter {
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

    fn sample(&self) -> Sample {
        if !self.running {
            return 0;
        }
        if self.lfsr.high() {
            self.envelope.read_as_sample()
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

impl NoiseChannel {
    pub fn new() -> NoiseChannel {
        NoiseChannel {
            lfsr: Lfsr::write(0),
            start_envelope: Envelope::write(0),
            envelope: Envelope::write(0),
            remaining: 64 * 0x4000,
            mode: Mode::Continuous,
            running: false,
        }
    }

    fn set_envelope(&mut self, envelope: Envelope) {
        self.start_envelope = envelope;
        if !envelope.dac_enabled() {
            self.running = false;
        }
    }

    fn set_length(&mut self, length: u8) {
        if length >= 64 {
            panic!("length out of range: {}", length);
        }
        let length = length as u32;
        self.remaining = (64 - length) * 0x4000;
    }
}
