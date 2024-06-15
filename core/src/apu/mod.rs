use registers::{AudioMasterControl, MasterVolumeAndVinPanning, SoundPanning};

use crate::bus::Memory;

pub mod registers;

pub struct Apu {
    audio_master_control: AudioMasterControl,
    sound_panning: SoundPanning,
    master_volume_and_vin_panning: MasterVolumeAndVinPanning,
}

impl Memory for Apu {
    fn mem_read(&mut self, address: u16) -> u8 {
        match address {
            0xFF24 => self.master_volume_and_vin_panning.reg_read(),
            0xFF25 => self.sound_panning.bits(),
            0xFF26 => self.audio_master_control.bits(),
            _ => 0xFF,
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        match address {
            0xFF24 => self.master_volume_and_vin_panning = MasterVolumeAndVinPanning::reg_write(data),
            0xFF25 => self.sound_panning = SoundPanning::from_bits_truncate(data),
            0xFF26 => self.audio_master_control_write(data),
            _ => {} //panic!("APU does not handle write {:04X}", address),
        }
    }
}

impl Apu {
    pub fn new() -> Self {
        Apu {
            audio_master_control: AudioMasterControl::from_bits_truncate(0),
            sound_panning: SoundPanning::from_bits_truncate(0),
            master_volume_and_vin_panning: MasterVolumeAndVinPanning::reg_write(0),
        }
    }

    fn audio_master_control_write(&mut self, data: u8) {
        let previous_audio_on = self.audio_master_control.contains(AudioMasterControl::AudioEnabled);
        self.audio_master_control.set(AudioMasterControl::AudioEnabled, data & 0x80 == 0x80);
        if previous_audio_on && !self.audio_master_control.contains(AudioMasterControl::AudioEnabled) {
            // Handle turning off the the APU
            // All registers should be cleared and read only
            // except NR52 and the length timer registers
        }
    }
}
