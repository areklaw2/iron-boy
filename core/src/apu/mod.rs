use channel::Channel;
use frame_sequencer::FrameSequencer;
use mixer::Mixer;
use noise::NoiseChannel;
use square::SquareChannel;
use wave::WaveChannel;

use crate::{bus::MemoryAccess, cpu::CPU_CLOCK_SPEED};
use std::collections::VecDeque;

mod channel;
mod frame_sequencer;
mod mixer;
mod noise;
mod square;
mod wave;

pub const SAMPLING_RATE: u16 = 1024;
pub const SAMPLING_FREQUENCY: u16 = 44100;
pub const APU_CLOCK_SPEED: u16 = 512;
const CPU_CYCLES_PER_SAMPLE: f32 = CPU_CLOCK_SPEED as f32 / SAMPLING_FREQUENCY as f32;
pub const AUDIO_BUFFER_THRESHOLD: usize = SAMPLING_RATE as usize * 4;

pub struct Apu {
    ch1: SquareChannel,
    ch2: SquareChannel,
    ch3: WaveChannel,
    ch4: NoiseChannel,
    frame_sequencer: FrameSequencer,
    mixer: Mixer,
    pub right_volume: u8,
    pub left_volume: u8,
    enabled: bool,
    counter: f32,
    pub audio_buffer: VecDeque<u8>,
}

impl MemoryAccess for Apu {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0xFF10..=0xFF14 => self.ch1.read_8(address),
            0xFF16..=0xFF19 => self.ch2.read_8(address),
            0xFF1A..=0xFF1E => self.ch3.read_8(address),
            0xFF20..=0xFF23 => self.ch4.read_8(address),
            0xFF24 => self.master_volume_read(),
            0xFF25 => self.mixer.read(),
            0xFF26 => self.master_control_read(),
            0xFF30..=0xFF3F => self.ch3.read_8(address),
            _ => 0xFF,
        }
    }

    fn write_8(&mut self, address: u16, data: u8) {
        if address == 0xFF26 {
            self.master_control_write(data);
            return;
        }

        if !self.enabled {
            return;
        }

        match address {
            0xFF10..=0xFF14 => self.ch1.write_8(address, data),
            0xFF16..=0xFF19 => self.ch2.write_8(address, data),
            0xFF1A..=0xFF1E => self.ch3.write_8(address, data),
            0xFF20..=0xFF23 => self.ch4.write_8(address, data),
            0xFF24 => self.master_volume_write(data),
            0xFF25 => self.mixer.write(data),
            0xFF26 => {}
            0xFF30..=0xFF3F => self.ch3.write_8(address, data),
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
            mixer: Mixer::new(),
            right_volume: 0,
            left_volume: 0,
            enabled: false,
            counter: 0.0,
            audio_buffer: VecDeque::new(),
        }
    }

    pub fn cycle(&mut self, ticks: u32) {
        if !self.enabled {
            return;
        }

        self.frame_sequencer
            .cycle(ticks, &mut self.ch1, &mut self.ch2, &mut self.ch3, &mut self.ch4);
        self.channel_cycle(ticks);
        self.counter += ticks as f32;

        while self.counter >= CPU_CYCLES_PER_SAMPLE {
            let (output_left, output_right) = self.mixer.mix([&self.ch1.base, &self.ch2.base, &self.ch3.base, &self.ch4.base]);
            self.audio_buffer.push_back(output_left);
            self.audio_buffer.push_back(output_right);
            self.counter -= CPU_CYCLES_PER_SAMPLE;
        }
    }

    fn channel_cycle(&mut self, ticks: u32) {
        self.ch1.cycle(ticks);
        self.ch2.cycle(ticks);
        self.ch3.cycle(ticks);
        self.ch4.cycle(ticks);
    }

    fn master_control_read(&self) -> u8 {
        let enabled = if self.enabled { 0x80 } else { 0x00 };
        let channel4_enabled = (self.ch4.base.enabled as u8) << 3;
        let channel3_enabled = (self.ch3.base.enabled as u8) << 2;
        let channel2_enabled = (self.ch2.base.enabled as u8) << 1;
        let channel1_enabled = self.ch1.base.enabled as u8;
        enabled | channel4_enabled | channel3_enabled | channel2_enabled | channel1_enabled
    }

    fn master_control_write(&mut self, data: u8) {
        self.enabled = data & 0x80 == 0x80;
        if !self.enabled {
            self.reset();
        }
    }

    fn master_volume_read(&self) -> u8 {
        let left_volume = (self.left_volume - 1) << 4;
        let right_volume = self.right_volume - 1;
        left_volume | right_volume
    }

    fn master_volume_write(&mut self, data: u8) {
        self.left_volume = ((data & 0x70) >> 4) + 1;
        self.right_volume = (data & 0x07) + 1;
    }

    fn reset(&mut self) {
        self.ch1.reset();
        self.ch2.reset();
        self.ch3.reset();
        self.ch4.reset();
        self.frame_sequencer.reset();
        self.mixer.reset();
        self.left_volume = 0;
        self.right_volume = 0;
        self.counter = 0.0;
        self.audio_buffer.clear();
    }
}
