use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use getset::Getters;

use crate::{JoypadButton, cartridge::Cartridge, cpu::Cpu, memory::system_bus::SystemBus};

#[derive(Getters)]
pub struct GameBoy {
    pub cpu: Cpu<SystemBus>,
    game_title: String,
    #[getset(get = "pub")]
    pub volume: u8,
    frame_count: u64,
    total_cycles: u64,
    last_profile_time: std::time::Instant,
}

impl GameBoy {
    pub fn new(rom_name: &str, buffer: Vec<u8>) -> GameBoy {
        let cartridge = Cartridge::load(rom_name.into(), buffer).unwrap();
        let game_title = cartridge.title().to_string();
        let mode = cartridge.mode();
        GameBoy {
            cpu: Cpu::new(SystemBus::new(cartridge), mode),
            game_title,
            volume: 50,
            frame_count: 0,
            total_cycles: 0,
            last_profile_time: std::time::Instant::now(),
        }
    }

    pub fn run_until_frame(&mut self) -> bool {
        let frame_start = std::time::Instant::now();
        let cycles_start = self.total_cycles;
        
        loop {
            self.cpu.cycle();
            self.total_cycles += 1;
            
            if self.frame_ready() {
                let frame_time = frame_start.elapsed();
                let cycles_this_frame = self.total_cycles - cycles_start;
                self.frame_count += 1;
                
                // Profile every 60 frames (roughly 1 second at 60fps)
                if self.frame_count % 60 == 0 {
                    let now = std::time::Instant::now();
                    let elapsed_since_last = now.duration_since(self.last_profile_time);
                    let avg_frame_time = elapsed_since_last.as_micros() / 60;
                    let fps = 1_000_000.0 / avg_frame_time as f64;
                    let cycles_per_second = (self.total_cycles as f64) / elapsed_since_last.as_secs_f64();
                    
                    eprintln!("Frame {}: {}μs, {:.1} FPS, {:.0} cycles/sec, {} cycles/frame", 
                        self.frame_count, 
                        frame_time.as_micros(),
                        fps,
                        cycles_per_second,
                        cycles_this_frame
                    );
                    
                    self.last_profile_time = now;
                }
                
                return true;
            }
        }
    }

    fn frame_ready(&mut self) -> bool {
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
