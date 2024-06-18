use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TrySendError};

use log::error;
use noise::NoiseChannel;
use pulse::PulseChannel;
use wave::WaveChannel;

use crate::bus::Memory;
use crate::bus::SYSTEM_CLOCK_FREQUENCY;

mod envelope;
mod lsfr;
mod noise;
mod pulse;
mod sweep;
mod wave;

pub const SAMPLES_PER_BUFFER: usize = 512;
const SAMPLER_DIVIDER: u32 = 95;
pub const SAMPLE_RATE: u32 = SYSTEM_CLOCK_FREQUENCY as u32 / SAMPLER_DIVIDER;
const CHANNEL_DEPTH: usize = 4;
pub const SOUND_MAX: Sample = 15;
pub const SAMPLE_MAX: Sample = SOUND_MAX * 4 * 2;
pub type Sample = u8;
pub type SampleBuffer = [Sample; SAMPLES_PER_BUFFER];

pub fn samples_per_steps(steps: u32) -> u32 {
    steps / SAMPLER_DIVIDER
}

pub trait Channel {
    fn step(&mut self);
    fn sample(&self) -> Sample;
    fn start(&mut self);
    fn running(&self) -> bool;
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Continuous = 0,
    Counter = 1,
}

#[derive(Debug, Clone, Copy)]
struct Mixer {
    channels: [bool; 4],
}

impl Mixer {
    fn write(data: u8) -> Mixer {
        let mut mixer = Mixer { channels: [false; 4] };
        for i in 0..4 {
            mixer.channels[i] = (data & (1 << i)) != 0;
        }
        mixer
    }

    fn read(self) -> u8 {
        let mut data = 0;
        for i in 0..self.channels.len() {
            data |= (self.channels[i] as u8) << i
        }
        data
    }

    fn mix(self, sounds: [Sample; 4]) -> Sample {
        let mut data = 0;
        for i in 0..4 {
            if self.channels[i] {
                data += sounds[i];
            }
        }
        data
    }
}

#[derive(Debug, Clone, Copy)]
struct OutputVolume {
    vin: bool,
    level: u8,
}

impl OutputVolume {
    fn write(data: u8) -> OutputVolume {
        OutputVolume {
            vin: data & 0x08 != 0,
            level: 8 - (data & 0x07),
        }
    }

    fn read(self) -> u8 {
        ((self.vin as u8) << 3) | (8 - self.level)
    }

    fn process(self, sample: Sample) -> Sample {
        sample / self.level as Sample
    }
}

struct Output {
    mixer: Mixer,
    volume: OutputVolume,
}

impl Output {
    fn new() -> Output {
        Output {
            mixer: Mixer::write(0),
            volume: OutputVolume::write(0),
        }
    }

    fn sample(&self, sounds: [Sample; 4]) -> Sample {
        let mixed = self.mixer.mix(sounds);
        self.volume.process(mixed)
    }

    fn mixer(&self) -> Mixer {
        self.mixer
    }

    fn set_mixer(&mut self, mixer: Mixer) {
        self.mixer = mixer;
    }

    fn volume(&self) -> OutputVolume {
        self.volume
    }

    fn set_volume(&mut self, volume: OutputVolume) {
        self.volume = volume;
    }
}

pub struct Apu {
    enabled: bool,
    timer: u32,
    divider: u32,
    output: SyncSender<SampleBuffer>,
    buffer: SampleBuffer,
    position: usize,
    channel1: PulseChannel,
    channel2: PulseChannel,
    channel3: WaveChannel,
    channel4: NoiseChannel,
    output1: Output,
    output2: Output,
}

