mod apu;
mod cartridge;
pub mod cpu;
mod dma;
pub mod gb;
mod interrupts;
mod joypad;
pub mod memory;
mod ppu;
mod serial_transfer;
mod speed_switch;
pub mod system_bus;
mod timer;

pub use apu::SAMPLING_FREQUENCY;
pub use gb::GameBoy;
pub use joypad::JoypadButton;
pub use ppu::{FPS, VIEWPORT_HEIGHT, VIEWPORT_WIDTH};

pub const T_CYCLES_PER_STEP: u8 = 4;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GbMode {
    Monochrome,
    Color,
    ColorAsMonochrome,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum GbSpeed {
    Normal,
    Double,
}

pub fn t_cycles(speed: GbSpeed) -> u8 {
    match speed {
        GbSpeed::Double => T_CYCLES_PER_STEP / 2,
        GbSpeed::Normal => T_CYCLES_PER_STEP,
    }
}
