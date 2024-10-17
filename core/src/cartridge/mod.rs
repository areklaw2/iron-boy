use mbc1::Mbc1;
use no_mbc::NoMbc;

use self::header::Header;
use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};

mod header;
mod mbc1;
mod no_mbc;

pub trait MemoryBankController {
    fn rom_read(&self, address: u16) -> u8;
    fn rom_write(&mut self, address: u16, value: u8);
    fn ram_read(&self, address: u16) -> u8;
    fn ram_write(&mut self, address: u16, value: u8);
    fn ram_updated(&mut self) -> bool;
    fn has_battery(&self) -> bool;
    fn load_ram(&mut self, data: &[u8]) -> Result<(), &'static str>;
    fn dump_ram(&self) -> Vec<u8>;
}

pub struct Cartridge {
    pub mbc: Box<dyn MemoryBankController>,
    ram_file: PathBuf,
}

impl Default for Cartridge {
    fn default() -> Self {
        Cartridge {
            mbc: NoMbc::new(vec![0; 0xFFFF])
                .map(|mbc| Box::new(mbc) as Box<dyn MemoryBankController>)
                .unwrap(),
            ram_file: PathBuf::new(),
        }
    }
}

impl Cartridge {
    pub fn load(rom_file: PathBuf) -> Result<Cartridge, &'static str> {
        let mut rom = File::open(&rom_file).expect("Unable to open file");
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

        let mut mbc = match header.cartridge_type {
            0x00 => NoMbc::new(buffer).map(|mbc| Box::new(mbc) as Box<dyn MemoryBankController>),
            0x01..=0x03 => Mbc1::new(buffer, header.rom_banks(), header.ram_banks(), header.has_battery())
                .map(|mbc| Box::new(mbc) as Box<dyn MemoryBankController>),
            0x05..=0x06 => todo!("MBC2"),
            0x0F..=0x13 => todo!("MBC3"),
            0x19..=0x1E => todo!("MBC5"),
            _ => Err("Unsupported Cartridge type"),
        }?;

        let ram_file = rom_file.with_extension("sav");
        if mbc.has_battery() {
            match File::open(&ram_file) {
                Ok(mut file) => {
                    let mut value = Vec::new();
                    match file.read_to_end(&mut value) {
                        Err(..) => return Err("Error reading existing save"),
                        Ok(..) => {
                            mbc.load_ram(&value)?;
                        }
                    }
                }
                Err(ref error) if error.kind() == io::ErrorKind::NotFound => {}
                Err(_) => return Err("Error loading existing save"),
            }
        }

        let cartridge = Cartridge { mbc, ram_file };
        Ok(cartridge)
    }
}

impl Drop for Cartridge {
    fn drop(&mut self) {
        if self.mbc.has_battery() {
            let mut file = match File::create(&self.ram_file) {
                Ok(file) => file,
                Err(..) => return,
            };
            let _ = file.write_all(&self.mbc.dump_ram());
        }
    }
}
