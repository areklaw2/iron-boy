use bitfields::bitfield;

use crate::ppu::tile::{TileDataArea, TileMap};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PpuMode {
    HBlank = 0,
    VBlank = 1,
    OamScan = 2,
    DrawingPixels = 3,
}

impl PpuMode {
    pub const fn from_bits(bits: u8) -> Self {
        use PpuMode::*;
        match bits {
            0 => HBlank,
            1 => VBlank,
            2 => OamScan,
            _ => DrawingPixels,
        }
    }

    pub const fn into_bits(self) -> u8 {
        self as u8
    }
}

#[bitfield(u8, order = msb)]
#[derive(Copy, Clone)]
pub struct LcdStatus {
    _reserved: bool,
    lyc_interrupt: bool,
    mode2_interrupt: bool,
    mode1_interrupt: bool,
    mode0_interrupt: bool,
    lyc_equals_ly: bool,
    #[bits(2)]
    mode: PpuMode,
}

#[bitfield(u8, order = msb)]
#[derive(Copy, Clone)]
pub struct LcdControl {
    lcd_enabled: bool,
    #[bits(1)]
    window_tile_map: TileMap,
    window_enabled: bool,
    #[bits(1)]
    tile_data_area: TileDataArea,
    #[bits(1)]
    bg_tile_map: TileMap,
    object_size: bool,
    object_enabled: bool,
    bg_window_enabled: bool,
}
