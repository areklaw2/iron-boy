use crate::bus::Memory;

pub mod object_attribute;
pub mod palette;
pub mod tile;

const VRAM_SIZE: usize = 0x4000;
const OAM_SIZE: usize = 160;
pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

pub struct Ppu {
    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
    vram_bank: usize,
    //background: [[Tile; 32]; 32],
    //window: [[Tile; 32]; 32],
    pub interrupt: u8,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            vram_bank: 0,
            //background: [[Tile::default(); 32]; 32],
            //window: [[Tile::default(); 32]; 32],
            interrupt: 0,
        }
    }
}

impl Memory for Ppu {
    fn mem_read(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => self.vram[(self.vram_bank * 0x2000) | (address as usize & 0x1FFF)],
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00],
            0xFF40..=0xFF49 => 0xFF,
            0xFF4D..=0xFF4F => todo!("CGB registers for speed switch and VRAM bank select"),
            0xFF68..=0xFF6C => todo!("CGB registers for BF and OBJ palettes"),
            _ => panic!("PPU does not handle read to address {:4X}", address),
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        match address {
            0x8000..=0x9FFF => self.vram[(self.vram_bank * 0x2000) | (address as usize & 0x1FFF)] = data,
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00] = data,
            0xFF40..=0xFF49 => {}
            0xFF4D..=0xFF4F => todo!("CGB registers for speed switch and VRAM bank select"),
            0xFF68..=0xFF6C => todo!("CGB registers for BF and OBJ palettes"),
            _ => panic!("PPU does not handle write to address {:4X}", address),
        }
    }
}

impl Ppu {
    //pub fn cycle(&mut self, ticks: u32) {}
}
