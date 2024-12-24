mod apu;
mod cartridge;
mod cpu;
pub mod gb;
mod io;
mod memory;
mod ppu;

pub use crate::apu::{AUDIO_BUFFER_THRESHOLD, SAMPLING_FREQUENCY, SAMPLING_RATE};
pub use crate::io::joypad::JoypadButton;
pub use crate::ppu::{FPS, VIEWPORT_HEIGHT, VIEWPORT_WIDTH};

pub trait Component {
    fn cycle(&mut self, cycles: u32);
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameBoyMode {
    Monochrome,
    Color,
    ColorAsMonochrome,
}
