use crate::system_bus::SystemMemoryAccess;

use super::channel::{Channel, ChannelBase, length_timer::LengthTimer, volume_envelope::VolumeEnvelope};

const DIVISORS: [u8; 8] = [8, 16, 32, 48, 64, 80, 96, 112];
const LENGTH_TIMER_MAX: u16 = 64;

pub struct NoiseChannel {
    base: ChannelBase,
    length_timer: LengthTimer,
    volume_envelope: VolumeEnvelope,
    lfsr: u16,
    clock_divider: u8,
    lfsr_width: bool,
    clock_shift: u8,
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
    fn cycle(&mut self, cycles: u32) {
        if !self.base.enabled || !self.base.dac_enabled {
            return;
        }

        let cycles = cycles as u16;
        self.base.timer = self.base.timer.saturating_sub(cycles as i32);
        if self.base.timer > 0 {
            return;
        }

        let result = ((self.lfsr & 0x01) ^ ((self.lfsr >> 1) & 0x01)) != 0;
        self.lfsr >>= 1;
        self.lfsr |= if result { 0x01 << 14 } else { 0x00 };

        if self.lfsr_width {
            self.lfsr &= 0xBF;
            self.lfsr |= if result { 0x40 } else { 0x00 };
        }

        self.base.output = if result { self.volume_envelope.volume() } else { 0x00 };
        self.base.timer += (DIVISORS[self.clock_divider as usize] as i32) << self.clock_shift;
    }

    fn length_timer_cycle(&mut self) {
        self.length_timer.cycle(&mut self.base.enabled)
    }

    fn volume_envelope_cycle(&mut self) {
        self.volume_envelope.cycle(self.base.enabled);
    }

    fn trigger(&mut self) {
        if self.base.dac_enabled {
            self.base.enabled = true;
        }

        self.base.timer = (DIVISORS[self.clock_divider as usize] as i32) << self.clock_shift;
        self.lfsr = 0x7FF1;
        self.volume_envelope.reset_timer();

        if self.length_timer.time() == 0 {
            self.length_timer.set_time(LENGTH_TIMER_MAX);
        }
    }

    fn reset(&mut self) {
        self.base.reset();
        self.length_timer.reset();
        self.volume_envelope.reset();
        self.lfsr = 0;
        self.clock_divider = 0;
        self.lfsr_width = false;
        self.clock_shift = 0;
    }

    fn enabled(&self) -> bool {
        self.base.enabled
    }

    fn output(&self) -> u8 {
        self.base.output()
    }
}

impl NoiseChannel {
    pub fn new() -> Self {
        Self {
            base: ChannelBase::new(),
            length_timer: LengthTimer::new(),
            volume_envelope: VolumeEnvelope::new(),
            lfsr: 0,
            clock_divider: 0,
            lfsr_width: false,
            clock_shift: 0,
        }
    }

    fn volume_envelope_write(&mut self, value: u8) {
        self.volume_envelope.write(value);
        self.base.dac_enabled = value & 0xF8 != 0;
        if !self.base.dac_enabled {
            self.base.enabled = false;
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
        let triggered = value & 0x80 == 0x80;
        if triggered {
            self.trigger();
        }
        self.length_timer.set_enabled(value & 0x40 == 0x40);
    }
}
