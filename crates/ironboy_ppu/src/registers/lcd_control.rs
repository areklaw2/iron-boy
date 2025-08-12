use crate::tile::{TileDataAddressingMode, TileMap};

#[derive(Copy, Clone)]
pub struct LcdControl {
    lcd_enabled: bool,
    window_tile_map: TileMap,
    window_enabled: bool,
    tile_data_addressing_mode: TileDataAddressingMode,
    bg_tile_map: TileMap,
    object_size: bool,
    object_enabled: bool,
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

    pub fn lcd_enabled(&self) -> bool {
        self.lcd_enabled
    }

    pub fn window_tile_map(&self) -> TileMap {
        self.window_tile_map
    }

    pub fn window_enabled(&self) -> bool {
        self.window_enabled
    }

    pub fn tile_data(&self) -> TileDataAddressingMode {
        self.tile_data_addressing_mode
    }

    pub fn bg_tile_map(&self) -> TileMap {
        self.bg_tile_map
    }

    pub fn object_size(&self) -> bool {
        self.object_size
    }

    pub fn object_enabled(&self) -> bool {
        self.object_enabled
    }

    pub fn bg_window_enabled(&self) -> bool {
        self.bg_window_enabled
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
