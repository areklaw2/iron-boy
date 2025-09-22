use crate::system_bus::SystemMemoryAccess;

use super::tile::{TILE_HEIGHT, TILE_WIDTH};

use bitfields::bitfield;

#[bitfield(u8)]
#[derive(Copy, Clone)]
pub struct BgMapAttributes {
    #[bits(3)]
    color_palette: u8,
    bank: bool,
    _reserved: bool,
    x_flip: bool,
    y_flip: bool,
    priority: bool,
}

pub struct Background {
    scx: u8,
    scy: u8,
}

impl Background {
    pub fn new() -> Self {
        Background { scx: 0, scy: 0 }
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

impl SystemMemoryAccess for Background {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            _ => panic!("Background does not handle read {:#04X}", address),
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0xFF42 => self.scy = value,
            0xFF43 => self.scx = value,
            _ => panic!("Background does not handle write {:#04X}", address),
        }
    }
}
