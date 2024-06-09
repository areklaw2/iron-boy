#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileMap {
    Low = 0x9800,
    High = 0x9C00,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileData {
    Block0 = 0x8000,
    Block1 = 0x8800,
}

impl TileData {
    pub fn address(self, tile: u8) -> u16 {
        match self {
            TileData::Block0 => (0x1000 + (((tile as i8) as i16) * 16)) as u16,
            TileData::Block1 => 0x0 + (tile as u16) * 16,
        }
    }
}
