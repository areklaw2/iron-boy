use utils::GbMode;

use crate::{
    apu::Apu,
    bus::Bus,
    cartridge::Cartridge,
    cpu::{registers::Registers, Cpu},
    io::{audio_player::AudioPlayer, joypad::JoypadButton},
};

pub struct GameBoy {
    cpu: Cpu,
}

impl GameBoy {
    pub fn new_dmg(rom_name: &str) -> GameBoy {
        let cartridge = Cartridge::load(rom_name).unwrap();
        cartridge.debug_output();
        GameBoy {
            cpu: Cpu::new(Bus::new(cartridge), Registers::new(GbMode::Monochrome)),
            //cpu: Cpu::new(Bus::new(cartridge), Registers::new1()),
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

    pub fn enable_audio(&mut self, audio_player: Box<dyn AudioPlayer>) {
        self.cpu.bus.apu = Some(Apu::new(audio_player))
    }

    pub fn sync_audio(&mut self) {
        if let Some(ref mut apu) = self.cpu.bus.apu {
            apu.sync();
        }
    }

    pub fn button_up(&mut self, button: JoypadButton) {
        //todo!()
    }

    pub fn button_down(&mut self, button: JoypadButton) {
        //todo!()
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

    pub fn ram_is_battery_backed(&self) -> bool {
        todo!()
    }

    pub fn check_and_reset_ram_updated(&mut self) -> bool {
        todo!()
    }
}
