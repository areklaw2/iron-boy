use super::{length_timer::LengthTimer, Channel};
use blip_buf::BlipBuf;

const WAVE_INITIAL_DELAY: u32 = 4;

pub struct WaveChannel {
    active: bool,
    dac_enabled: bool,
    length: LengthTimer,
    volume_shift: u8,
    frequency: u16,
    period: u32,
    last_amplitude: i32,
    delay: u32,
    wave_ram: [u8; 16],
    current_wave: u8,
    sample_recently_accessed: bool,
    pub blip_buffer: BlipBuf,
}

impl Channel for WaveChannel {
    fn mem_read(&mut self, address: u16) -> u8 {
        match address {
            0xFF1A => {
                let mut data = 0;
                data |= (self.dac_enabled as u8) << 7;
                data |= 0x7F;
                data
            }
            0xFF1B => 0xFF,
            0xFF1C => {
                let mut data = 0x80;
                data |= (self.volume_shift & 0b11) << 5;
                data |= 0x1F;
                data
            }
            0xFF1D => 0xFF,
            0xFF1E => {
                let mut data = 0x80;
                data |= (self.length.enabled() as u8) << 6;
                data |= 0x3F;
                data
            }
            0xFF30..=0xFF3F => {
                if !self.active {
                    return self.wave_ram[address as usize - 0xFF30];
                }
                if self.sample_recently_accessed {
                    return self.wave_ram[self.current_wave as usize >> 1];
                }
                0xFF
            }
            _ => 0xFF,
        }
    }

    fn mem_write(&mut self, address: u16, data: u8, frame_step: u8) {
        match address {
            0xFF1A => {
                self.dac_enabled = data & 0x80 == 0x80;
                self.active = self.active && self.dac_enabled;
            }
            0xFF1B => self.length.set(data),
            0xFF1C => self.volume_shift = (data >> 5) & 0b11,
            0xFF1D => {
                self.frequency = (self.frequency & 0x0700) | (data as u16);
                self.calculate_period();
            }
            0xFF1E => {
                self.frequency = (((data & 0x07) as u16) << 8) | (self.frequency & 0x00FF);
                self.calculate_period();

                self.length.enable(data & 0x40 == 0x40, frame_step);
                self.active &= self.length.active();

                if data & 0x80 == 0x80 {
                    self.correct_wave_ram_corruption();

                    self.length.trigger(frame_step);

                    self.current_wave = 0;
                    self.delay = self.period + WAVE_INITIAL_DELAY;

                    if self.dac_enabled {
                        self.active = true;
                    }
                }
            }
            0xFF30..=0xFF3F => {
                if !self.active {
                    self.wave_ram[address as usize - 0xFF30] = data;
                    return;
                }
                if self.sample_recently_accessed {
                    self.wave_ram[self.current_wave as usize >> 1] = data;
                }
            }
            _ => {}
        }
    }

    fn on(&self) -> bool {
        self.active
    }

    fn calculate_period(&mut self) {
        if self.frequency > 2048 {
            self.period = 0;
        } else {
            self.period = (2048 - self.frequency as u32) * 2;
        }
    }

    fn step_length(&mut self) {
        self.length.step();
        self.active &= self.length.active();
    }

    fn run(&mut self, start_time: u32, end_time: u32) {
        self.sample_recently_accessed = false;
        if !self.active || self.period == 0 {
            if self.last_amplitude != 0 {
                self.blip_buffer.add_delta(start_time, -self.last_amplitude);
                self.last_amplitude = 0;
                self.delay = 0;
            }
        } else {
            let mut time = start_time + self.delay;

            let volume_shift = match self.volume_shift {
                1 => 0,
                2 => 1,
                3 => 2,
                _ => 4 + 2,
            };

            while time < end_time {
                let wave_byte = self.wave_ram[self.current_wave as usize >> 1];
                let sample = if self.current_wave % 2 == 0 { wave_byte >> 4 } else { wave_byte & 0xF };

                let amplitude = ((sample << 2) >> volume_shift) as i32;
                if amplitude != self.last_amplitude {
                    self.blip_buffer.add_delta(time, amplitude - self.last_amplitude);
                    self.last_amplitude = amplitude;
                }

                if time >= end_time - 2 {
                    self.sample_recently_accessed = true;
                }
                time += self.period;
                self.current_wave = (self.current_wave + 1) % 32;
            }

            self.delay = time - end_time;
        }
    }
}

impl WaveChannel {
    pub fn new(blip_buffer: BlipBuf) -> WaveChannel {
        WaveChannel {
            active: false,
            dac_enabled: false,
            length: LengthTimer::new(256),
            volume_shift: 0,
            frequency: 0,
            period: 2048,
            last_amplitude: 0,
            delay: 0,
            wave_ram: [0; 16],
            current_wave: 0,
            sample_recently_accessed: false,
            blip_buffer,
        }
    }

    fn correct_wave_ram_corruption(&mut self) {
        if !self.active || self.delay != 0 {
            return;
        }

        let byteindex = ((self.current_wave + 1) % 32) as usize >> 1;
        if byteindex < 4 {
            self.wave_ram[0] = self.wave_ram[byteindex];
        } else {
            let blockstart = byteindex & 0b1100;
            self.wave_ram[0] = self.wave_ram[blockstart];
            self.wave_ram[1] = self.wave_ram[blockstart + 1];
            self.wave_ram[2] = self.wave_ram[blockstart + 2];
            self.wave_ram[3] = self.wave_ram[blockstart + 3];
        }
    }
}
