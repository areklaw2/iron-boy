use blip_buf::BlipBuf;
use pulse::PulseChannel;
use registers::{AudioMasterControl, MasterVolumeAndVinPanning, SoundPanning};

use crate::{bus::Memory, io::audio_player::AudioPlayer};

mod length_timer;
mod pulse;
mod registers;
mod volume_envelope;

pub const WAVE_PATTERN: [[i32; 8]; 4] = [
    [-1, -1, -1, -1, 1, -1, -1, -1],
    [-1, -1, -1, -1, 1, 1, -1, -1],
    [-1, -1, 1, 1, 1, 1, -1, -1],
    [1, 1, 1, 1, -1, -1, 1, 1],
];
const CLOCKS_PER_SECOND: u32 = 4194304;

pub trait ChannelMemory {
    fn mem_read(&mut self, address: u16) -> u8;

    fn mem_write(&mut self, address: u16, data: u8, frame_step: u8);
}

pub struct Apu {
    channel1: PulseChannel,
    frame_step: u8,
    master_volume_and_vin_panning: MasterVolumeAndVinPanning,
    sound_panning: SoundPanning,
    audio_master_control: AudioMasterControl,
}

impl Memory for Apu {
    fn mem_read(&mut self, address: u16) -> u8 {
        match address {
            0xFF10..=0xFF14 => self.channel1.mem_read(address),
            0xFF24 => self.master_volume_and_vin_panning.reg_read(),
            0xFF25 => self.sound_panning.bits(),
            0xFF26 => self.audio_master_control.bits(),
            _ => 0xFF,
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        match address {
            0xFF10..=0xFF14 => self.channel1.mem_write(address, data, self.frame_step),
            0xFF24 => self.master_volume_and_vin_panning = MasterVolumeAndVinPanning::reg_write(data),
            0xFF25 => self.sound_panning = SoundPanning::from_bits_truncate(data),
            0xFF26 => self.audio_master_control_write(data),
            _ => {} //panic!("APU does not handle write {:04X}", address),
        }
    }
}

impl Apu {
    pub fn new(audio_player: Box<dyn AudioPlayer>) -> Self {
        let blip_buffer1 = create_blip_buffer(audio_player.samples_rate());
        Apu {
            audio_master_control: AudioMasterControl::from_bits_truncate(0),
            sound_panning: SoundPanning::from_bits_truncate(0),
            master_volume_and_vin_panning: MasterVolumeAndVinPanning::reg_write(0),
            frame_step: 0,
            channel1: PulseChannel::new(blip_buffer1, true),
        }
    }

    pub fn cycle(&mut self, ticks: u32) {
        todo!()
    }

    pub fn sync(&mut self) {
        todo!();
    }

    fn audio_master_control_write(&mut self, data: u8) {
        let previous_audio_on = self.audio_master_control.contains(AudioMasterControl::AudioOn);
        self.audio_master_control.set(AudioMasterControl::AudioOn, data & 0x80 == 0x80);
        if previous_audio_on && !self.audio_master_control.contains(AudioMasterControl::AudioOn) {
            // Handle turning off the the APU
            // All registers should be cleared and read only
            // except NR52 and the length timer registers
        }
    }
}

fn create_blip_buffer(samples_rate: u32) -> BlipBuf {
    let mut blip_buffer = BlipBuf::new(samples_rate);
    blip_buffer.set_rates(CLOCKS_PER_SECOND as f64, samples_rate as f64);
    blip_buffer
}
