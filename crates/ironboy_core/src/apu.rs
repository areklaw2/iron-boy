use std::cell::RefCell;
use std::rc::Rc;

use crate::apu::noise::NoiseChannel;
use crate::apu::pulse::PulseChannel;
use crate::apu::wave::WaveChannel;
use crate::cpu::CPU_CLOCK_SPEED;
use crate::system_bus::SystemMemoryAccess;
use crate::{GbMode, GbSpeed, T_CYCLES_PER_STEP};
use getset::{Getters, Setters};

mod length_timer;
mod noise;
mod period;
mod pulse;
mod sweep;
mod volume_envelope;
mod wave;

pub const SAMPLING_FREQUENCY: u32 = 44100;
pub const SAMPLES_PER_FRAME: usize = 1024;
const AUDIO_BUFFER_SIZE: usize = SAMPLES_PER_FRAME / 2;
const CHANNEL_STEP_RATE: u8 = 4;
const CYCLES_PER_SAMPLE: u32 = CPU_CLOCK_SPEED / SAMPLING_FREQUENCY;

#[derive(Debug, Getters, Setters)]
pub struct Apu {
    ch1: PulseChannel,
    ch2: PulseChannel,
    ch3: WaveChannel,
    ch4: NoiseChannel,
    div_apu_step: Rc<RefCell<u8>>,
    previous_divider: u8,
    sound_panning: u8,
    master_volume: u8,
    enabled: bool,
    sample_cycles: u8,
    channel_clock: u8,
    ch1_sample_sum: f32,
    ch2_sample_sum: f32,
    ch3_sample_sum: f32,
    ch4_sample_sum: f32,
    #[getset(get = "pub")]
    left_audio_buffer: Vec<f32>,
    #[getset(get = "pub")]
    right_audio_buffer: Vec<f32>,
    #[getset(set = "pub")]
    speed: GbSpeed,
    gb_mode: GbMode,
}

impl SystemMemoryAccess for Apu {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0xFF10..=0xFF14 => self.ch1.read_8(address),
            0xFF16..=0xFF19 => self.ch2.read_8(address),
            0xFF1A..=0xFF1E => self.ch3.read_8(address),
            0xFF20..=0xFF23 => self.ch4.read_8(address),
            0xFF24 => self.master_volume,
            0xFF25 => self.sound_panning,
            0xFF26 => self.master_control(),
            0xFF30..=0xFF3F => self.ch3.read_8(address),
            _ => 0xFF,
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0xFF26 => return self.set_master_control(value),
            0xFF30..=0xFF3F => return self.ch3.write_8(address, value),

            0xFF11 | 0xFF16 if self.enabled || self.gb_mode != GbMode::Color => {
                let masked_value = if self.enabled { value } else { value & 0x3F };
                return match address {
                    0xFF11 => self.ch1.write_8(address, masked_value),
                    0xFF16 => self.ch2.write_8(address, masked_value),
                    _ => unreachable!(),
                };
            }

            0xFF1B | 0xFF20 if self.enabled || self.gb_mode != GbMode::Color => {
                return match address {
                    0xFF1B => self.ch3.write_8(address, value),
                    0xFF20 => self.ch4.write_8(address, value),
                    _ => unreachable!(),
                };
            }

            _ if !self.enabled => return,

            0xFF10 | 0xFF12..=0xFF14 => self.ch1.write_8(address, value),
            0xFF17..=0xFF19 => self.ch2.write_8(address, value),
            0xFF1A | 0xFF1C..=0xFF1E => self.ch3.write_8(address, value),
            0xFF21..=0xFF23 => self.ch4.write_8(address, value),
            0xFF24 => self.master_volume = value,
            0xFF25 => self.sound_panning = value,
            _ => {}
        }
    }
}

impl Apu {
    pub fn new(gb_mode: GbMode) -> Self {
        let div_apu_step = Rc::new(RefCell::new(0));
        Apu {
            ch1: PulseChannel::new(true, gb_mode, div_apu_step.clone()),
            ch2: PulseChannel::new(false, gb_mode, div_apu_step.clone()),
            ch3: WaveChannel::new(gb_mode, div_apu_step.clone()),
            ch4: NoiseChannel::new(gb_mode, div_apu_step.clone()),
            div_apu_step,
            previous_divider: 0,
            enabled: false,
            sound_panning: 0,
            master_volume: 0,
            sample_cycles: 0,
            channel_clock: 0,
            ch1_sample_sum: 0.0,
            ch2_sample_sum: 0.0,
            ch3_sample_sum: 0.0,
            ch4_sample_sum: 0.0,
            left_audio_buffer: Vec::new(),
            right_audio_buffer: Vec::new(),
            speed: GbSpeed::Normal,
            gb_mode,
        }
    }

    pub fn audio_buffers_full(&self) -> bool {
        self.left_audio_buffer.len() >= AUDIO_BUFFER_SIZE && self.right_audio_buffer.len() >= AUDIO_BUFFER_SIZE
    }

    pub fn clear_audio_buffers(&mut self) {
        self.left_audio_buffer.clear();
        self.right_audio_buffer.clear();
    }

