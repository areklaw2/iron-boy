#[derive(Clone, Copy)]
pub enum TileMap {
    Low,
    High,
}

impl TileMap {
    fn base_offset(self) -> u16 {
        match self {
            TileMap::Low => 0x1800,
            TileMap::High => 0x1C00,
        }
    }
}

#[derive(Clone, Copy)]
pub enum TileData {
    Block0,
    Block1,
}

impl TileData {
    fn tile_address(self, tile: u8) -> u16 {
        match self {
            TileData::Block0 => (0x1000 + (((tile as i8) as i16) * 16)) as u16,
            TileData::Block1 => 0x0 + (tile as u16) * 16,
        }
    }
}
