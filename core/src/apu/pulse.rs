use crate::bus::Memory;

use super::{envelope::Envelope, sweep::Sweep, Channel};

#[derive(Clone, Copy)]
pub enum DutyCycle {
    Duty13 = 1, //12.5
    Duty25 = 2,
    Duty50 = 4,
    Duty75 = 6,
}

impl DutyCycle {
    pub fn write(field: u8) -> DutyCycle {
        match field {
            1 => DutyCycle::Duty25,
            2 => DutyCycle::Duty50,
            3 => DutyCycle::Duty75,
            _ => DutyCycle::Duty13,
        }
    }

    pub fn read(self) -> u8 {
        match self {
            DutyCycle::Duty13 => 0,
            DutyCycle::Duty25 => 1,
            DutyCycle::Duty50 => 2,
            DutyCycle::Duty75 => 3,
        }
    }
}

pub struct PulseChannel {
    running: bool,
    duty: DutyCycle,
    counter: u16,
    period: u16,
    phase: u8,
    start_envelope: Envelope,
    envelope: Envelope,
    length_enable: bool,
    remaining: u32,
    sweep: Sweep,
}

impl PulseChannel {
    pub fn new() -> PulseChannel {
        PulseChannel {
            running: false,
            duty: DutyCycle::write(0),
            counter: 0,
            period: 0,
            phase: 0,
            start_envelope: Envelope::write(0),
            envelope: Envelope::write(0),
            length_enable: false,
            remaining: 64 * 0x4000,
            sweep: Sweep::write(0),
        }
    }
}

impl Memory for PulseChannel {
    fn mem_read(&mut self, address: u16) -> u8 {
        match address {
            0xFF10 => self.sweep.read(),
            0xFF11 | 0xFF16 => self.duty.read() << 6 | 0x3F,
            0xFF12 | 0xFF17 => self.start_envelope.read(),
            0xFF13 | 0xFF18 => 0xFF,
            0xFF14 | 0xFF19 => (self.length_enable as u8) << 6 | 0xBF,
            _ => 0xFF,
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        match address {
            0xFF10 => self.sweep = Sweep::write(data),
            0xFF11 | 0xFF16 => {
                self.duty = DutyCycle::write(data >> 6);
                let length = data & 0x3F;
                if length >= 64 {
                    panic!("length out of range: {}", length);
                }
                self.remaining = (64 - (length as u32)) * 0x4000;
            }
            0xFF12 | 0xFF17 => {
                self.start_envelope = Envelope::write(data);
                if !self.start_envelope.dac_enabled() {
                    self.running = false;
                }
            }
            0xFF13 | 0xFF18 => {
                let mut period = self.period;
                period &= 0x0700;
                period |= data as u16;
                if period >= 2048 {
                    panic!("period value out of range: {:04X}", period);
                }
                self.period = period;
            }
            0xFF14 | 0xFF19 => {
                let mut period = self.period;
                period &= 0xFF;
                period |= ((data & 0x07) as u16) << 8;
                if period >= 2048 {
                    panic!("perious value out of range: {:04x}", period);
                }
                self.period = period;

                self.length_enable = data & 0x40 == 0x40;
                if data & 0x80 == 0x80 {
                    self.start();
                }
            }
            _ => {}
        }
    }
}

impl Channel for PulseChannel {
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
        self.period = match self.sweep.step(self.period) {
            Some(period) => period,
            None => {
                self.running = false;
                return;
            }
        };

        if self.counter == 0 {
            self.counter = 4 * (0x0800 - self.period);
            self.phase = (self.phase + 1) % 8;
        }

        self.counter -= 1;
    }

    fn sample(&self) -> u8 {
        if !self.running {
            return 0;
        }

        if self.phase < (self.duty as u8) {
            self.envelope.read_volume()
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
