use crate::{
    bus::Bus,
    cartridge::Cartridge,
    cpu::{registers::Registers, Cpu, CPU_CLOCK_SPEED},
    GameBoyMode, JoypadButton, FPS,
};

pub struct GameBoy {
    pub cpu: Cpu,
    game_title: String,
    pub volume: u8,
}

impl GameBoy {
    pub fn new(rom_name: &str, buffer: Vec<u8>, skip_boot: bool) -> GameBoy {
        let cartridge = Cartridge::load(rom_name.into(), buffer).unwrap();
        let game_title = cartridge.title().to_string();
        GameBoy {
            cpu: Cpu::new(Bus::new(cartridge), Registers::new(GameBoyMode::Monochrome, skip_boot)),
            game_title,
            volume: 50,
        }
    }

    pub fn run_frame(&mut self) {
        let cycles_per_frame = CPU_CLOCK_SPEED as f32 / FPS;
        let mut cycles_passed = 0.0;
        while cycles_passed <= cycles_per_frame {
            let ticks = self.cpu.cycle();
            cycles_passed += (ticks) as f32;
        }
    }

    pub fn ppu_buffer(&self) -> &[(u8, u8, u8)] {
        &self.cpu.bus.ppu.screen_buffer
    }

    pub fn game_title(&self) -> String {
        self.game_title.clone()
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
        self.cpu.bus.joy_pad.button_up(button)
    }

    pub fn button_down(&mut self, button: JoypadButton) {
        self.cpu.bus.joy_pad.button_down(button)
    }
}
