use std::cell::RefCell;
use std::rc::Rc;

use crate::apu::length_timer::{Length, WAVE_MAX_LENGTH};
use crate::apu::period::Period;
use crate::system_bus::SystemMemoryAccess;
use crate::{GbMode, T_CYCLES_PER_STEP};
use getset::{CopyGetters, Setters};

#[derive(Debug, CopyGetters, Setters)]
pub struct WaveChannel {
    #[getset(get_copy = "pub", set = "pub")]
    enabled: bool,
    dac_enabled: bool,
    length: Length,
    volume: u8,
    period: Period,
    wave_position: u8,
    wave_ram: [u8; 0x10],
    gb_mode: GbMode,
    div_apu_step: Rc<RefCell<u8>>,
}

impl SystemMemoryAccess for WaveChannel {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0xFF1A => match self.dac_enabled {
                true => 0xFF,
                false => 0x7F,
            },
            0xFF1C => self.volume | 0x9F,
            0xFF1E => self.period.high() | 0xBF,
            0xFF30..=0xFF3F => self.read_wave_ram(address),
            _ => 0xFF,
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0xFF1A => self.write_dac_enabled(value),
            0xFF1B => self.write_length_timer(value),
            0xFF1C => self.write_volume(value),
            0xFF1D => self.write_period_low(value),
            0xFF1E => self.write_period_high(value),
            0xFF30..=0xFF3F => self.write_wave_ram(address, value),
            _ => {}
        }
    }
}

impl WaveChannel {
    pub fn new(gb_mode: GbMode, div_apu_step: Rc<RefCell<u8>>) -> Self {
        WaveChannel {
            enabled: false,
            dac_enabled: false,
            length: Length::new(WAVE_MAX_LENGTH),
            volume: 0,
            period: Period::new(),
            wave_position: 1,
            wave_ram: [0; 0x10],
            gb_mode,
            div_apu_step,
        }
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.dac_enabled = false;
        self.volume = 0;
        self.period = Period::new();

        if self.gb_mode != GbMode::Color {
            self.length.reset();
        } else {
            self.length = Length::new(WAVE_MAX_LENGTH);
        }
    }

    pub fn cycle(&mut self) {
        if self.enabled {
            self.period.cycle(T_CYCLES_PER_STEP / 2, || {
                self.wave_position = (self.wave_position + 1) % 32;
            });
        }
    }

    pub fn cycle_length_on_enable(&self, value: u8) -> bool {
        (value & (1 << 6)) == 0 && (self.period.high() & (1 << 6)) != 0
    }

    pub fn cycle_clock_length_on_trigger(&self) -> bool {
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
            let localized_address = self.wave_position / 2;
            let byte_offset = self.wave_position % 2;

            let byte = self.wave_ram[localized_address as usize];
            let sample = if byte_offset == 0 { (byte & 0xF0) >> 4 } else { byte & 0xF };

            let output_level = (self.volume & 0b01100000) >> 5;
            match output_level {
                0b01 => sample as f32,
                0b10 => (sample >> 1) as f32,
                0b11 => (sample >> 2) as f32,
                _ => 7.5,
            }
        } else {
            7.5
        }
    }

    pub fn trigger(&mut self) {
        let period_timer = self.period.timer();
        if self.enabled && period_timer == 1 && self.gb_mode != GbMode::Color {
            self.wave_ram_bug();
        }

        self.wave_position = 0;

        if self.dac_enabled {
            self.enabled = true;
        }

        self.period.trigger();
        self.period.wave_channel_trigger_delay();
        self.length.reload();
    }

    pub fn should_trigger(&self) -> bool {
        (self.period.high() & (1 << 7)) != 0
    }

    fn write_dac_enabled(&mut self, value: u8) {
        self.dac_enabled = (value & 0x80) != 0;
        if !self.dac_enabled {
            self.enabled = false;
        }
    }

    fn write_length_timer(&mut self, value: u8) {
        self.length.set_initial_time(value);
        self.length.initialize();
    }

    fn write_volume(&mut self, value: u8) {
        self.volume = value;
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

            if self.cycle_clock_length_on_trigger() && period_in_first_half {
                self.cycle_length();
            }
        }
    }

    fn read_wave_ram(&self, address: u16) -> u8 {
        let mut wave_index = (address & 0xF) as u8;
        if self.enabled {
            wave_index = self.wave_position / 2;
            match self.period.reloaded() || self.gb_mode == GbMode::Color {
                true => self.wave_ram[wave_index as usize],
                false => 0xFF,
            }
        } else {
            self.wave_ram[wave_index as usize]
        }
    }

    fn write_wave_ram(&mut self, address: u16, value: u8) {
        let mut wave_index = (address & 0xF) as u8;
        if self.enabled {
            wave_index = self.wave_position / 2;
            if self.period.reloaded() || self.gb_mode == GbMode::Color {
                self.wave_ram[wave_index as usize] = value;
            }
        } else {
            self.wave_ram[wave_index as usize] = value;
        }
    }

    fn wave_ram_bug(&mut self) {
        let wave_position = (((self.wave_position + 1) / 2) % 16) as usize;
        if wave_position < 4 {
            self.wave_ram[0] = self.wave_ram[wave_position];
        } else {
            let position = wave_position & !0b11;
            for i in 0..=3 {
                self.wave_ram[i] = self.wave_ram[position + i];
            }
        }
    }
}
