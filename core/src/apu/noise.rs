use std::env::consts::FAMILY;

use blip_buf::BlipBuf;

use crate::bus::Memory;

use super::{length_timer::LengthTimer, volume_envelope::VolumeEnvelope, Channel};

pub struct NoiseChannel {
    active: bool,
    dac_enabled: bool,
    length: LengthTimer,
    pub volume_envelope: VolumeEnvelope,
    frequency_and_randomness: u8,
    period: u32,
    lfsr_width: u8,
    state: u16,
    delay: u32,
    last_amplitude: i32,
    pub blip_buffer: BlipBuf,
}

impl Channel for NoiseChannel {
    fn mem_read(&mut self, address: u16) -> u8 {
        match address {
            0xFF20 => 0xFF,
            0xFF21 => self.volume_envelope.mem_read(address),
            0xFF22 => self.frequency_and_randomness,
            0xFF23 => {
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
            0xFF20 => self.length.set(data & 0x3F),
            0xFF21 => {
                self.dac_enabled = data & 0xF8 != 0;
                self.active = self.active && self.dac_enabled;
            }
            0xFF22 => {
                self.frequency_and_randomness = data;
                self.lfsr_width = match data & 0x08 == 0x8 {
                    true => 6,
                    false => 14,
                };
                let clock_divider = match data & 0x07 {
                    0 => 8,
                    n => n as u32 * 16,
                };
                self.period = clock_divider << (data >> 4);
            }
            0xFF23 => {
                self.length.enable(data & 0x40 == 0x40, frame_step);
                self.active &= self.length.active();

                if data & 0x80 == 0x80 {
                    self.length.trigger(frame_step);
                    self.state = 0xFF;
                    self.delay = 0;
                    if self.dac_enabled {
                        self.active = true;
                    }
                }
            }
            _ => {}
        }
        self.volume_envelope.mem_write(address, data);
    }

    fn on(&self) -> bool {
        self.active
    }

    fn calculate_period(&mut self) {}

    fn step_length(&mut self) {
        self.length.step();
        self.active &= self.length.active();
    }

    fn run(&mut self, start_time: u32, end_time: u32) {
        if !self.active {
            if self.last_amplitude != 0 {
                self.blip_buffer.add_delta(start_time, -self.last_amplitude);
                self.last_amplitude = 0;
                self.delay = 0;
            }
        } else {
            let mut time = start_time + self.delay;
            while time < end_time {
                let curr_state = self.state;
                self.state <<= 1;
                let bit = ((curr_state >> self.lfsr_width) ^ (self.state >> self.lfsr_width)) & 0x1;
                self.state |= bit;

                let amplitude = match (curr_state >> self.lfsr_width) & 0x1 {
                    0 => -(self.volume_envelope.volume as i32),
                    _ => self.volume_envelope.volume as i32,
                };

                if self.last_amplitude != amplitude {
                    self.blip_buffer.add_delta(time, amplitude - self.last_amplitude);
                    self.last_amplitude = amplitude;
                }
                time += self.period;
            }
            self.delay = time - end_time;
        }
    }
}

impl NoiseChannel {
    pub fn new(blip_buffer: BlipBuf) -> NoiseChannel {
        NoiseChannel {
            active: false,
            dac_enabled: false,
            length: LengthTimer::new(64),
            volume_envelope: VolumeEnvelope::new(),
            frequency_and_randomness: 0,
            period: 2048,
            lfsr_width: 14,
            state: 1,
            delay: 0,
            last_amplitude: 0,
            blip_buffer,
        }
    }
}
