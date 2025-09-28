use std::{cell::RefCell, rc::Rc};

use crate::{T_CYCLES_PER_STEP, system_bus::SystemMemoryAccess};

use super::{Channel, length_timer::LengthTimer, volume_envelope::VolumeEnvelope};

const DIVISORS: [u8; 8] = [8, 16, 32, 48, 64, 80, 96, 112];
const LENGTH_TIMER_MAX: u16 = 64;

pub struct NoiseChannel {
    sample: u8,
    enabled: bool,
    dac_enabled: bool,
    trigger: bool,
    period_timer: i32,
    length_timer: LengthTimer,
    volume_envelope: VolumeEnvelope,
    lfsr: u16,
    clock_divider: u8,
    lfsr_width: bool,
    clock_shift: u8,
    div_apu_step: Rc<RefCell<u8>>,
}

impl SystemMemoryAccess for NoiseChannel {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0xFF21 => self.volume_envelope.read(),
            0xFF22 => self.frequency_randomness_read(),
            0xFF23 => (self.length_timer.enabled() as u8) << 6 | 0xBF,
            _ => 0xFF,
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0xFF20 => self.length_timer.set_time(LENGTH_TIMER_MAX - (value & 0x3F) as u16),
            0xFF21 => self.volume_envelope_write(value),
            0xFF22 => self.frequency_randomness_write(value),
            0xFF23 => self.control_write(value),
            _ => {}
        }
    }
}

impl Channel for NoiseChannel {
    fn cycle(&mut self) {
        if !self.enabled || !self.dac_enabled {
            return;
        }

        self.period_timer = self.period_timer.saturating_sub(T_CYCLES_PER_STEP as i32);
        if self.period_timer > 0 {
            return;
        }

        let result = ((self.lfsr & 0x01) ^ ((self.lfsr >> 1) & 0x01)) != 0;
        self.lfsr >>= 1;
        self.lfsr |= if result { 0x01 << 14 } else { 0x00 };

        if self.lfsr_width {
            self.lfsr &= 0xBF;
            self.lfsr |= if result { 0x40 } else { 0x00 };
        }

        self.sample = if result { self.volume_envelope.volume() } else { 0x00 };
        self.period_timer += (DIVISORS[self.clock_divider as usize] as i32) << self.clock_shift;
    }

    fn length_timer_cycle(&mut self) {
        self.length_timer.cycle(&mut self.enabled)
    }

    fn volume_envelope_cycle(&mut self) {
        self.volume_envelope.cycle(self.enabled);
    }

    fn trigger(&mut self) {
        if self.dac_enabled {
            self.enabled = true;
        }

        self.period_timer = (DIVISORS[self.clock_divider as usize] as i32) << self.clock_shift;
        self.lfsr = 0x7FF1;
        self.volume_envelope.reset_timer();

        if self.length_timer.time() == 0 {
            self.length_timer.set_time(LENGTH_TIMER_MAX);
        }
    }

    fn reset(&mut self) {
        self.enabled = false;
        self.dac_enabled = false;
        self.sample = 0;
        self.period_timer = 0;
        self.trigger = false;
        self.length_timer.reset();
        self.volume_envelope.reset();
        self.lfsr = 0;
        self.clock_divider = 0;
        self.lfsr_width = false;
        self.clock_shift = 0;
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn sample(&self) -> u8 {
        if self.enabled && self.dac_enabled { self.sample } else { 0 }
    }
}

impl NoiseChannel {
    pub fn new(div_apu_step: Rc<RefCell<u8>>) -> Self {
        Self {
            sample: 0,
            enabled: false,
            dac_enabled: false,
            trigger: false,
            period_timer: 0,
            length_timer: LengthTimer::new(),
            volume_envelope: VolumeEnvelope::new(),
            lfsr: 0,
            clock_divider: 0,
            lfsr_width: false,
            clock_shift: 0,
            div_apu_step,
        }
    }

    fn volume_envelope_write(&mut self, value: u8) {
        self.volume_envelope.write(value);
        self.dac_enabled = value & 0xF8 != 0;
        if !self.dac_enabled {
            self.enabled = false;
        }
    }

    fn frequency_randomness_read(&self) -> u8 {
        let clock_shift = (self.clock_shift & 0x0F) << 4;
        let lfsr_width = (self.lfsr_width as u8) << 3;
        let clock_divider = self.clock_divider & 0x07;
        clock_shift | lfsr_width | clock_divider
    }

    fn frequency_randomness_write(&mut self, value: u8) {
        self.clock_shift = (value & 0xF0) >> 4;
        self.lfsr_width = value & 0x08 == 0x08;
        self.clock_divider = value & 0x07;
    }

    fn control_write(&mut self, value: u8) {
        let first_half_of_cycle = matches!(*self.div_apu_step.borrow(), 1 | 3 | 5 | 7);
        let length_will_enabled = !self.length_timer.enabled() && value & 0x40 != 0;
        self.length_timer.set_enabled(value & 0x40 != 0);
        if first_half_of_cycle && length_will_enabled {
            self.length_timer.cycle(&mut self.enabled);
        }

        if value & 0x80 != 0 {
            self.trigger();

            if first_half_of_cycle && self.length_timer.time() == LENGTH_TIMER_MAX {
                self.length_timer.cycle(&mut self.enabled);
            }
        }
    }
}
