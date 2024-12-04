use ironboy_core::{gb::GameBoy, JoypadButton};
use wasm_bindgen::{prelude::wasm_bindgen, Clamped, JsValue};
use web_sys::CanvasRenderingContext2d;

mod utils;

#[wasm_bindgen]
pub struct Emulator {
    game_boy: GameBoy,
    game_title: String,
    frame: Option<Box<[u8]>>,
    volume: u8,
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new(rom_name: &str, buffer: Vec<u8>) -> Emulator {
        utils::set_panic_hook();
        let game_boy = GameBoy::new(rom_name, buffer);
        let game_title = game_boy.game_title();
        Emulator {
            game_boy,
            game_title,
            frame: Some(vec![0; 160 * 144 * 4].into_boxed_slice()),
            volume: 50,
        }
    }

    pub fn get_frame(&mut self, context: &CanvasRenderingContext2d) -> Result<(), JsValue> {
        self.game_boy.run();
        let mut frame = self.frame.take().unwrap();
        transform(&self.game_boy.ppu_buffer(), &mut frame);
        let data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut frame), 160, 144)?;
        self.frame.replace(frame);
        context.put_image_data(&data, 0.0, 0.0)?;
        Ok(())
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
            self.game_boy.button_up(button)
        }
    }

    pub fn button_down(&mut self, key: &str) {
        if let Some(button) = match_key_to_button(&key) {
            self.game_boy.button_down(button)
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
