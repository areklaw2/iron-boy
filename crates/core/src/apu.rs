use channel::Channel;
use div_apu::DivApu;
use getset::{Getters, MutGetters, Setters};
use noise::NoiseChannel;
use square::SquareChannel;
use wave::WaveChannel;

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crate::{GbMode, GbSpeed, T_CYCLES_PER_STEP, apu::div_apu::DivApuContext, cpu::CPU_CLOCK_SPEED, system_bus::SystemMemoryAccess};

mod channel;
mod div_apu;
mod noise;
mod square;
mod wave;

pub const SAMPLING_FREQUENCY: u16 = 44100;
pub const AUDIO_BUFFER_THRESHOLD: usize = SAMPLING_FREQUENCY as usize * 150 / 1000;
pub const CPU_CYCLES_PER_SAMPLE: f32 = CPU_CLOCK_SPEED as f32 / SAMPLING_FREQUENCY as f32;

#[derive(Getters, MutGetters, Setters)]
pub struct Apu {
    ch1: SquareChannel,
    ch2: SquareChannel,
    ch3: WaveChannel,
    ch4: NoiseChannel,
    div_apu: DivApu,
    sound_panning: u8,
    master_volume: u8,
    enabled: bool,
    counter: f32,
    #[getset(get_mut = "pub")]
    audio_buffer: Arc<Mutex<VecDeque<f32>>>,
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
            0xFF30..=0xFF3F => self.ch3.read_wave_ram(address, self.gb_mode),
            0xFF76 => self.ch2.sample() << 4 | self.ch1.sample(),
            0xFF77 => self.ch4.sample() << 4 | self.ch3.sample(),
            _ => 0xFF,
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        if !self.enabled && !matches!(address, 0xFF26 | 0xFF30..=0xFF3F) {
            return;
        }

        match address {
            0xFF10..=0xFF14 => self.ch1.write_8(address, value),
            0xFF16..=0xFF19 => self.ch2.write_8(address, value),
            0xFF1A..=0xFF1E => self.ch3.write_8(address, value),
            0xFF20..=0xFF23 => self.ch4.write_8(address, value),
            0xFF24 => self.set_master_volume(value),
            0xFF25 => self.set_sound_panning(value),
            0xFF26 => self.set_master_control(value),
            0xFF30..=0xFF3F => self.ch3.write_wave_ram(address, value, self.gb_mode),
            0xFF76..=0xFF77 => {}
            _ => {}
        }
    }
}

impl Apu {
    pub fn new(gb_mode: GbMode) -> Self {
        Self {
            ch1: SquareChannel::new(true),
            ch2: SquareChannel::new(false),
            ch3: WaveChannel::new(),
            ch4: NoiseChannel::new(),
            div_apu: DivApu::new(),
            sound_panning: 0,
            master_volume: 0,
            enabled: false,
            counter: 0.0,
            audio_buffer: Arc::new(Mutex::new(VecDeque::new())),
            gb_mode,
        }
    }

    pub fn cycle(&mut self, div: u8, speed: GbSpeed) {
        if !self.enabled {
            return;
        }

        self.div_apu.cycle(DivApuContext {
            ch1: &mut self.ch1,
            ch2: &mut self.ch2,
            ch3: &mut self.ch3,
            ch4: &mut self.ch4,
            div,
            speed,
        });

        self.ch1.cycle();
        self.ch2.cycle();
        self.ch3.cycle();
        self.ch4.cycle();

        self.counter += T_CYCLES_PER_STEP as f32;
        if self.counter >= CPU_CYCLES_PER_SAMPLE {
            let (output_left, output_right) = self.mix();

            let mut buffer = self.audio_buffer.lock().unwrap();
            if buffer.len() < AUDIO_BUFFER_THRESHOLD {
                buffer.push_back(output_left);
                buffer.push_back(output_right);
            }
            drop(buffer);

            self.counter -= CPU_CYCLES_PER_SAMPLE;
        }
    }

    fn master_control(&self) -> u8 {
        (self.enabled as u8) << 7
            | 0x70
            | (self.ch4.enabled() as u8) << 3
            | (self.ch3.enabled() as u8) << 2
            | (self.ch2.enabled() as u8) << 1
            | self.ch1.enabled() as u8
    }

    fn set_master_control(&mut self, value: u8) {
        self.enabled = value & 0x80 != 0;
        if !self.enabled {
            self.reset();
        }
    }

    fn set_master_volume(&mut self, value: u8) {
        if !self.enabled {
            return;
        }
        self.master_volume = value
    }

    fn set_sound_panning(&mut self, value: u8) {
        if !self.enabled {
            return;
        }
        self.sound_panning = value
    }

    fn mix(&self) -> (f32, f32) {
        let mut sample_left = 0.0;
        let mut sample_right = 0.0;
        let channel_samples = [self.ch4.sample(), self.ch3.sample(), self.ch2.sample(), self.ch1.sample()];
        for (i, sample) in channel_samples.iter().enumerate() {
            if self.sound_panning & (1 << (7 - i)) != 0 {
                sample_left += *sample as f32;
            }
            if self.sound_panning & (1 << (3 - i)) != 0 {
                sample_right += *sample as f32;
            }
        }

        let left_volume = self.master_volume & 0x70 >> 4;
        let right_volume = self.master_volume & 0x07;

        sample_left *= (left_volume + 1) as f32 / 64.0;
        sample_right *= (right_volume + 1) as f32 / 64.0;

        (sample_left, sample_right)
    }

    fn reset(&mut self) {
        self.ch1.reset();
        self.ch2.reset();
        self.ch3.reset();
        self.ch4.reset();
        self.div_apu.reset();
        self.sound_panning = 0;
        self.master_volume = 0;
        self.counter = 0.0;
        self.audio_buffer.lock().unwrap().clear();
    }
}
