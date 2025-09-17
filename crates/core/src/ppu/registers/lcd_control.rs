use getset::CopyGetters;

use crate::ppu::tile::{TileDataAddressingMode, TileMap};

#[derive(Copy, Clone, CopyGetters)]
pub struct LcdControl {
    #[getset(get_copy = "pub")]
    lcd_enabled: bool,
    #[getset(get_copy = "pub")]
    window_tile_map: TileMap,
    #[getset(get_copy = "pub")]
    window_enabled: bool,
    #[getset(get_copy = "pub")]
    tile_data_addressing_mode: TileDataAddressingMode,
    #[getset(get_copy = "pub")]
    bg_tile_map: TileMap,
    #[getset(get_copy = "pub")]
    object_size: bool,
    #[getset(get_copy = "pub")]
    object_enabled: bool,
    #[getset(get_copy = "pub")]
    bg_window_enabled: bool,
}

impl LcdControl {
    pub fn new() -> Self {
        LcdControl {
            lcd_enabled: true,
            window_tile_map: TileMap::Low,
            window_enabled: false,
            tile_data_addressing_mode: TileDataAddressingMode::Low,
            bg_tile_map: TileMap::Low,
            object_size: false,
            object_enabled: false,
            bg_window_enabled: true,
        }
    }
}

impl From<&LcdControl> for u8 {
    fn from(lcd_control: &LcdControl) -> Self {
        (lcd_control.lcd_enabled as u8) << 7
            | (lcd_control.window_tile_map as u8) << 6
            | (lcd_control.window_enabled as u8) << 5
            | (lcd_control.tile_data_addressing_mode as u8) << 4
            | (lcd_control.bg_tile_map as u8) << 3
            | (lcd_control.object_size as u8) << 2
            | (lcd_control.object_enabled as u8) << 1
            | (lcd_control.bg_window_enabled as u8)
    }
}

impl From<u8> for LcdControl {
    fn from(value: u8) -> Self {
        LcdControl {
            lcd_enabled: (value & 0x80) != 0,
            window_tile_map: ((value & 0x40) != 0).into(),
            window_enabled: (value & 0x20) != 0,
            tile_data_addressing_mode: ((value & 0x10) != 0).into(),
            bg_tile_map: ((value & 0x08) != 0).into(),
            object_size: (value & 0x04) != 0,
            object_enabled: (value & 0x02) != 0,
            bg_window_enabled: (value & 0x01) != 0,
        }
    }
}
