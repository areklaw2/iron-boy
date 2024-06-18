use std::sync::mpsc::Receiver;

use crate::{
    apu::{Apu, SampleBuffer},
    bus::Bus,
    cartridge::Cartridge,
    cpu::{registers::Registers, Cpu},
    io::joypad::JoypadButton,
};

pub struct GameBoy {
    cpu: Cpu,
    audio_channel: Receiver<SampleBuffer>,
}

impl GameBoy {
    pub fn new_dmg(rom_name: &str, skip_boot: bool) -> GameBoy {
        let cartridge = Cartridge::load(rom_name).unwrap();
        cartridge.debug_output();

        let (apu, audio_channel) = Apu::new();

        GameBoy {
            cpu: Cpu::new(Bus::new(cartridge, apu), Registers::new(utils::GbMode::Monochrome, skip_boot)),
            audio_channel,
        }
    }

    pub fn new_cgb(rom_name: &str) -> GameBoy {
        todo!()
    }

    pub fn cycle(&mut self) -> u32 {
        self.cpu.cycle()
    }

    pub fn get_ppu_update(&mut self) -> bool {
        let result = self.cpu.bus.ppu.screen_updated;
        self.cpu.bus.ppu.screen_updated = false;
        result
    }

    pub fn get_ppu_data(&self) -> &[(u8, u8, u8)] {
        &self.cpu.bus.ppu.screen_buffer
    }

    pub fn get_vram(&self) -> &[u8] {
        &self.cpu.bus.ppu.vram
    }

    pub fn audio_channel(&self) -> &Receiver<SampleBuffer> {
        &self.audio_channel
    }

    pub fn button_up(&mut self, button: JoypadButton) {
        self.cpu.bus.joy_pad.button_up(button)
    }

    pub fn button_down(&mut self, button: JoypadButton) {
        self.cpu.bus.joy_pad.button_down(button)
    }

    pub fn rom_name(&self) -> String {
        todo!()
    }

    pub fn load_ram(&mut self, ramdata: &[u8]) {
        todo!()
    }

    pub fn dump_ram(&self) -> Vec<u8> {
        todo!()
    }

    pub fn cartrige_has_battery(&self) -> bool {
        todo!()
    }

    pub fn ram_updated(&mut self) -> bool {
        todo!()
    }
}
