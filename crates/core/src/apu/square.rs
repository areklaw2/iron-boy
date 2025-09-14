use crate::system_bus::SystemMemoryAccess;

use super::channel::{Channel, ChannelBase, length_timer::LengthTimer, sweep::Sweep, volume_envelope::VolumeEnvelope};

const DUTY_TABLE: [[u8; 8]; 4] = [
    [0, 0, 0, 0, 0, 0, 0, 1],
    [0, 0, 0, 0, 0, 0, 1, 1],
    [0, 0, 0, 0, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 0, 0],
];
const LENGTH_TIMER_MAX: u16 = 64;

pub struct SquareChannel {
    base: ChannelBase,
    length_timer: LengthTimer,
    volume_envelope: VolumeEnvelope,
    sweep: Option<Sweep>,
    sequence: u8,
    frequency: u16,
    wave_duty: u8,
}

impl SystemMemoryAccess for SquareChannel {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0xFF10 => match &self.sweep {
                Some(sweep) => sweep.read(),
                None => 0x00,
            },
            0xFF11 | 0xFF16 => self.read_timer_and_duty(),
            0xFF12 | 0xFF17 => self.volume_envelope.read(),
            0xFF13 | 0xFF18 => self.frequency as u8,
            0xFF14 | 0xFF19 => self.frequency_high_read(),
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
            0xFF13 | 0xFF18 => self.frequency = (self.frequency & 0x0700) | value as u16,
            0xFF14 | 0xFF19 => self.frequency_high_write(value),
            _ => {}
        }
    }
}

impl Channel for SquareChannel {
    fn cycle(&mut self, cycles: u32) {
        if !self.base.enabled || !self.base.dac_enabled {
            return;
        }

        let cycles = cycles as u16;

        self.base.timer = self.base.timer.saturating_sub(cycles as i32);
        if self.base.timer > 0 {
            return;
        }

        self.base.output = if DUTY_TABLE[self.wave_duty as usize][self.sequence as usize] == 1 {
            self.volume_envelope.volume()
        } else {
            0
        };

        self.base.timer += ((2048 - self.frequency) * 4) as i32;
        self.sequence = (self.sequence + 1) & 0x07;
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

        self.base.timer = ((2048 - self.frequency) * 4) as i32;
        self.volume_envelope.reset_timer();

        if let Some(sweep) = &mut self.sweep {
            sweep.reset_timer();
        }

        if self.length_timer.time() == 0 {
            self.length_timer.set_time(LENGTH_TIMER_MAX);
        }
    }

    fn reset(&mut self) {
        self.base.reset();
        self.length_timer.reset();
        self.volume_envelope.reset();
        self.sequence = 0;
        self.frequency = 0;
        self.wave_duty = 0;

        if let Some(sweep) = &mut self.sweep {
            sweep.reset();
        }
    }

    fn enabled(&self) -> bool {
        self.base.enabled
    }

    fn output(&self) -> u8 {
        self.base.output()
    }
}

impl SquareChannel {
    pub fn new(with_sweep: bool) -> Self {
        let sweep = match with_sweep {
            true => Some(Sweep::new()),
            false => None,
        };
        Self {
            base: ChannelBase::new(),
            length_timer: LengthTimer::new(),
            volume_envelope: VolumeEnvelope::new(),
            sweep,
            sequence: 0,
            frequency: 0,
            wave_duty: 0,
        }
    }

    pub fn sweep_cycle(&mut self) {
        if let Some(sweep) = &mut self.sweep {
            sweep.cycle(&mut self.frequency, &mut self.base.enabled);
        }
    }

    fn read_timer_and_duty(&self) -> u8 {
        let wave_duty = self.wave_duty << 6;
        let length_timer = (self.length_timer.time() & 0x3F) as u8;
        wave_duty | length_timer
    }

    fn write_timer_and_duty(&mut self, value: u8) {
        self.wave_duty = value >> 6;
        self.length_timer.set_time(LENGTH_TIMER_MAX - (value & 0x3F) as u16);
    }

    fn volume_envelope_write(&mut self, value: u8) {
        self.volume_envelope.write(value);
        self.base.dac_enabled = value & 0xF8 != 0x00;
        if !self.base.dac_enabled {
            self.base.enabled = false;
        }
    }

    fn frequency_high_read(&self) -> u8 {
        let triggered = (self.base.triggered as u8) << 7;
        let length_enabled = (self.length_timer.enabled() as u8) << 6;
        let frequency_high = ((self.frequency & 0x0700) >> 8) as u8;
        triggered | length_enabled | frequency_high
    }

    fn frequency_high_write(&mut self, value: u8) {
        let triggered = value & 0x80 == 0x80;
        if triggered {
            self.trigger();
        }
        self.length_timer.set_enabled(value & 0x40 == 0x40);
        self.frequency = (self.frequency & 0x00FF) | ((value & 0x07) as u16) << 8;
    }
}
