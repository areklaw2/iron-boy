pub const TILE_WIDTH: u8 = 8;
pub const TILE_HEIGHT: u8 = TILE_WIDTH;
const TILE_BYTES: u16 = 16;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileDataAddressingMode {
    High,
    Low,
}

impl TileDataAddressingMode {
    pub fn tile_address(self, tile_index: u8) -> u16 {
        match self {
            TileDataAddressingMode::Low => 0x8000 + (tile_index as u16 * TILE_BYTES),
            TileDataAddressingMode::High => {
                if tile_index < 128 {
                    0x9000 + (tile_index as u16 * TILE_BYTES)
                } else {
                    0x8800 + ((tile_index - 128) as u16 * TILE_BYTES)
                }
            }
        }
    }
}

impl From<bool> for TileDataAddressingMode {
    fn from(value: bool) -> Self {
        match value {
            true => TileDataAddressingMode::Low,
            false => TileDataAddressingMode::High,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TileMap {
    Low,
    High,
}

impl TileMap {
    pub fn base_address(self) -> u16 {
        match self {
            TileMap::Low => 0x9800,
            TileMap::High => 0x9C00,
        }
    }
}

impl From<bool> for TileMap {
    fn from(value: bool) -> Self {
        match value {
            true => TileMap::High,
            false => TileMap::Low,
        }
    }
}