impl Memory for Apu {
    fn mem_read(&mut self, address: u16) -> u8 {
        match address {
            0xFF10..=0xFF14 => self.channel1.mem_read(address),
            0xFF16..=0xFF19 => self.channel2.mem_read(address),
            0xFF1A..=0xFF1E => self.channel3.mem_read(address),
            0xFF20..=0xFF23 => self.channel4.mem_read(address),
            0xFF24 => {
                let data1 = self.output1.volume().read();
                let data2 = self.output2.volume().read();
                (data2 << 4) | data1
            }
            0xFF25 => {
                let data1 = self.output1.mixer().read();
                let data2 = self.output2.mixer().read();
                (data2 << 4) | data1
            }
            0xFF26 => {
                let mut data = 0;
                data |= (self.enabled as u8) << 7;
                data |= 0x70;
                data |= (self.channel4.running() as u8) << 3;
                data |= (self.channel3.running() as u8) << 2;
                data |= (self.channel2.running() as u8) << 1;
                data |= self.channel1.running() as u8;
                data
            }
            0xFF30..=0xFF3F => self.channel3.mem_read(address),
            _ => 0xFF,
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        match address {
            0xFF10..=0xFF14 => {
                if !self.enabled {
                    return;
                }
                self.channel1.mem_write(address, data)
            }
            0xFF16..=0xFF19 => {
                if !self.enabled {
                    return;
                }
                self.channel2.mem_write(address, data)
            }
            0xFF1A..=0xFF1E => self.channel3.mem_write(address, data),
            0xFF20..=0xFF23 => {
                if !self.enabled {
                    return;
                }
                self.channel4.mem_write(address, data)
            }
            0xFF24 => {
                if !self.enabled {
                    return;
                }
                self.output1.set_volume(OutputVolume::write(data & 0x0F));
                self.output2.set_volume(OutputVolume::write(data >> 4));
            }
            0xFF25 => {
                if !self.enabled {
                    return;
                }

                self.output1.set_mixer(Mixer::write(data & 0x0F));
                self.output2.set_mixer(Mixer::write(data >> 4));
            }
            0xFF26 => {
                self.enabled = data & 0x80 == 0x80;
                if !self.enabled {
                    self.reset()
                }
            }
            0xFF30..=0xFF3F => self.channel3.mem_write(address, data),
            _ => {}
        }
    }
}

impl Apu {
    pub fn new() -> (Apu, Receiver<SampleBuffer>) {
        let (tx, rx) = sync_channel(CHANNEL_DEPTH);
        let apu = Apu {
            enabled: false,
            timer: 0,
            divider: 0,
            output: tx,
            buffer: [0; SAMPLES_PER_BUFFER],
            position: 0,
            channel1: PulseChannel::new(),
            channel2: PulseChannel::new(),
            channel3: WaveChannel::new(),
            channel4: NoiseChannel::new(),
            output1: Output::new(),
            output2: Output::new(),
        };

        (apu, rx)
    }

    pub fn cycle(&mut self) {
        if !self.enabled {
            return;
        }

        self.channel1.step();
        self.channel2.step();
        self.channel3.step();
        self.channel4.step();

        if self.divider == 0 {
            self.divider = SAMPLER_DIVIDER;
            self.sample();
        }

        self.divider -= 1;
    }

    fn sample(&mut self) {
        let channels = [
            self.channel1.sample(),
            self.channel2.sample(),
            self.channel3.sample(),
            self.channel4.sample(),
        ];

        let sample = self.output1.sample(channels) + self.output2.sample(channels);
        self.output_sample(sample);
    }

    fn output_sample(&mut self, sample: Sample) {
        self.buffer[self.position] = sample;
        self.position += 1;
        if self.position == self.buffer.len() {
            if let Err(e) = self.output.try_send(self.buffer) {
                match e {
                    TrySendError::Full(_) => error!("Sound channel is full, dropping {} samples", self.buffer.len()),
                    e => panic!("Couldn't send audio buffer: {:?}", e),
                }
            }

            self.position = 0;
        }
    }

    fn reset(&mut self) {
        self.channel1 = PulseChannel::new();
        self.channel2 = PulseChannel::new();
        self.channel3 = WaveChannel::with_ram(&self.channel3);
        self.channel4 = NoiseChannel::new();
        self.output1 = Output::new();
        self.output2 = Output::new();
    }
}
