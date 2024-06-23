use crate::{
    bus::Bus,
    cartridge::Cartridge,
    cpu::{registers::Registers, Cpu},
    io::joypad::JoypadButton,
};

pub struct GameBoy {
    pub cpu: Cpu,
    pub volume: u8,
}

impl GameBoy {
    pub fn new_dmg(rom_name: &str, skip_boot: bool) -> GameBoy {
        let cartridge = Cartridge::load(rom_name.into()).unwrap();
        GameBoy {
            cpu: Cpu::new(Bus::new(cartridge), Registers::new(utils::GbMode::Monochrome, skip_boot)),
            volume: 50,
        }
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

    pub fn button_up(&mut self, button: JoypadButton) {
        self.cpu.bus.joy_pad.button_up(button)
    }

    pub fn button_down(&mut self, button: JoypadButton) {
        self.cpu.bus.joy_pad.button_down(button)
    }

    pub fn increase_volume(&mut self) {
        if self.volume > 95 {
            return;
        }
        self.volume += 5;
    }

    pub fn decrease_volume(&mut self) {
        if self.volume < 5 {
            return;
        }
        self.volume -= 5;
    }
}
