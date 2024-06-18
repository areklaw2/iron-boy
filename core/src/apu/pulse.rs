use crate::bus::Memory;

use super::{envelope::Envelope, sweep::Sweep, Channel, Mode, Sample};

const SWEEP_DELAY_ZERO_PACE: u8 = 8;

pub struct PulseChannel {
    running: bool,
    duty: u8,
    counter: u16,
    divider: u16,
    phase: u8,
    start_envelope: Envelope,
    envelope: Envelope,
    mode: Mode,
    remaining: u32,
    sweep: Sweep,
}

impl Memory for PulseChannel {
    fn mem_read(&mut self, address: u16) -> u8 {
        match address {
            0xFF10 => self.sweep.read(),
            0xFF11 | 0xFF16 => self.duty << 6 | 0x3F,
            0xFF12 | 0xFF17 => self.start_envelope.read(),
            0xFF13 | 0xFF18 => 0xFF,
            0xFF14 | 0xFF19 => (self.mode as u8) << 6 | 0xBF,
            _ => 0xFF,
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        match address {
            0xFF10 => self.sweep = Sweep::write(data),
            0xFF11 | 0xFF16 => {
                self.duty = data >> 6;
                self.set_length(data & 0x3F);
            }
            0xFF12 | 0xFF17 => self.set_envelope(Envelope::write(data)),
            0xFF13 | 0xFF18 => {
                let mut divider = self.divider;
                divider &= 0x0700;
                divider |= data as u16;
                self.set_divider(divider)
            }
            0xFF14 | 0xFF19 => {
                let mut divider = self.divider;
                divider &= 0xff;
                divider |= ((data & 0x07) as u16) << 8;
                self.set_divider(divider);
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

impl Channel for PulseChannel {
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
        self.divider = match self.sweep.step(self.divider) {
            Some(div) => div,
            None => {
                self.running = false;
                return;
            }
        };

        if self.counter == 0 {
            self.counter = 4 * (0x800 - self.divider);
            self.phase = (self.phase + 1) % 8;
        }

        self.counter -= 1;
    }

    fn sample(&self) -> Sample {
        if !self.running {
            return 0;
        }

        if self.phase < self.duty {
            self.envelope.read()
        } else {
            0
        }
    }

    fn start(&mut self) {
        self.envelope = self.start_envelope;
        self.running = self.envelope.dac_enabled();
    }

    fn running(&self) -> bool {
        self.running
    }
}

impl PulseChannel {
    pub fn new() -> PulseChannel {
        PulseChannel {
            running: false,
            duty: 0,
            counter: 0,
            divider: 0,
            phase: 0,
            start_envelope: Envelope::write(0),
            envelope: Envelope::write(0),
            mode: Mode::Continuous,
            remaining: 64 * 0x4000,
            sweep: Sweep::write(0),
        }
    }

    fn set_divider(&mut self, divider: u16) {
        if divider >= 0x800 {
            panic!("divider out of range: {:04x}", divider);
        }

        self.divider = divider;
    }

    fn set_envelope(&mut self, envelope: Envelope) {
        self.start_envelope = envelope;
        if !self.envelope.dac_enabled() {
            self.running = false;
        }
    }

    fn set_length(&mut self, length: u8) {
        if length >= 64 {
            panic!("sound length out of range: {}", length);
        }
        let length = length as u32;
        self.remaining = (64 - length) * 0x4000;
    }
}
