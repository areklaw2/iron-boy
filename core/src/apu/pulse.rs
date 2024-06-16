use blip_buf::BlipBuf;

use crate::bus::Memory;

use super::{length_timer::LengthTimer, volume_envelope::VolumeEnvelope, ChannelMemory, WAVE_PATTERN};

const SWEEP_DELAY_ZERO_PACE: u8 = 8;

pub struct PulseChannel {
    active: bool,
    dac_enabled: bool,
    length: LengthTimer,
    volume_envelope: VolumeEnvelope,
    period: u32,
    frequency: u16,
    duty: u8,
    phase: u8,
    last_amplitude: i32,
    delay: u32,
    blip_buffer: BlipBuf,
    has_sweep: bool,
    sweep_enabled: bool,
    sweep_frequency: u16,
    sweep_delay: u8,
    sweep_pace: u8,
    sweep_individual_step: u8,
    sweep_direction: bool,
    sweep_is_increasing: bool,
}

impl ChannelMemory for PulseChannel {
    fn mem_read(&mut self, address: u16) -> u8 {
        match address {
            0xFF10 => {
                let mut data = 0x80;
                data |= (self.sweep_pace & 0x07) << 4;
                data |= (self.sweep_direction as u8) << 3;
                data |= self.sweep_individual_step & 0x07;
                data
            }
            0xFF11 | 0xFF16 => ((self.duty & 0b11) << 6) | 0x3F,
            0xFF12 | 0xFF17 => self.volume_envelope.mem_read(address),
            0xFF13 | 0xFF18 => 0xFF,
            0xFF14 | 0xFF19 => {
                let mut data = 0x80;
                data |= (self.length.enabled() as u8) << 6;
                data |= 0x3F;
                data
            }
            _ => 0xFF,
        }
    }

    fn mem_write(&mut self, address: u16, data: u8, frame_step: u8) {
        match address {
            0xFF10 => {
                self.sweep_pace = (data >> 4) & 0x07;
                self.sweep_individual_step = data & 0x07;

                let prev_sweep_direction = self.sweep_direction;
                self.sweep_direction = data & 0x8 == 0x8;
                if prev_sweep_direction && !self.sweep_direction && self.sweep_is_increasing {
                    self.active = false;
                }
                self.sweep_is_increasing = false;
            }
            0xFF11 | 0xFF16 => {
                self.duty = data >> 6;
                self.length.set(data & 0x3F)
            }
            0xFF12 | 0xFF17 => {
                self.dac_enabled = data & 0xF8 != 0;
                self.active = self.active && self.dac_enabled
            }
            0xFF13 | 0xFF18 => {
                self.frequency = (self.frequency & 0x0700) | (data as u16);
                self.calculate_period();
            }
            0xFF14 | 0xFF19 => {
                self.frequency = (((data & 0x07) as u16) << 8) | (self.frequency & 0x00FF);
                self.calculate_period();

                self.length.enable(data & 0x40 == 0x40, frame_step);
                self.active &= self.length.active();

                if data & 0x80 == 0x80 {
                    if self.dac_enabled {
                        self.active = true
                    }
                    self.length.trigger(frame_step);
                    if !self.has_sweep {
                        return;
                    }

                    self.sweep_frequency = self.frequency;
                    self.sweep_delay = if self.sweep_pace != 0 { self.sweep_pace } else { SWEEP_DELAY_ZERO_PACE };

                    self.sweep_enabled = self.sweep_pace > 0 || self.sweep_individual_step > 0;
                    if self.sweep_individual_step > 0 {
                        self.calculate_sweep_frequency();
                    }
                }
            }
            _ => {}
        }
        self.volume_envelope.mem_write(address, data);
    }
}

impl PulseChannel {
    pub fn new(blip_buffer: BlipBuf, with_sweep: bool) -> PulseChannel {
        PulseChannel {
            active: false,
            dac_enabled: false,
            length: LengthTimer::new(64),
            volume_envelope: VolumeEnvelope::new(),
            period: 2048,
            frequency: 0,
            duty: 1,
            phase: 1,
            last_amplitude: 0,
            delay: 0,
            blip_buffer,
            has_sweep: with_sweep,
            sweep_enabled: false,
            sweep_frequency: 0,
            sweep_delay: 0,
            sweep_pace: 0,
            sweep_individual_step: 0,
            sweep_direction: false,
            sweep_is_increasing: false,
        }
    }

    pub fn on(&self) -> bool {
        self.active
    }

    fn calculate_period(&mut self) {
        if self.frequency > 2047 {
            self.period = 0;
        } else {
            self.period = (2048 - self.frequency as u32) * 4
        }
    }

    pub fn step_length(&mut self) {
        self.length.step();
        self.active &= self.length.active();
    }

    fn run(&mut self, start_time: u32, end_time: u32) {
        if !self.active || self.period == 0 {
            if self.last_amplitude != 0 {
                self.blip_buffer.add_delta(start_time, -self.last_amplitude);
                self.last_amplitude = 0;
                self.delay = 0;
            }
        } else {
            let mut time = start_time + self.delay;
            let pattern = WAVE_PATTERN[self.duty as usize];
            let volume = self.volume_envelope.volume() as i32;

            while time < end_time {
                let amplitude = volume * pattern[self.phase as usize];
                if amplitude != self.last_amplitude {
                    self.blip_buffer.add_delta(time, amplitude - self.last_amplitude);
                    self.last_amplitude = amplitude;
                }
                time += self.period;
                self.phase = (self.phase + 1) % 8;
            }
            self.delay = time - end_time;
        }
    }

    fn calculate_sweep_frequency(&mut self) -> u16 {
        let offset = self.sweep_frequency >> self.sweep_individual_step;

        let frequency = if self.sweep_direction {
            self.sweep_is_increasing = true;
            self.sweep_frequency.wrapping_sub(offset)
        } else {
            self.sweep_frequency.wrapping_add(offset)
        };

        if frequency > 2047 {
            self.active = false;
        }
        return frequency;
    }

    pub fn step_sweep(&mut self) {
        if self.sweep_delay > 1 {
            self.sweep_delay -= 1;
            return;
        }

        if self.sweep_pace == 0 {
            self.sweep_delay = SWEEP_DELAY_ZERO_PACE;
        } else {
            self.sweep_delay = self.sweep_pace;
            if !self.sweep_enabled {
                return;
            }

            let frequency = self.calculate_sweep_frequency();
            if frequency > 2047 {
                return;
            }

            if self.sweep_individual_step != 0 {
                self.sweep_frequency = frequency;
                self.frequency = frequency;
                self.calculate_period();
            }
            self.calculate_sweep_frequency();
        }
    }
}
