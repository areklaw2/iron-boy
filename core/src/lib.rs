pub mod apu;
mod boot_rom;
pub mod bus;
pub mod cartridge;
pub mod cpu;
mod io;
mod ppu;

pub use crate::io::joypad::JoypadButton;
pub use crate::ppu::{FPS, VIEWPORT_HEIGHT, VIEWPORT_WIDTH};

pub trait Component {
    fn cycle(&mut self, ticks: u32);
}

#[derive(PartialEq, Copy, Clone)]
pub enum GameBoyMode {
    Monochrome,
    Color,
    ColorAsMonochrome,
}

pub enum Speed {
    Single,
    Double,
}
