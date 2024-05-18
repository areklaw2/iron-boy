use utils::{Mode, Speed};

use crate::cartridge::{self, Cartridge};

pub trait Memory {
    fn mem_read(&self, address: u16) -> u8;

    fn mem_read_16(&self, address: u16) -> u16 {
        let lo = self.mem_read(address) as u16;
        let hi = self.mem_read(address + 1) as u16;
        hi << 8 | lo
    }

    fn mem_write(&mut self, address: u16, data: u8);

    fn mem_write_16(&mut self, address: u16, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.mem_write(address, lo);
        self.mem_write(address + 1, hi);
    }
}

const WRAM_SIZE: usize = 0x8000;
const HRAM_SIZE: usize = 0x007F;

pub struct Bus {
    cartridge: Cartridge,
    wram: [u8; WRAM_SIZE],
    hram: [u8; HRAM_SIZE],
    wram_bank: usize,
    speed: Speed,
    speed_change_requested: bool,
}

// 0000	3FFF	16 KiB ROM bank 00	From cartridge, usually a fixed bank
// 4000	7FFF	16 KiB ROM Bank 01–NN	From cartridge, switchable bank via mapper (if any)
// 8000	9FFF	8 KiB Video RAM (VRAM)	In CGB mode, switchable bank 0/1
// A000	BFFF	8 KiB External RAM	From cartridge, switchable bank if any
// C000	CFFF	4 KiB Work RAM (WRAM)
// D000	DFFF	4 KiB Work RAM (WRAM)	In CGB mode, switchable bank 1–7
// E000	FDFF	Echo RAM (mirror of C000–DDFF)	Nintendo says use of this area is prohibited.
// FE00	FE9F	Object attribute memory (OAM)
// FEA0	FEFF	Not Usable	Nintendo says use of this area is prohibited.
// FF00	FF7F	I/O Registers
// FF80	FFFE	High RAM (HRAM)
// FFFF	FFFF	Interrupt Enable register (IE)

impl Memory for Bus {
    fn mem_read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.cartridge.mem_read(address),
            0x8000..=0x9FFF => todo!("Not implemented vram"),
            0xA000..=0xBFFF => self.cartridge.mem_read(address),
            0xC000..=0xCFFF | 0xE000..=0xEFFF => self.wram[address as usize & 0x0FFF],
            0xD000..=0xDFFF | 0xF000..=0xFDFF => self.wram[(self.wram_bank * 0x1000) | address as usize & 0x0FFF],
            0xFE00..=0xFE9F => todo!("Not implemented OAM"),
            0xFEA0..=0xFEFF => panic!("Reserved"),
            0xFF00..=0xFF7F => todo!("Not implemented I/O registers"),
            0xFF80..=0xFFFE => self.hram[address as usize & 0x007F],
            0xFFFF => todo!("Not implemented IE"),
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x7FFF => self.cartridge.mem_write(address, data),
            0x8000..=0x9FFF => todo!("Not implemented vram"),
            0xA000..=0xBFFF => self.cartridge.mem_write(address, data),
            0xC000..=0xCFFF | 0xE000..=0xEFFF => self.wram[address as usize & 0x0FFF] = data,
            0xD000..=0xDFFF | 0xF000..=0xFDFF => self.wram[(self.wram_bank * 0x1000) | address as usize & 0x0FFF] = data,
            0xFE00..=0xFE9F => todo!("Not implemented OAM"),
            0xFEA0..=0xFEFF => panic!("Reserved"),
            0xFF00..=0xFF7F => todo!("Not implemented I/O registers"),
            0xFF80..=0xFFFE => self.hram[address as usize & 0x007F] = data,
            0xFFFF => todo!("Not implemented IE"),
        }
    }
}

impl Bus {
    pub fn new(cartridge: Cartridge) -> Self {
        Bus {
            cartridge,
            wram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
            wram_bank: 1,
            speed: Speed::Normal,
            speed_change_requested: false,
        }
    }

    pub fn change_speed(&mut self) {
        if self.speed_change_requested {
            self.speed = match self.speed {
                Speed::Normal => Speed::Double,
                Speed::Double => Speed::Normal,
            }
        }
        self.speed_change_requested = false;
    }

    pub fn determine_mode(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_read_write() {
        // let mut bus = Bus::new(Cartridge::default());
        // bus.mem_write(0x01, 0x55);
        // assert_eq!(bus.mem_read(0x01), 0x55);
    }
}