    pub fn cycle(&mut self, divider: u8) {
        self.channel_clock = self.channel_clock.wrapping_add(T_CYCLES_PER_STEP);
        if self.enabled {
            if self.channel_clock >= CHANNEL_STEP_RATE {
                self.channel_clock = 0;

                self.ch1.cycle();
                self.ch2.cycle();
                self.ch3.cycle();
                self.ch4.cycle();
            }

            self.cycle_div_apu(divider);
        }
        self.sample_channels();

        self.previous_divider = divider;
    }

    fn cycle_div_apu(&mut self, divider: u8) {
        let bit_index = match self.speed {
            GbSpeed::Double => 5,
            GbSpeed::Normal => 4,
        };
        let cycle_div_apu = (self.previous_divider & (1 << bit_index)) != 0 && (divider & (1 << bit_index)) == 0;

        if cycle_div_apu {
            let div_apu_step = *self.div_apu_step.borrow();

            if div_apu_step == 7 {
                self.ch1.cycle_envelope();
                self.ch2.cycle_envelope();
                self.ch4.cycle_envelope();
            }

            if matches!(div_apu_step, 0 | 2 | 4 | 6) {
                self.ch1.cycle_length();
                self.ch2.cycle_length();
                self.ch3.cycle_length();
                self.ch4.cycle_length();
            }

            if matches!(div_apu_step, 2 | 6) {
                self.ch1.cycle_sweep();
            }

            *self.div_apu_step.borrow_mut() = (div_apu_step + 1) % 8;
        }
    }

    fn mix_left(&mut self, ch1_sample: f32, ch2_sample: f32, ch3_sample: f32, ch4_sample: f32) {
        let left_volume = (self.master_volume & 0x70) >> 4;
        let mut left_sample = 0.0;
        let channel_samples = [ch4_sample, ch3_sample, ch2_sample, ch1_sample];

        for (i, sample) in channel_samples.iter().enumerate() {
            if self.sound_panning & (1 << (7 - i)) != 0 {
                left_sample += sample
            }
        }

        let volume_reduction = (left_volume as f32 + 1.0) / 32.0;
        left_sample *= volume_reduction;
        self.left_audio_buffer.push(left_sample);
    }

    fn mix_right(&mut self, ch1_sample: f32, ch2_sample: f32, ch3_sample: f32, ch4_sample: f32) {
        let right_volume = self.master_volume & 0x07;
        let mut right_sample = 0.0;
        let channel_samples = [ch4_sample, ch3_sample, ch2_sample, ch1_sample];

        for (i, sample) in channel_samples.iter().enumerate() {
            if self.sound_panning & (1 << (3 - i)) != 0 {
                right_sample += sample
            }
        }

        let volume_reduction = (right_volume as f32 + 1.0) / 32.0;
        right_sample *= volume_reduction;

        self.right_audio_buffer.push(right_sample);
    }

    fn sample_channels(&mut self) {
        self.sample_cycles = self.sample_cycles.wrapping_add(T_CYCLES_PER_STEP);
        let m_cycles = self.sample_cycles / T_CYCLES_PER_STEP;

        let weight = (((CYCLES_PER_SAMPLE as u8 - m_cycles) as f32).ln() + 1.0) / ((CYCLES_PER_SAMPLE as f32).ln() + 1.0);
        self.ch1_sample_sum += self.ch1.digital_output() * weight;
        self.ch2_sample_sum += self.ch2.digital_output() * weight;
        self.ch3_sample_sum += self.ch3.digital_output() * weight;
        self.ch4_sample_sum += self.ch4.digital_output() * weight;

        if self.sample_cycles as u32 >= CYCLES_PER_SAMPLE {
            self.sample_cycles = 0;

            let ch1_sample = convert_digital_to_analog(self.ch1_sample_sum, m_cycles);
            let ch2_sample = convert_digital_to_analog(self.ch2_sample_sum, m_cycles);
            let ch3_sample = convert_digital_to_analog(self.ch3_sample_sum, m_cycles);
            let ch4_sample = convert_digital_to_analog(self.ch4_sample_sum, m_cycles);

            self.mix_left(ch1_sample, ch2_sample, ch3_sample, ch4_sample);
            self.mix_right(ch1_sample, ch2_sample, ch3_sample, ch4_sample);

            self.clear_samples();
        }
    }

    pub fn clear_samples(&mut self) {
        self.ch1_sample_sum = 0.0;
        self.ch2_sample_sum = 0.0;
        self.ch3_sample_sum = 0.0;
        self.ch4_sample_sum = 0.0;
    }

    pub fn master_control(&self) -> u8 {
        (self.enabled as u8) << 7
            | 0x70
            | (self.ch4.enabled() as u8) << 3
            | (self.ch3.enabled() as u8) << 2
            | (self.ch2.enabled() as u8) << 1
            | self.ch1.enabled() as u8
    }

    fn set_master_control(&mut self, value: u8) {
        self.enabled = (value & 0x80) != 0;
        if !self.enabled {
            self.reset();
        }
    }

    pub fn reset(&mut self) {
        self.enabled = false;
        self.sound_panning = 0;
        self.master_volume = 0;
        *self.div_apu_step.borrow_mut() = 0;
        self.previous_divider = 0;
        self.ch1.reset();
        self.ch2.reset();
        self.ch3.reset();
        self.ch4.reset();
    }
}

fn convert_digital_to_analog(sample_sum: f32, cycles: u8) -> f32 {
    let sample_average = sample_sum / cycles as f32;
    (sample_average / 7.5) - 1.0
}
