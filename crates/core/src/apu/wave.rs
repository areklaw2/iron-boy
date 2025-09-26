use crate::{GbMode, T_CYCLES_PER_STEP, system_bus::SystemMemoryAccess};

use super::channel::{Channel, ChannelBase, length_timer::LengthTimer};

const LENGTH_TIMER_MAX: u16 = 256;

pub struct WaveChannel {
    base: ChannelBase,
    length_timer: LengthTimer,
    volume: u8,
    frequency: u16,
    wave_ram: [u8; 16],
    wave_ram_position: u8,
    can_access_wave_ram: bool,
}

impl SystemMemoryAccess for WaveChannel {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0xFF1A => (self.base.dac_enabled as u8) << 7 | 0x7F,
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
            0xFF1D => self.frequency = (self.frequency & 0x0700) | value as u16,
            0xFF1E => self.frequency_high_write(value),
            _ => {}
        }
    }
}

impl Channel for WaveChannel {
    fn cycle(&mut self) {
        self.can_access_wave_ram = false;
        if !self.base.enabled || !self.base.dac_enabled {
            return;
        }

        self.base.timer = self.base.timer.saturating_sub(T_CYCLES_PER_STEP as i32);
        if self.base.timer > 0 {
            return;
        }

        self.can_access_wave_ram = true;
        let wave_index = (self.wave_ram_position / 2) as usize;
        let output = match self.wave_ram_position % 2 == 0 {
            true => (self.wave_ram[wave_index] & 0xF0) >> 4,
            false => self.wave_ram[wave_index] & 0x0F,
        };

        self.base.sample = output >> self.volume_shift();

        self.base.timer += ((2048 - self.frequency) * 2) as i32;
        self.wave_ram_position = (self.wave_ram_position + 1) % 32;
    }

    fn length_timer_cycle(&mut self) {
        self.length_timer.cycle(&mut self.base.enabled)
    }

    fn volume_envelope_cycle(&mut self) {
        unimplemented!()
    }

    fn trigger(&mut self) {
        if self.base.dac_enabled {
            self.base.enabled = true;
        }

        self.base.timer = ((2048 - self.frequency) * 2) as i32;
        self.wave_ram_position = 0;

        if self.length_timer.time() == 0 {
            self.length_timer.set_time(LENGTH_TIMER_MAX);
        }
    }

    fn reset(&mut self) {
        self.base.reset();
        self.length_timer.reset();
        self.volume = 0;
        self.wave_ram_position = 0;
        self.frequency = 0;
        self.can_access_wave_ram = false;
    }

    fn enabled(&self) -> bool {
        self.base.enabled
    }

    fn sample(&self) -> u8 {
        self.base.sample()
    }
}

impl WaveChannel {
    pub fn new() -> Self {
        Self {
            base: ChannelBase::new(),
            length_timer: LengthTimer::new(),
            volume: 0,
            frequency: 0,
            wave_ram: [0; 16],
            wave_ram_position: 0,
            can_access_wave_ram: false,
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
        self.base.dac_enabled = value & 0x80 != 0;
        if !self.base.dac_enabled {
            self.base.enabled = false;
        }
    }

    fn frequency_high_write(&mut self, value: u8) {
        if value & 0x80 != 0 {
            self.trigger();
        }
        self.length_timer.set_enabled(value & 0x40 == 0x40);
        self.frequency = (self.frequency & 0x00FF) | ((value & 0x07) as u16) << 8;
    }

    pub fn read_wave_ram(&self, address: u16, mode: GbMode) -> u8 {
        let mut wave_index = (address & 0xF) as u8;
        if self.base.enabled {
            wave_index = self.wave_ram_position / 2;
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
        if self.base.enabled {
            wave_index = self.wave_ram_position / 2;
            if self.can_access_wave_ram || mode == GbMode::Color {
                self.wave_ram[wave_index as usize] = value;
            }
        } else {
            self.wave_ram[wave_index as usize] = value;
        }
    }
}
