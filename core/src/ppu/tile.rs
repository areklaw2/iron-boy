#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileData {
    Area0,
    Area1,
}

impl TileData {
    pub fn tile_address(self, tile_index: u8) -> u16 {
        match self {
            TileData::Area0 => (0x1000 + (((tile_index as i8) as i16) * 16)) as u16,
            TileData::Area1 => tile_index as u16 * 16,
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
            TileMap::Low => 0x1800,
            TileMap::High => 0x1C00,
        }
    }
}
