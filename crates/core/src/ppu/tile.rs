use crate::ppu::FULL_WIDTH;

pub const TILE_WIDTH: u8 = 8;
pub const TILE_HEIGHT: u8 = TILE_WIDTH;
const TILE_BYTES: u16 = 16;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileDataArea {
    High,
    Low,
}

impl TileDataArea {
    pub fn tile_address(self, tile_index: u8) -> u16 {
        match self {
            TileDataArea::Low => 0x8000 + (tile_index as u16 * TILE_BYTES),
            TileDataArea::High => {
                if tile_index < 128 {
                    0x9000 + (tile_index as u16 * TILE_BYTES)
                } else {
                    0x8800 + ((tile_index - 128) as u16 * TILE_BYTES)
                }
            }
        }
    }

    pub const fn from_bits(bits: u8) -> Self {
        use TileDataArea::*;
        match bits {
            0 => High,
            1 => Low,
            _ => unreachable!(),
        }
    }

    pub const fn into_bits(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TileMap {
    Low,
    High,
}

impl TileMap {
    pub fn tile_index_address(&self, x: u8, y: u8) -> u16 {
        let base_address = match self {
            TileMap::Low => 0x9800,
            TileMap::High => 0x9C00,
        };

        let tiles_per_row: u16 = (FULL_WIDTH / TILE_WIDTH as usize) as u16;
        let tile_x = (x as u16) / TILE_WIDTH as u16;
        let tile_y = (y as u16) / TILE_HEIGHT as u16;
        let offset = tile_y * tiles_per_row + tile_x;
        base_address + offset
    }

    pub const fn from_bits(bits: u8) -> Self {
        use TileMap::*;
        match bits {
            0 => Low,
            1 => High,
            _ => unreachable!(),
        }
    }

    pub const fn into_bits(self) -> u8 {
        self as u8
    }
}
