use utils::Mode;

use crate::bus::Memory;

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
    hdma1: u8,
    hdma2: u8,
    hdma3: u8,
    hdma4: u8,
    hdma5: u8,
    bgpi: u8,
    bgpd: u8,
    obpi: u8,
    obpd: u8,
    opri: u8,
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
            hdma1: 0,
            hdma2: 0,
            hdma3: 0,
            hdma4: 0,
            hdma5: 0,
            bgpi: 0,
            bgpd: 0,
            obpi: 0,
            obpd: 0,
            opri: 0,
            mode: Mode::Monochrome,
            interrupt: 0,
        }
    }
}

impl Memory for Ppu {
    fn mem_read(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => self.vram[(self.vram_bank * 0x2000) | (address as usize & 0x1FFF)],
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
            0xFF4D => self.key1,
            0xFF4F..=0xFF6B if self.mode != Mode::Color => 0xFF,
            0xFF4F => self.vram_bank as u8,
            0xFF55 => self.hdma5,
            0xFF68 => self.bgpi,
            0xFF69 => self.bgpd,
            0xFF6A => self.obpi,
            0xFF6B => self.obpd,
            0xFF6C => self.opri,
            _ => panic!("PPU does not handle read to address {:4X}", address),
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        match address {
            0x8000..=0x9FFF => self.vram[(self.vram_bank * 0x2000) | (address as usize & 0x1FFF)] = data,
            0xFF40 => self.lcdc = data,
            0xFF41 => self.stat = data,
            0xFF42 => self.scy = data,
            0xFF43 => self.scx = data,
            0xFF45 => self.lyc = data,
            0xFF46 => self.dma = data,
            0xFF47 => self.bgp = data,
            0xFF48 => self.obp0 = data,
            0xFF49 => self.obp1 = data,
            0xFF4A => self.wy = data,
            0xFF4B => self.wx = data,
            0xFF4D => self.key1 = data,
            0xFF4F..=0xFF6B if self.mode != Mode::Color => {}
            0xFF4F => self.vram_bank = (data) as usize,
            0xFF51 => self.hdma1 = data,
            0xFF52 => self.hdma1 = data,
            0xFF53 => self.hdma1 = data,
            0xFF54 => self.hdma1 = data,
            0xFF55 => self.hdma1 = data,
            0xFF68 => self.bgpi = data,
            0xFF69 => self.bgpd = data,
            0xFF6A => self.obpi = data,
            0xFF6B => self.obpd = data,
            0xFF6C => self.opri = data,
            _ => panic!("PPU does not handle write to address {:4X}", address),
        }
    }
}
