use std::{cell::RefCell, rc::Rc};

use crate::{GbMode, T_CYCLES_PER_STEP, system_bus::SystemMemoryAccess};

use super::{Channel, length_timer::LengthTimer};

const LENGTH_TIMER_MAX: u16 = 256;

pub struct WaveChannel {
    sample: u8,
    enabled: bool,
    dac_enabled: bool,
    trigger: bool,
    period_timer: i32,
    length_timer: LengthTimer,
    volume: u8,
    period: u16,
    wave_ram: [u8; 16],
    wave_ram_nibble: u8,
    can_access_wave_ram: bool,
    div_apu_step: Rc<RefCell<u8>>,
}

impl SystemMemoryAccess for WaveChannel {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0xFF1A => (self.dac_enabled as u8) << 7 | 0x7F,
            0xFF1C => (self.volume & 0x03) << 5 | 0x9F,
            0xFF1E => (self.length_timer.enabled() as u8) << 6 | 0xBF,
            _ => 0xFF,
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0xFF1A => self.dac_enable_write(value),
            0xFF1B => self.length_timer.set_time(LENGTH_TIMER_MAX - (value as u16)),
            0xFF1C => self.volume = (value & 0x60) >> 5,
            0xFF1D => self.period = (self.period & 0x0700) | value as u16,
            0xFF1E => self.period_high_write(value),
            _ => {}
        }
    }
}

impl Channel for WaveChannel {
    fn cycle(&mut self) {
        self.can_access_wave_ram = false;
        if !self.enabled || !self.dac_enabled {
            return;
        }

        self.period_timer = self.period_timer.saturating_sub(T_CYCLES_PER_STEP as i32);
        if self.period_timer > 0 {
            return;
        }

        self.can_access_wave_ram = true;
        let wave_index = (self.wave_ram_nibble / 2) as usize;
        let output = match self.wave_ram_nibble % 2 == 0 {
            true => (self.wave_ram[wave_index] & 0xF0) >> 4,
            false => self.wave_ram[wave_index] & 0x0F,
        };

        self.sample = output >> self.volume_shift();

        self.period_timer += ((2048 - self.period) * 2) as i32;
        self.wave_ram_nibble = (self.wave_ram_nibble + 1) % 32;
    }

    fn length_timer_cycle(&mut self) {
        self.length_timer.cycle(&mut self.enabled)
    }

    fn volume_envelope_cycle(&mut self) {
        unimplemented!()
    }

    fn trigger(&mut self) {
        if self.dac_enabled {
            self.enabled = true;
        }

        self.period_timer = ((2048 - self.period) * 2) as i32;
        self.wave_ram_nibble = 0;

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
        self.volume = 0;
        self.wave_ram_nibble = 0;
        self.period = 0;
        self.can_access_wave_ram = false;
    }

    fn enabled(&self) -> bool {
        self.enabled
    }

    fn sample(&self) -> u8 {
        if self.enabled && self.dac_enabled { self.sample } else { 0 }
    }
}

impl WaveChannel {
    pub fn new(div_apu_step: Rc<RefCell<u8>>) -> Self {
        Self {
            sample: 0,
            enabled: false,
            dac_enabled: false,
            trigger: false,
            period_timer: 0,
            length_timer: LengthTimer::new(),
            volume: 0,
            period: 0,
            wave_ram: [0; 16],
            wave_ram_nibble: 0,
            can_access_wave_ram: false,
            div_apu_step,
        }
    }

    fn volume_shift(&self) -> u8 {
        match self.volume {
            0x01 => 0,
            0x02 => 1,
            0x03 => 2,
            _ => 4,
        }
    }

    fn dac_enable_write(&mut self, value: u8) {
        self.dac_enabled = value & 0x80 != 0;
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

    pub fn read_wave_ram(&self, address: u16, mode: GbMode) -> u8 {
        let mut wave_index = (address & 0xF) as u8;
        if self.enabled {
            wave_index = self.wave_ram_nibble / 2;
            match self.can_access_wave_ram || mode == GbMode::Color {
                true => self.wave_ram[wave_index as usize],
                false => 0xFF,
            }
        } else {
            self.wave_ram[wave_index as usize]
        }
    }

    pub fn write_wave_ram(&mut self, address: u16, value: u8, mode: GbMode) {
        let mut wave_index = (address & 0xF) as u8;
        if self.enabled {
            wave_index = self.wave_ram_nibble / 2;
            if self.can_access_wave_ram || mode == GbMode::Color {
                self.wave_ram[wave_index as usize] = value;
            }
        } else {
            self.wave_ram[wave_index as usize] = value;
        }
    }
}
