use crate::system_bus::SystemMemoryAccess;

const WRAM_SIZE: usize = 0x8000;
const HRAM_SIZE: usize = 0x007F;

pub struct Memory {
    wram_bank: usize,
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
}

impl SystemMemoryAccess for Memory {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0xC000..=0xCFFF | 0xE000..=0xEFFF => self.wram[address as usize & 0x0FFF],
            0xD000..=0xDFFF | 0xF000..=0xFDFF => self.wram[(self.wram_bank * 0x1000) | address as usize & 0x0FFF],
            0xFF70 => self.wram_bank as u8,
            0xFF80..=0xFFFE => self.hram[address as usize & 0x007F],
            _ => panic!("Memory does not handle read from address {:#4X}", address),
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0xC000..=0xCFFF | 0xE000..=0xEFFF => self.wram[address as usize & 0x0FFF] = value,
            0xD000..=0xDFFF | 0xF000..=0xFDFF => self.wram[(self.wram_bank * 0x1000) | address as usize & 0x0FFF] = value,
            0xFF70 => {
                self.wram_bank = match value & 0x7 {
                    0 => 1,
                    n => n as usize,
                };
            }
            0xFF80..=0xFFFE => self.hram[address as usize & 0x007F] = value,
            _ => panic!("Memory does not handle read to address {:#4X}", address),
        }
    }
}

impl Memory {
    pub fn new() -> Self {
        Self {
            wram_bank: 1,
            wram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
        }
    }
}
