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
