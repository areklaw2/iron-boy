use crate::apu::length_timer::{DEFAULT_MAX_LENGTH, Length};
use crate::apu::period::Period;
use crate::apu::sweep::Sweep;
use crate::apu::volume_envelope::VolumeEnvelope;
use crate::system_bus::SystemMemoryAccess;
use crate::{GbMode, T_CYCLES_PER_STEP};

use getset::{CopyGetters, Setters};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug, CopyGetters, Setters)]
pub struct PulseChannel {
    #[getset(get_copy = "pub", set = "pub")]
    enabled: bool,
    dac_enabled: bool,
    wave_duty_position: u8,
    sweep: Option<Sweep>,
    length: Length,
    envelope: VolumeEnvelope,
    period: Period,
    gb_mode: GbMode,
    div_apu_step: Rc<RefCell<u8>>,
}

const WAVEFORMS: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1],
    [0, 0, 0, 0, 0, 0, 1, 1],
    [0, 0, 0, 0, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 0, 0],
];

impl SystemMemoryAccess for PulseChannel {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0xFF10 => match &self.sweep {
                Some(sweep) => sweep.read() | 0x80,
                None => 0xFF,
            },
            0xFF11 | 0xFF16 => self.length.initial_time() | 0x3F,
            0xFF12 | 0xFF17 => self.envelope.read(),
            0xFF14 | 0xFF19 => self.period.high() | 0xBF,
            _ => 0xFF,
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0xFF10 => self.write_sweep(value),
            0xFF11 | 0xFF16 => self.write_length_timer(value),
            0xFF12 | 0xFF17 => self.write_volume_envelope(value),
            0xFF13 | 0xFF18 => self.write_period_low(value),
            0xFF14 | 0xFF19 => self.write_period_high(value),
            _ => {}
        }
    }
}

impl PulseChannel {
    pub fn new(with_sweep: bool, gb_mode: GbMode, div_apu_step: Rc<RefCell<u8>>) -> Self {
        let sweep = match with_sweep {
            true => Some(Sweep::new()),
            false => None,
        };

        PulseChannel {
            enabled: false,
            dac_enabled: false,
            wave_duty_position: 0,
            sweep: sweep,
            length: Length::new(DEFAULT_MAX_LENGTH),
            envelope: VolumeEnvelope::new(),
            period: Period::new(),
            gb_mode,
            div_apu_step,
        }
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.dac_enabled = false;
        self.wave_duty_position = 0;
        if self.sweep.is_some() {
            self.sweep = Some(Sweep::new());
        }
        self.envelope = VolumeEnvelope::new();
        self.period = Period::new();
        if self.gb_mode != GbMode::Color {
            self.length.reset();
        } else {
            self.length = Length::new(DEFAULT_MAX_LENGTH);
        }
    }

    pub fn cycle(&mut self) {
        if self.enabled {
            self.period.cycle(T_CYCLES_PER_STEP / 4, || {
                self.wave_duty_position = (self.wave_duty_position + 1) % 8;
            });
        }
    }

    pub fn cycle_envelope(&mut self) {
        if self.enabled {
            self.envelope.cycle();
        }
    }

    pub fn cycle_length_on_enable(&self, value: u8) -> bool {
        (value & (1 << 6)) == 0 && (self.period.high() & (1 << 6)) != 0
    }

    pub fn cycle_length_on_trigger(&self) -> bool {
        self.length.maxxed() && (self.period.high() & (1 << 6)) != 0
    }

    pub fn cycle_length(&mut self) {
        let length_timer_enabled = (self.period.high() & (1 << 6)) != 0;
        if length_timer_enabled {
            self.length.cycle();
            if self.length.expired() {
                self.set_enabled(false);
            }
        }
    }

    pub fn digital_output(&self) -> f32 {
        if self.enabled {
            let length_settings = self.length.initial_time();
            let wave_duty = (length_settings & 0b11000000) >> 6;
            let amplitude = WAVEFORMS[wave_duty as usize][self.wave_duty_position as usize];
            let current_volume = self.envelope.volume();
            (amplitude * current_volume) as f32
        } else {
            7.5
        }
    }

    pub fn cycle_sweep(&mut self) {
        if self.enabled {
            if let Some(sweep) = &mut self.sweep {
                sweep.cycle(&mut self.period);
                if sweep.disable_channel() {
                    self.set_enabled(false);
                }
            }
        }
    }

    pub fn trigger(&mut self) {
        if self.dac_enabled {
            self.enabled = true;
        }
        self.period.trigger();
        self.length.reload();
        self.envelope.reset();
        if let Some(sweep) = &mut self.sweep {
            sweep.trigger(self.period.value());
            if sweep.disable_channel() {
                self.set_enabled(false);
            }
        }
    }

    pub fn should_trigger(&self) -> bool {
        (self.period.high() & (1 << 7)) != 0
    }

    fn write_sweep(&mut self, value: u8) {
        if let Some(sweep) = &mut self.sweep {
            sweep.write(value);
            if sweep.disable_channel() {
                self.enabled = false;
            }
        }
    }

    fn write_length_timer(&mut self, value: u8) {
        self.length.set_initial_time(value);
        self.length.initialize();
    }

    fn write_volume_envelope(&mut self, value: u8) {
        self.envelope.write(value);
        self.envelope.reset();

        self.dac_enabled = !self.envelope.disable_dac();
        if !self.dac_enabled {
            self.enabled = false;
        }
    }

    fn write_period_low(&mut self, value: u8) {
        self.period.set_low(value);
    }

    fn write_period_high(&mut self, value: u8) {
        let period_high = self.period.high();
        self.period.set_high(value);

        let period_in_first_half = matches!(*self.div_apu_step.borrow(), 1 | 3 | 5 | 7);
        if self.cycle_length_on_enable(period_high) && period_in_first_half {
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
