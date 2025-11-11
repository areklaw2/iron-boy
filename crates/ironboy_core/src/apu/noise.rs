use std::cell::RefCell;
use std::rc::Rc;

use crate::apu::length_timer::{DEFAULT_MAX_LENGTH, Length};
use crate::apu::volume_envelope::VolumeEnvelope;
use crate::system_bus::SystemMemoryAccess;
use crate::{GbMode, T_CYCLES_PER_STEP};
use getset::{CopyGetters, Setters};

#[derive(Debug, CopyGetters, Setters)]
pub struct NoiseChannel {
    #[getset(get_copy = "pub", set = "pub")]
    enabled: bool,
    #[getset(get_copy = "pub", set = "pub")]
    dac_enabled: bool,
    length: Length,
    envelope: VolumeEnvelope,
    polynomial: u8,
    lfsr: u16,
    control: u8,
    period_divider: u16,
    instruction_cycles: u16,
    gb_mode: GbMode,
    div_apu_step: Rc<RefCell<u8>>,
}

impl SystemMemoryAccess for NoiseChannel {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0xFF21 => self.envelope.read(),
            0xFF22 => self.polynomial,
            0xFF23 => self.control | 0xBF,
            _ => 0xFF,
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0xFF20 => self.write_length_timer(value),
            0xFF21 => self.write_volume_envolope(value),
            0xFF22 => self.write_polynomial(value),
            0xFF23 => self.write_control(value),
            _ => {}
        }
    }
}

impl NoiseChannel {
    pub fn new(gb_mode: GbMode, div_apu_step: Rc<RefCell<u8>>) -> Self {
        NoiseChannel {
            enabled: false,
            dac_enabled: false,
            length: Length::new(DEFAULT_MAX_LENGTH),
            envelope: VolumeEnvelope::new(),
            polynomial: 0,
            lfsr: 0,
            control: 0,
            period_divider: 0,
            instruction_cycles: 0,
            gb_mode,
            div_apu_step,
        }
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.dac_enabled = false;
        self.envelope = VolumeEnvelope::new();
        self.polynomial = 0;
        self.lfsr = 0;
        self.control = 0;
        self.period_divider = 0;
        self.instruction_cycles = 0;

        if self.gb_mode != GbMode::Color {
            self.length.reset();
        } else {
            self.length = Length::new(DEFAULT_MAX_LENGTH);
        }
    }

    fn period_timer(&self) -> u16 {
        let clock_shift = (self.polynomial & 0xF0) >> 4;
        let clock_divider = self.polynomial & 0x07;
        let divisor = if clock_divider == 0 { 8 } else { (clock_divider as u16) << 4 };
        divisor << clock_shift
    }

    fn next_lfsr(&self) -> u16 {
        let narrow_width = (self.polynomial & (1 << 3)) != 0;
        let first_lfsr_bit = self.lfsr & 0b01;
        let second_lfsr_bit = (self.lfsr & 0b10) >> 1;
        let result = (!(first_lfsr_bit ^ second_lfsr_bit)) & 0b1;

        let mut next_lfsr = self.lfsr | (result << 15);

        if narrow_width {
            next_lfsr |= result << 7;
        }

        next_lfsr >> 1
    }

    pub fn cycle(&mut self) {
        self.instruction_cycles += T_CYCLES_PER_STEP as u16;
        if self.instruction_cycles >= self.period_divider {
            self.instruction_cycles = 0;
            self.period_divider = self.period_timer();
            self.lfsr = self.next_lfsr();
        }
    }

    pub fn cycle_envelope(&mut self) {
        if self.enabled {
            self.envelope.cycle();
        }
    }

    pub fn cycle_length_on_enable(&self, value: u8) -> bool {
        (value & (1 << 6)) == 0 && (self.control & (1 << 6)) != 0
    }

    pub fn cycle_length_on_trigger(&self) -> bool {
        self.length.maxxed() && (self.control & (1 << 6)) != 0
    }

    pub fn cycle_length(&mut self) {
        if (self.control & (1 << 6)) != 0 {
            self.length.cycle();
            if self.length.expired() {
                self.set_enabled(false);
            }
        }
    }

    pub fn digital_output(&self) -> f32 {
        if self.enabled {
            let amplitude = (self.lfsr & 0x01) as u8;
            let current_volume = self.envelope.volume();
            (amplitude * current_volume) as f32
        } else {
            7.5
        }
    }

    pub fn trigger(&mut self) {
        if self.dac_enabled {
            self.enabled = true;
        }
        self.period_divider = self.period_timer();
        self.lfsr = 0;
        self.length.reload();
        self.envelope.reset();
    }

    pub fn should_trigger(&self) -> bool {
        (self.control & (1 << 7)) != 0
    }

    fn write_length_timer(&mut self, value: u8) {
        self.length.set_initial_time(value);
        self.length.initialize();
    }

    fn write_volume_envolope(&mut self, value: u8) {
        self.envelope.write(value);
        self.envelope.reset();

        self.dac_enabled = !self.envelope.disable_dac();
        if !self.dac_enabled {
            self.enabled = false;
        }
    }

    fn write_polynomial(&mut self, value: u8) {
        self.polynomial = value;
    }

    fn write_control(&mut self, value: u8) {
        let control = self.control;
        self.control = value;

        let period_in_first_half = matches!(*self.div_apu_step.borrow(), 1 | 3 | 5 | 7);
        if self.cycle_length_on_enable(control) && period_in_first_half {
            self.cycle_length();
        }

        if self.should_trigger() {
            self.trigger();

            if self.cycle_length_on_trigger() && period_in_first_half {
                self.cycle_length();
            }
        }
    }
}
