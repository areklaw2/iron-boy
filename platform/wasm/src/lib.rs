use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use ironboy_core::{
    apu::AUDIO_BUFFER_THRESHOLD,
    bus::Bus,
    cartridge::Cartridge,
    cpu::{registers::Registers, Cpu, CPU_CLOCK_SPEED},
    GameBoyMode, JoypadButton, FPS,
};
use wasm_bindgen::{
    prelude::{wasm_bindgen, Closure},
    Clamped, JsCast, JsValue,
};
use web_sys::CanvasRenderingContext2d;

mod utils;

const FRAME_DURATION_MS: f32 = 1_000.0 / FPS;
const FRAME_DURATION_MICROS: f32 = FRAME_DURATION_MS * 1000.0;
const FRAME_DURATION: web_time::Duration = web_time::Duration::from_micros(FRAME_DURATION_MICROS as u64);

#[wasm_bindgen]
pub struct GameBoy {
    cpu: Cpu,
    game_title: String,
    frame: Option<Box<[u8]>>,
    volume: u8,
}

#[wasm_bindgen]
impl GameBoy {
    #[wasm_bindgen(constructor)]
    pub fn new_dmg(rom_name: &str, buffer: Vec<u8>, skip_boot: bool) -> GameBoy {
        utils::set_panic_hook();
        let cartridge = Cartridge::load(rom_name.into(), buffer).unwrap();
        let game_title = cartridge.title().to_string();
        GameBoy {
            cpu: Cpu::new(Bus::new(cartridge), Registers::new(GameBoyMode::Monochrome, skip_boot)),
            game_title,
            frame: Some(vec![0; 160 * 144 * 4].into_boxed_slice()),
            volume: 50,
        }
    }

    fn cycle(&mut self) -> u32 {
        self.cpu.cycle()
    }

    pub fn get_frame(&mut self, context: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        let cycles_per_frame = CPU_CLOCK_SPEED as f32 / FPS;
        let mut cycles_passed = 0.0;
        while cycles_passed <= cycles_per_frame {
            let ticks = self.cycle();
            if self.update_ppu() {
                let mut frame = self.frame.take().unwrap();
                transform(self.ppu_buffer(), &mut frame);
                let data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut frame), 160, 144)?;
                self.frame.replace(frame);
                context.put_image_data(&data, 0.0, 0.0)?;
            }
            cycles_passed += (ticks) as f32;
        }

        Ok(())
    }

    fn update_ppu(&mut self) -> bool {
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

    pub fn button_up(&mut self, key: &str) {
        if let Some(button) = match_key_to_button(&key) {
            self.cpu.bus.joy_pad.button_up(button)
        }
    }

    pub fn button_down(&mut self, key: &str) {
        if let Some(button) = match_key_to_button(&key) {
            self.cpu.bus.joy_pad.button_down(button)
        }
    }

    pub fn game_title(&self) -> String {
        self.game_title.clone()
    }
}

fn match_key_to_button(key: &str) -> Option<JoypadButton> {
    match key {
        "a" => Some(JoypadButton::A),
        "s" => Some(JoypadButton::B),
        "Enter" => Some(JoypadButton::Select),
        "Backspace" => Some(JoypadButton::Start),
        "ArrowUp" => Some(JoypadButton::Up),
        "ArrowLeft" => Some(JoypadButton::Left),
        "ArrowDown" => Some(JoypadButton::Down),
        "ArrowRight" => Some(JoypadButton::Right),
        _ => None,
    }
}

fn transform(input_buffer: &[(u8, u8, u8)], output_buffer: &mut [u8]) {
    for i in 0..input_buffer.len() {
        let (red, green, blue) = input_buffer[i];
        output_buffer[4 * i + 0] = red;
        output_buffer[4 * i + 1] = green;
        output_buffer[4 * i + 2] = blue;
        output_buffer[4 * i + 3] = 255
    }
}

fn should_sync(frame_start_time: web_time::Instant, audio_buffer: &Arc<Mutex<VecDeque<u8>>>) -> bool {
    frame_start_time.elapsed().as_micros() < FRAME_DURATION.as_micros() && audio_buffer.lock().unwrap().len() > AUDIO_BUFFER_THRESHOLD
}
