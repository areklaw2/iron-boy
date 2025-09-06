use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use getset::Getters;

use crate::{
    JoypadButton,
    cartridge::Cartridge,
    cpu::{CPU_CLOCK_SPEED, Cpu, registers::Registers},
    memory::system_bus::SystemBus,
    ppu::FPS,
};

#[derive(Getters)]
pub struct GameBoy {
    pub cpu: Cpu<SystemBus>,
    game_title: String,
    #[getset(get = "pub")]
    pub volume: u8,
}

impl GameBoy {
    pub fn new(rom_name: &str, buffer: Vec<u8>) -> GameBoy {
        let cartridge = Cartridge::load(rom_name.into(), buffer).unwrap();
        let game_title = cartridge.title().to_string();
        let mode = cartridge.mode();
        GameBoy {
            cpu: Cpu::new(SystemBus::new(cartridge), Registers::new(mode)),
            game_title,
            volume: 50,
        }
    }

    pub fn run_frame(&mut self) -> bool {
        let cycles_per_frame = CPU_CLOCK_SPEED as f32 / FPS;
        let mut cycles_passed = 0.0;
        while cycles_passed <= cycles_per_frame {
            let cycles = self.cpu.cycle();
            if self.ppu_updated() {
                return true;
            }
            cycles_passed += cycles as f32;
        }
        false
    }

    fn ppu_updated(&mut self) -> bool {
        let result = self.cpu.bus().ppu().frame_ready();
        self.cpu.bus_mut().ppu_mut().set_frame_ready(false);
        result
    }

    pub fn current_frame(&self) -> &Vec<(u8, u8, u8)> {
        self.cpu.bus().ppu().read_buffer()
    }

    pub fn game_title(&self) -> String {
        self.game_title.clone()
    }

    pub fn audio_buffer(&mut self) -> &mut Arc<Mutex<VecDeque<u8>>> {
        self.cpu.bus_mut().apu_mut().audio_buffer_mut()
    }

    pub fn left_volume(&self) -> &u8 {
        self.cpu.bus().apu().left_volume()
    }

    pub fn right_volume(&self) -> &u8 {
        self.cpu.bus().apu().right_volume()
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

    pub fn button_up(&mut self, button: JoypadButton) {
        self.cpu.bus_mut().joy_pad_mut().button_up(button)
    }

    pub fn button_down(&mut self, button: JoypadButton) {
        self.cpu.bus_mut().joy_pad_mut().button_down(button)
    }
}
