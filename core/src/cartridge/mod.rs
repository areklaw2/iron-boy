use no_mbc::NoMbc;

use self::header::Header;
use std::{fs::File, io::Read};

mod header;
mod no_mbc;

pub trait MemoryBankController: Send {
    fn rom_read(&self, address: u16) -> u8;
    fn rom_write(&mut self, address: u16, data: u8);
    fn ram_read(&self, address: u16) -> u8;
    fn ram_write(&mut self, address: u16, data: u8);
    fn check_and_reset_ram_updated(&mut self) -> bool;
    fn has_battery(&self) -> bool;
    fn load_ram(&mut self, data: &[u8]) -> Result<(), &'static str>;
    fn dump_ram(&self) -> Vec<u8>;
}

pub struct Cartridge {
    header: Header,
    pub mbc: Box<dyn MemoryBankController>,
}

impl Default for Cartridge {
    fn default() -> Self {
        Cartridge {
            header: Header::default(),
            mbc: NoMbc::new(vec![0; 0xFFFF])
                .map(|mbc| Box::new(mbc) as Box<dyn MemoryBankController>)
                .unwrap(),
        }
    }
}

impl Cartridge {
    pub fn load(filename: &str) -> Result<Cartridge, &'static str> {
        let mut rom = File::open(filename).expect("Unable to open file");
        let mut buffer = Vec::new();
        rom.read_to_end(&mut buffer).expect("Issue while reading file");

        let header = Header::load(&buffer[0x000..=0x014F]);

        let mut checksum: u8 = 0;
        for address in 0x0134..=0x014C {
            checksum = checksum.wrapping_sub(buffer[address]).wrapping_sub(1)
        }

        match checksum == header.checksum {
            true => Ok(()),
            false => Err("Cartridge checksum not valid"),
        }?;

        let mbc = match header.cartridge_type {
            0x00 => NoMbc::new(buffer).map(|mbc| Box::new(mbc) as Box<dyn MemoryBankController>),
            0x01..=0x03 => todo!(), //1
            0x05..=0x06 => todo!(), //2
            0x0F..=0x13 => todo!(), //3
            0x19..=0x1E => todo!(), //5
            _ => Err("Unsupported Cartridge type"),
        }?;

        let cartridge = Cartridge { header, mbc };
        Ok(cartridge)
    }

    fn rom_banks(&self) -> usize {
        if self.header.rom_size <= 8 {
            2 << self.header.rom_size
        } else {
            0
        }
    }

    fn ram_banks(&self) -> usize {
        match self.header.ram_size {
            0x1 => 1,
            0x2 => 1,
            0x3 => 4,
            0x4 => 16,
            0x5 => 8,
            _ => 0,
        }
    }

    pub fn debug_output(&self) {
        println!("Cartridge Loaded:");
        println!("\t Title    : {}", self.header.title);
        println!("\t Type     : {:#04X}, {}", self.header.cartridge_type, self.header.get_cartridge_type());
        println!("\t ROM Size : {} KB", 32 << self.header.rom_size);
        println!("\t RAM Size : {:#04X}", self.header.ram_size);
        println!("\t LIC Code : {:#04X} {}", self.header.old_licensee_code, self.header.get_license_code());
        println!("\t ROM Vers : {:#04X}", self.header.version);
        println!("\t Checksum : PASSED")
    }
}
