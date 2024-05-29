use crate::bus::Memory;
use utils::Mode;

pub mod tile;

const VRAM_SIZE: usize = 0x4000;
const OAM_SIZE: usize = 0x00A0;
const SCREEN_WIDTH: usize = 160;
const SCREEN_HEIGHT: usize = 144;

pub struct Ppu {
    vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
    lcdc: u8,
    stat: u8,
    scy: u8,
    scx: u8,
    ly: u8,
    lyc: u8,
    dma: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
    wy: u8,
    wx: u8,
    key1: u8,
    vram_bank: usize,
    mode: Mode,
    pub interrupt: u8,
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            lcdc: 0,
            stat: 0,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dma: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            wy: 0,
            wx: 0,
            key1: 0,
            vram_bank: 0,
            mode: Mode::Monochrome,
            interrupt: 0,
        }
    }
}

impl Memory for Ppu {
    fn mem_read(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => self.vram[(self.vram_bank * 0x2000) | (address as usize & 0x1FFF)],
            0xFE00..=0xFE9F => self.oam[address as usize],
            0xFF40 => self.lcdc,
            0xFF41 => self.stat,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => self.dma,
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            0xFF4D..=0xFF4F => todo!("CGB registers for speed switch and VRAM bank select"),
            0xFF68..=0xFF6C => todo!("CGB registers for BF and OBJ palettes"),
            _ => panic!("PPU does not handle read to address {:4X}", address),
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        match address {
            0x8000..=0x9FFF => self.vram[(self.vram_bank * 0x2000) | (address as usize & 0x1FFF)] = data,
            0xFE00..=0xFE9F => self.oam[address as usize] = data,
            0xFF40 => self.lcdc = data,
            0xFF41 => self.stat = data,
            0xFF42 => self.scy = data,
            0xFF43 => self.scx = data,
            //0xFF44 => self.ly = data,
            0xFF45 => self.lyc = data,
            0xFF46 => self.dma = data,
            0xFF47 => self.bgp = data,
            0xFF48 => self.obp0 = data,
            0xFF49 => self.obp1 = data,
            0xFF4A => self.wy = data,
            0xFF4B => self.wx = data,
            0xFF4D..=0xFF4F => todo!("CGB registers for speed switch and VRAM bank select"),
            0xFF68..=0xFF6C => todo!("CGB registers for BF and OBJ palettes"),
            _ => panic!("PPU does not handle write to address {:4X}", address),
        }
    }
}
