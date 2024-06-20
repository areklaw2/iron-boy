use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TrySendError};

use envelope::Envelope;
use log::error;
use lsfr::Lfsr;
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
pub const SOUND_MAX: u8 = 15;
pub const SAMPLE_MAX: u8 = SOUND_MAX * 4 * 2;
pub type SampleBuffer = [u8; SAMPLES_PER_BUFFER];

pub fn samples_per_steps(steps: u32) -> u32 {
    steps / SAMPLER_DIVIDER
}

pub trait Channel {
    fn step(&mut self);
    fn sample(&self) -> u8;
    fn start(&mut self);
    fn running(&self) -> bool;
}

#[derive(Clone, Copy)]
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

    fn mix(self, channels: [u8; 4]) -> u8 {
        let mut data = 0;
        for i in 0..4 {
            if self.channels[i] {
                data += channels[i];
            }
        }
        data
    }
}

#[derive(Clone, Copy)]
struct OutputVolume {
    vin: bool,
    level: u8,
}

impl OutputVolume {
    fn write(data: u8) -> OutputVolume {
        OutputVolume {
            vin: data & 0x8 != 0,
            level: 8 - (data & 0x7),
        }
    }

    fn read(self) -> u8 {
        ((self.vin as u8) << 3) | (8 - self.level)
    }

    fn process(self, sample: u8) -> u8 {
        sample / self.level as u8
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

    fn sample(&self, channels: [u8; 4]) -> u8 {
        let mixed = self.mixer.mix(channels);

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
            0xFF24 => self.nr50(),
            0xFF25 => self.nr51(),
            0xFF26 => self.nr52(),
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
            0xFF1A..=0xFF1E => {
                if !self.enabled {
                    return;
                }
                self.channel3.mem_write(address, data)
            }
            0xFF20..=0xFF23 => {
                if !self.enabled {
                    return;
                }
                self.channel4.mem_write(address, data)
            }
            0xFF24 => self.set_nr50(data),
            0xFF25 => self.set_nr51(data),
            0xFF26 => self.set_nr52(data),
            0xFF30..=0xFF3F => self.channel3.mem_write(address, data),
            _ => {}
        }
    }
}

impl Apu {
    pub fn new() -> (Apu, Receiver<SampleBuffer>) {
        let (tx, rx) = sync_channel(CHANNEL_DEPTH);

        let spu = Apu {
            enabled: false,
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

        (spu, rx)
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
        let sounds = [
            self.channel1.sample(),
            self.channel2.sample(),
            self.channel3.sample(),
            self.channel4.sample(),
        ];

        let sample = self.output1.sample(sounds) + self.output2.sample(sounds);
        self.output_sample(sample);
    }

    fn output_sample(&mut self, sample: u8) {
        self.buffer[self.position] = sample;
        self.position += 1;
        if self.position == self.buffer.len() {
            if let Err(e) = self.output.try_send(self.buffer) {
                match e {
                    TrySendError::Full(_) => error!("Channel is full, dropping {} samples", self.buffer.len()),
                    e => panic!("Couldn't send audio buffer: {:?}", e),
                }
            }
            self.position = 0;
        }
    }

    /// Retreive sound output volume register
    pub fn nr50(&self) -> u8 {
        let v1 = self.output1.volume().read();
        let v2 = self.output2.volume().read();

        (v2 << 4) | v1
    }

    /// Set sound output volume
    pub fn set_nr50(&mut self, val: u8) {
        if !self.enabled {
            return;
        }

        self.output1.set_volume(OutputVolume::write(val & 0xf));
        self.output2.set_volume(OutputVolume::write(val >> 4));
    }

    /// Retreive sound output mixer register
    pub fn nr51(&self) -> u8 {
        let v1 = self.output1.mixer().read();
        let v2 = self.output2.mixer().read();

        (v2 << 4) | v1
    }

    /// Set sound output mixers
    pub fn set_nr51(&mut self, val: u8) {
        if !self.enabled {
            return;
        }

        self.output1.set_mixer(Mixer::write(val & 0xf));
        self.output2.set_mixer(Mixer::write(val >> 4));
    }

    /// Get global sound enable and sound status
    pub fn nr52(&self) -> u8 {
        let enabled = self.enabled as u8;
        let r1 = self.channel1.running() as u8;
        let r2 = self.channel2.running() as u8;
        let r3 = self.channel3.running() as u8;
        let r4 = self.channel4.running() as u8;

        enabled << 7 | 0x70 | (r4 << 3) | (r3 << 2) | (r2 << 1) | r1
    }

    /// Set SPU enable
    pub fn set_nr52(&mut self, val: u8) {
        self.enabled = val & 0x80 != 0;

        if !self.enabled {
            self.reset()
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
