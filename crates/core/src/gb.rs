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
    // frame_count: u64,
    // last_profile_time: std::time::Instant,
    // last_cycle_count: u64,
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
            // frame_count: 0,
            // last_profile_time: std::time::Instant::now(),
            // last_cycle_count: 0,
        }
    }

    pub fn run_until_frame(&mut self) -> bool {
        // let frame_start = std::time::Instant::now();
        // let cycles_start = *self.cpu.total_cycles();

        loop {
            self.cpu.cycle();

            if self.frame_ready() {
                // TODO: Make this into its own component FPS calculations
                // let frame_time = frame_start.elapsed();
                // let current_cycles = *self.cpu.total_cycles();
                // let cycles_this_frame = current_cycles - cycles_start;
                // self.frame_count += 1;

                // if self.frame_count % 60 == 0 {
                //     let now = std::time::Instant::now();
                //     let elapsed_since_last = now.duration_since(self.last_profile_time);
                //     let frame_per_μs = elapsed_since_last.as_micros() / 60;
                //     let fps = 1_000_000.0 / frame_per_μs as f64;
                //     let cycles_per_second = (current_cycles - self.last_cycle_count) as f64 / elapsed_since_last.as_secs_f64();

                //     eprintln!(
                //         "Frame {}: {}μs, {:.1} FPS, {:.0} cycles/sec, {} cycles/frame (should be about 70224)",
                //         self.frame_count,
                //         frame_time.as_micros(),
                //         fps,
                //         cycles_per_second,
                //         cycles_this_frame
                //     );

                //     self.last_profile_time = now;
                //     self.last_cycle_count = current_cycles;
                // }

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
