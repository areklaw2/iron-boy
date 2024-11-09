use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use ironboy_core::{
    bus::Bus,
    cartridge::Cartridge,
    cpu::{registers::Registers, Cpu},
    GameBoyMode, JoypadButton,
};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::KeyboardEvent;

#[wasm_bindgen]
pub struct GameBoy {
    cpu: Cpu,
    volume: u8,
}

#[wasm_bindgen]
impl GameBoy {
    #[wasm_bindgen(constructor)]
    pub fn new_dmg(rom_name: &str, buffer: Vec<u8>, skip_boot: bool) -> GameBoy {
        let cartridge = Cartridge::load(rom_name.into(), buffer).unwrap();
        GameBoy {
            cpu: Cpu::new(Bus::new(cartridge), Registers::new(GameBoyMode::Monochrome, skip_boot)),
            volume: 50,
        }
    }

    pub fn cycle(&mut self) -> u32 {
        self.cpu.cycle()
    }

    pub fn update_ppu(&mut self) -> bool {
        let result = self.cpu.bus.ppu.screen_updated;
        self.cpu.bus.ppu.screen_updated = false;
        result
    }

    fn ppu_buffer(&self) -> &[(u8, u8, u8)] {
        &self.cpu.bus.ppu.screen_buffer
    }

    fn audio_buffer(&self) -> &Arc<Mutex<VecDeque<u8>>> {
        &self.cpu.bus.apu.audio_buffer
    }

    fn audio_buffer_mut(&mut self) -> &mut Arc<Mutex<VecDeque<u8>>> {
        &mut self.cpu.bus.apu.audio_buffer
    }

    pub fn left_volume(&self) -> u8 {
        self.cpu.bus.apu.left_volume
    }

    pub fn right_volume(&self) -> u8 {
        self.cpu.bus.apu.left_volume
    }

    pub fn volume(&self) -> u8 {
        self.volume
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

    pub fn button_up(&mut self, event: KeyboardEvent) {
        let key = event.key();
        if let Some(button) = match_key_to_button(&key) {
            self.cpu.bus.joy_pad.button_up(button)
        }
    }

    pub fn button_down(&mut self, event: KeyboardEvent) {
        let key = event.key();
        if let Some(button) = match_key_to_button(&key) {
            self.cpu.bus.joy_pad.button_down(button)
        }
    }
}

fn match_key_to_button(key: &str) -> Option<JoypadButton> {
    match key {
        "x" => Some(JoypadButton::A),
        "z" => Some(JoypadButton::B),
        "return" => Some(JoypadButton::Select),
        "space" => Some(JoypadButton::Start),
        "up" => Some(JoypadButton::Up),
        "left" => Some(JoypadButton::Left),
        "down" => Some(JoypadButton::Down),
        "right" => Some(JoypadButton::Right),
        _ => None,
    }
}
