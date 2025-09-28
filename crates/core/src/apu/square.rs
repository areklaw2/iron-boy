use std::{cell::RefCell, rc::Rc};

use crate::{T_CYCLES_PER_STEP, system_bus::SystemMemoryAccess};

use super::{Channel, length_timer::LengthTimer, sweep::Sweep, volume_envelope::VolumeEnvelope};

const DUTY_TABLE: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1],
    [0, 0, 0, 0, 0, 0, 1, 1],
    [0, 0, 0, 0, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 0, 0],
];
const LENGTH_TIMER_MAX: u16 = 64;

pub struct SquareChannel {
    sample: u8,
    enabled: bool,
    dac_enabled: bool,
    trigger: bool,
    period_timer: i32,
    length_timer: LengthTimer,
    volume_envelope: VolumeEnvelope,
    sweep: Option<Sweep>,
    period: u16,
    wave_index: u8,
    amplitude_index: u8,
    div_apu_step: Rc<RefCell<u8>>,
}

impl SystemMemoryAccess for SquareChannel {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0xFF10 => match &self.sweep {
                Some(sweep) => sweep.read(),
                None => 0xFF,
            },
            0xFF11 | 0xFF16 => self.wave_index << 6 | 0x3F,
            0xFF12 | 0xFF17 => self.volume_envelope.read(),
            0xFF14 | 0xFF19 => (self.length_timer.enabled() as u8) << 6 | 0xBF,
            _ => 0xFF,
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0xFF10 => match &mut self.sweep {
                Some(sweep) => sweep.write(value),
                None => {}
            },
            0xFF11 | 0xFF16 => self.write_timer_and_duty(value),
            0xFF12 | 0xFF17 => self.volume_envelope_write(value),
            0xFF13 | 0xFF18 => self.period = (self.period & 0x0700) | value as u16,
            0xFF14 | 0xFF19 => self.period_high_write(value),
            _ => {}
        }
    }
}

impl Channel for SquareChannel {
    fn cycle(&mut self) {
        if !self.enabled || !self.dac_enabled {
            return;
        }

        self.period_timer = self.period_timer.saturating_sub(T_CYCLES_PER_STEP as i32);
        if self.period_timer > 0 {
            return;
        }

        self.sample = if DUTY_TABLE[self.wave_index as usize][self.amplitude_index as usize] == 1 {
            self.volume_envelope.volume()
        } else {
            0
        };

        self.period_timer += ((2048 - self.period) * 4) as i32;
        self.amplitude_index = (self.amplitude_index + 1) % 8;
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

        self.period_timer = ((2048 - self.period) * 4) as i32;
        self.volume_envelope.reset_timer();

        if let Some(sweep) = &mut self.sweep {
            sweep.reset_timer();
        }

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
        self.amplitude_index = 0;
        self.period = 0;
        self.wave_index = 0;

        if let Some(sweep) = &mut self.sweep {
            sweep.reset();
        }
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn sample(&self) -> u8 {
        if self.enabled && self.dac_enabled { self.sample } else { 0 }
    }
}

impl SquareChannel {
    pub fn new(with_sweep: bool, div_apu_step: Rc<RefCell<u8>>) -> Self {
        let sweep = match with_sweep {
            true => Some(Sweep::new()),
            false => None,
        };
        Self {
            sample: 0,
            enabled: false,
            dac_enabled: false,
            trigger: false,
            period_timer: 0,
            length_timer: LengthTimer::new(),
            volume_envelope: VolumeEnvelope::new(),
            sweep,
            amplitude_index: 0,
            period: 0,
            wave_index: 0,
            div_apu_step,
        }
    }

    pub fn sweep_cycle(&mut self) {
        if let Some(sweep) = &mut self.sweep {
            sweep.cycle(&mut self.period, &mut self.enabled);
        }
    }

    fn write_timer_and_duty(&mut self, value: u8) {
        self.wave_index = value >> 6;
        self.length_timer.set_time(LENGTH_TIMER_MAX - (value & 0x3F) as u16);
    }

    fn volume_envelope_write(&mut self, value: u8) {
        self.volume_envelope.write(value);
        self.dac_enabled = value & 0xF8 != 0x00;
        if !self.dac_enabled {
            self.enabled = false;
        }
    }

    fn period_high_write(&mut self, value: u8) {
        self.period = (self.period & 0x00FF) | ((value & 0x07) as u16) << 8;

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
