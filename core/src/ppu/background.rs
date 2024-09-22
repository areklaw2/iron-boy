use super::tile::{TILE_HEIGHT, TILE_WIDTH};

pub struct Background {
    scx: u8,
    scy: u8,
}

impl Background {
    pub fn new() -> Self {
        Background { scx: 0, scy: 0 }
    }

    pub fn scx(&self) -> u8 {
        self.scx
    }

    pub fn set_scx(&mut self, value: u8) {
        self.scx = value;
    }

    pub fn scy(&self) -> u8 {
        self.scy
    }

    pub fn set_scy(&mut self, value: u8) {
        self.scy = value;
    }

    pub fn tile_map_coordinates(&self, lx: u8, ly: u8) -> (u8, u8) {
        let x = lx.wrapping_add(self.scx);
        let y = ly.wrapping_add(self.scy);
        (x, y)
    }

    pub fn pixel_offsets(&self, lx: u8, ly: u8) -> (u8, u8) {
        let x_offset = 7 - (lx % TILE_WIDTH);
        let y_offset = 2 * (ly % TILE_HEIGHT);
        (x_offset, y_offset)
    }
}
