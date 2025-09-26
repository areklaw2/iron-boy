use channel::Channel;
use frame_sequencer::FrameSequencer;
use getset::{Getters, MutGetters, Setters};
use noise::NoiseChannel;
use square::SquareChannel;
use wave::WaveChannel;

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crate::{T_CYCLES_PER_STEP, cpu::CPU_CLOCK_SPEED, system_bus::SystemMemoryAccess};

mod channel;
mod frame_sequencer;
mod noise;
mod square;
mod wave;

pub const SAMPLING_RATE: u16 = 1024;
pub const SAMPLING_FREQUENCY: u16 = 44100;
pub const APU_CLOCK_SPEED: u16 = 512;
pub const AUDIO_BUFFER_THRESHOLD: usize = SAMPLING_RATE as usize * 4;
pub const CPU_CYCLES_PER_SAMPLE: f32 = CPU_CLOCK_SPEED as f32 / SAMPLING_FREQUENCY as f32;

#[derive(Getters, MutGetters, Setters)]
pub struct Apu {
    ch1: SquareChannel,
    ch2: SquareChannel,
    ch3: WaveChannel,
    ch4: NoiseChannel,
    frame_sequencer: FrameSequencer,
    sound_panning: u8,
    #[getset(get = "pub")]
    pub right_volume: u8,
    #[getset(get = "pub")]
    pub left_volume: u8,
    enabled: bool,
    counter: f32,
    #[getset(get_mut = "pub")]
    pub audio_buffer: Arc<Mutex<VecDeque<u8>>>,
}

impl SystemMemoryAccess for Apu {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0xFF10..=0xFF14 => self.ch1.read_8(address),
            0xFF16..=0xFF19 => self.ch2.read_8(address),
            0xFF1A..=0xFF1E => self.ch3.read_8(address),
            0xFF20..=0xFF23 => self.ch4.read_8(address),
            0xFF24 => self.master_volume(),
            0xFF25 => self.sound_panning,
            0xFF26 => self.master_control(),
            0xFF30..=0xFF3F => self.ch3.read_8(address),
            0xFF76 => self.ch2.output() << 4 | self.ch1.output(),
            0xFF77 => self.ch4.output() << 4 | self.ch3.output(),
            _ => 0xFF,
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        if address == 0xFF26 {
            self.set_master_control(value);
            return;
        }

        if !self.enabled {
            return;
        }

        match address {
            0xFF10..=0xFF14 => self.ch1.write_8(address, value),
            0xFF16..=0xFF19 => self.ch2.write_8(address, value),
            0xFF1A..=0xFF1E => self.ch3.write_8(address, value),
            0xFF20..=0xFF23 => self.ch4.write_8(address, value),
            0xFF24 => self.set_master_volume(value),
            0xFF25 => self.set_sound_panning(value),
            0xFF26 => {}
            0xFF30..=0xFF3F => self.ch3.write_8(address, value),
            0xFF76..=0xFF77 => {}
            _ => {}
        }
    }
}

impl Apu {
    pub fn new() -> Self {
        Self {
            ch1: SquareChannel::new(true),
            ch2: SquareChannel::new(false),
            ch3: WaveChannel::new(),
            ch4: NoiseChannel::new(),
            frame_sequencer: FrameSequencer::new(),
            sound_panning: 0,
            right_volume: 0,
            left_volume: 0,
            enabled: false,
            counter: 0.0,
            audio_buffer: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn cycle(&mut self) {
        let cycles = T_CYCLES_PER_STEP as u32;
        self.frame_sequencer
            .cycle(cycles, &mut self.ch1, &mut self.ch2, &mut self.ch3, &mut self.ch4);

        if !self.enabled {
            return;
        }
        self.ch1.cycle(cycles);
        self.ch2.cycle(cycles);
        self.ch3.cycle(cycles);
        self.ch4.cycle(cycles);
        self.counter += cycles as f32;

        while self.counter >= CPU_CYCLES_PER_SAMPLE {
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
            | (self.ch4.enabled() as u8) << 3
            | (self.ch3.enabled() as u8) << 2
            | (self.ch2.enabled() as u8) << 1
            | self.ch1.enabled() as u8
    }

    fn set_master_control(&mut self, value: u8) {
        self.enabled = value & 0x80 == 0x80;
        if !self.enabled {
            self.reset();
        }
    }

    fn master_volume(&self) -> u8 {
        let left_volume = self.left_volume << 4;
        let right_volume = self.right_volume;
        left_volume | right_volume
    }

    fn set_master_volume(&mut self, value: u8) {
        self.left_volume = (value & 0x70) >> 4;
        self.right_volume = value & 0x07;
    }

    fn set_sound_panning(&mut self, value: u8) {
        self.sound_panning = value
    }

    fn mix(&self) -> (u8, u8) {
        let mut output_left = 0;
        let mut output_right = 0;
        let channel_outputs = [self.ch4.output(), self.ch3.output(), self.ch2.output(), self.ch1.output()];
        for (i, output) in channel_outputs.iter().enumerate() {
            if self.sound_panning & (1 << (7 - i)) != 0 {
                output_left += output;
            }
            if self.sound_panning & (1 << (3 - i)) != 0 {
                output_right += output;
            }
        }
        (output_left / 4, output_right / 4)
    }

    fn reset(&mut self) {
        self.ch1.reset();
        self.ch2.reset();
        self.ch3.reset();
        self.ch4.reset();
        self.frame_sequencer.reset();
        self.sound_panning = 0;
        self.left_volume = 0;
        self.right_volume = 0;
        self.counter = 0.0;
        self.audio_buffer.lock().unwrap().clear();
    }
}
