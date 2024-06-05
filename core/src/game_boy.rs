use utils::Mode;

use crate::{
    bus::Bus,
    cartridge::{self, Cartridge},
    cpu::{registers::Registers, Cpu},
    io::joypad::JoypadButton,
};

pub struct GameBoy {
    cpu: Cpu,
}

impl GameBoy {
    pub fn new_dmg(rom_name: &str) -> GameBoy {
        let cartridge = Cartridge::load(rom_name);
        GameBoy {
            cpu: Cpu::new(Bus::new(cartridge), Registers::new(Mode::Monochrome)),
        }
    }

    pub fn new_cgb(rom_name: &str) -> GameBoy {
        todo!()
    }

    pub fn cycle(&mut self) -> u32 {
        self.cpu.cycle()
    }

    pub fn get_ppu_update(&mut self) -> bool {
        let result = self.cpu.bus.ppu.updated;
        self.cpu.bus.ppu.updated = false;
        result
    }

    pub fn get_ppu_data(&self) -> &[u8] {
        &self.cpu.bus.ppu.vram
    }

    pub fn enable_audio() {
        todo!()
    }

    pub fn sync_audio(&mut self) {
        //todo!()
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

    pub fn lines(&self) -> &[String] {
        &self.cpu.lines
    }
}
