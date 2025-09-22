use bitfields::bitfield;

use crate::system_bus::SystemMemoryAccess;

pub const OAM_SIZE: usize = 40;

#[bitfield(u8)]
#[derive(Copy, Clone)]
pub struct OamAttributes {
    #[bits(3)]
    cgb_palette: u8,
    bank: bool,
    dmg_palette: bool,
    x_flip: bool,
    y_flip: bool,
    priority: bool,
}

#[bitfield(u32)]
#[derive(Copy, Clone)]
pub struct OamEntry {
    y_position: u8,
    x_position: u8,
    tile_index: u8,
    #[bits(8)]
    attributes: OamAttributes,
}

pub struct Oam {
    data: [OamEntry; OAM_SIZE],
}

impl SystemMemoryAccess for Oam {
    fn read_8(&self, address: u16) -> u8 {
        let address = address - 0xFE00;
        match address {
            0x00..=0x9F => {
                let index = (address / 4) as usize;
                let oam_address = (address % 4) as usize;
                match oam_address {
                    0 => self.data[index].y_position(),
                    1 => self.data[index].x_position(),
                    2 => self.data[index].tile_index(),
                    3 => self.data[index].attributes().into(),
                    _ => unreachable!(),
                }
            }
            _ => panic!("OAM does not handle read {:#04X}", address),
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        let address = address - 0xFE00;
        match address {
            0x00..=0x9F => {
                let index = (address / 4) as usize;
                let oam_address = (address % 4) as usize;

                match oam_address {
                    0 => self.data[index].set_y_position(value),
                    1 => self.data[index].set_x_position(value),
                    2 => self.data[index].set_tile_index(value),
                    3 => self.data[index].set_attributes(value.into()),
                    _ => unreachable!(),
                }
            }
            _ => panic!("OAM does not handle write {:#04X}", address),
        }
    }
}

impl Oam {
    pub fn new() -> Self {
        Self {
            data: [OamEntry::new(); OAM_SIZE],
        }
    }

    pub fn oam_entry(&self, index: usize) -> OamEntry {
        self.data[index]
    }
}
