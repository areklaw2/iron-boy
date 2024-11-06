pub mod apu;
mod boot_rom;
pub mod bus;
mod cartridge;
pub mod cpu;
pub mod game_boy;
mod io;
mod ppu;

pub use crate::io::joypad::JoypadButton;
pub use crate::ppu::{FPS, VIEWPORT_HEIGHT, VIEWPORT_WIDTH};

pub trait Component {
    fn cycle(&mut self, ticks: u32);
}
