use std::str::from_utf8;

use crate::GbMode;

// May use these at some point
#[allow(dead_code)]
pub struct Header {
    entry: [u8; 4],
    logo: [u8; 48],
    title: String,
    cgb_flag: u8,
    new_licensee_code: [u8; 2],
    sgb_flag: u8,
    pub cartridge_type: u8,
    rom_size: u8,
    ram_size: u8,
    destination_code: u8,
    old_licensee_code: u8,
    version: u8,
    pub checksum: u8,
    global_checksum: u16,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            entry: [0; 4],
            logo: [0; 48],
            title: Default::default(),
            cgb_flag: 0,
            new_licensee_code: [0; 2],
            sgb_flag: 0,
            cartridge_type: 0,
            rom_size: 0,
            ram_size: 0,
            destination_code: 0,
            old_licensee_code: 0,
            version: 0,
            checksum: 0,
            global_checksum: 0,
        }
    }
}

impl Header {
    pub fn load(bytes: &[u8]) -> Self {
        Header {
            entry: bytes[0x0100..=0x0103].try_into().unwrap(),
            logo: bytes[0x0104..=0x0133].try_into().unwrap(),
            title: from_utf8(&bytes[0x0134..=0x0143]).unwrap_or("NO NAME").to_owned(),
            cgb_flag: bytes[0x0143],
            new_licensee_code: bytes[0x0144..=0x0145].try_into().unwrap(),
            sgb_flag: bytes[0x0146],
            cartridge_type: bytes[0x0147],
            rom_size: bytes[0x0148],
            ram_size: bytes[0x0149],
            destination_code: bytes[0x014A],
            old_licensee_code: bytes[0x014B],
            version: bytes[0x014C],
            checksum: bytes[0x014D],
            global_checksum: (bytes[0x014E] as u16) << 8 | bytes[0x014F] as u16,
        }
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn mode(&self) -> GbMode {
        match self.cgb_flag {
            0xC0 => GbMode::Color,
            _ => GbMode::Monochrome,
        }
    }

    pub fn rom_banks(&self) -> usize {
        if self.rom_size <= 8 { 2 << self.rom_size } else { 0 }
    }

    pub fn ram_banks(&self) -> usize {
        match self.ram_size {
            0x1 | 0x2 => 1,
            0x3 => 4,
            0x4 => 16,
            0x5 => 8,
            _ => 0,
        }
    }

    pub fn has_ram(&self) -> bool {
        match self.cartridge_type {
            0x02 | 0x03 | 0x0F | 0x10 | 0x12 | 0x13 | 0x1A | 0x1B | 0x1D | 0x1E => true,
            _ => false,
        }
    }

    pub fn has_battery(&self) -> bool {
        match self.cartridge_type {
            0x03 | 0x06 | 0x0F | 0x10 | 0x13 | 0x1B | 0x1E => true,
            _ => false,
        }
    }

    pub fn has_real_time_clock(&self) -> bool {
        match self.cartridge_type {
            0x0F | 0x10 => true,
            _ => false,
        }
    }
}
