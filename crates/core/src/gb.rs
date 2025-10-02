use std::{cell::RefCell, rc::Rc};

use getset::Getters;

use crate::{JoypadButton, cartridge::Cartridge, cpu::Cpu, system_bus::SystemBus};

#[derive(Getters)]
pub struct GameBoy {
    pub cpu: Cpu<SystemBus>,
    game_title: String,
}

impl GameBoy {
    pub fn new(rom_name: &str, buffer: Vec<u8>) -> GameBoy {
        let cartridge = Cartridge::load(rom_name.into(), buffer).unwrap();
        let game_title = cartridge.title().to_string();
        let mode = cartridge.mode();
        let halted = Rc::new(RefCell::new(false));
        GameBoy {
            cpu: Cpu::new(SystemBus::new(cartridge, halted.clone()), mode, halted),
            game_title,
        }
    }

    pub fn run_until_frame(&mut self) -> bool {
        loop {
            self.cpu.cycle();

            if self.frame_ready() {
                return true;
            }
        }
    }

    pub fn run_until_audio_buffer_full(&mut self) -> (&[f32], &[f32]) {
        self.cpu.bus_mut().apu_mut().clear_audio_buffers();

        while !self.cpu.bus().apu().audio_buffers_full() {
            self.cpu.cycle();
        }

        let left_samples = self.cpu.bus().apu().left_audio_buffer();
        let right_samples = self.cpu.bus().apu().right_audio_buffer();

        (left_samples, right_samples)
    }

    fn frame_ready(&mut self) -> bool {
        let result = self.cpu.bus().ppu().frame_ready();
        self.cpu.bus_mut().ppu_mut().set_frame_ready(false);
        result
    }

    pub fn current_frame(&self) -> &Vec<(u8, u8, u8)> {
        self.cpu.bus().ppu().frame_buffer()
    }

    pub fn game_title(&self) -> String {
        self.game_title.clone()
    }

    pub fn button_up(&mut self, button: JoypadButton) {
        self.cpu.bus_mut().joy_pad_mut().button_up(button)
    }

    pub fn button_down(&mut self, button: JoypadButton) {
        self.cpu.bus_mut().joy_pad_mut().button_down(button)
    }
}
