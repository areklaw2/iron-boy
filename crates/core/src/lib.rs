mod apu;
mod cartridge;
pub mod cpu;
pub mod gb;
pub mod interrupts;
mod joypad;
pub mod memory;
mod ppu;
mod serial_transfer;
mod timer;
mod utils;

pub use apu::{SAMPLING_FREQUENCY, SAMPLING_RATE};
pub use gb::GameBoy;
pub use joypad::JoypadButton;
pub use ppu::{FPS, VIEWPORT_HEIGHT, VIEWPORT_WIDTH};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GameBoyMode {
    Monochrome,
    Color,
    ColorAsMonochrome,
}
