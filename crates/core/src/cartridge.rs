use mbc1::Mbc1;
use mbc2::Mbc2;
use mbc3::Mbc3;
use mbc5::Mbc5;
use no_mbc::NoMbc;
use thiserror::Error;

use crate::{GbMode, system_bus::SystemMemoryAccess};

use self::header::Header;
use std::{
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
};

mod header;
mod mbc1;
mod mbc2;
mod mbc3;
mod mbc5;
mod no_mbc;
mod rtc;

pub trait MemoryBankController {
    fn read_rom(&self, address: u16) -> u8;
    fn write_rom(&mut self, address: u16, value: u8);
    fn read_ram(&self, address: u16) -> u8;
    fn write_ram(&mut self, address: u16, value: u8);
    fn load_ram(&mut self, data: &[u8]) -> Result<(), CartridgeError>;
    fn dump_ram(&self) -> Vec<u8>;
    fn has_battery(&self) -> bool;
}

pub struct Cartridge {
    mbc: Box<dyn MemoryBankController>,
    title: String,
    mode: GbMode,
    ram_file: PathBuf,
}

impl SystemMemoryAccess for Cartridge {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x7FFF => self.mbc.read_rom(address),
            0xA000..=0xBFFF => self.mbc.read_ram(address),
            _ => panic!("Cartridge does not handle read from address {:#4X}", address),
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x7FFF => self.mbc.write_rom(address, value),
            0xA000..=0xBFFF => self.mbc.write_ram(address, value),
            _ => panic!("Cartridge does not handle write to address {:#4X}", address),
        }
    }
}

impl Cartridge {
    pub fn load(rom_file: PathBuf, buffer: Vec<u8>) -> Result<Cartridge, CartridgeError> {
        let header = Header::load(&buffer[0x000..=0x014F])?;

        let mut checksum: u8 = 0;
        for address in 0x0134..=0x014C {
            checksum = checksum.wrapping_sub(buffer[address]).wrapping_sub(1)
        }

        match checksum == header.checksum() {
            true => Ok(()),
            false => Err(CartridgeError::CheckSumFailure),
        }?;

        let mut mbc = match header.cartridge_type() {
            0x00 => NoMbc::new(buffer).map(|mbc| Box::new(mbc) as Box<dyn MemoryBankController>),
            0x01..=0x03 => Mbc1::new(buffer, header.rom_banks(), header.ram_banks(), header.has_battery())
                .map(|mbc| Box::new(mbc) as Box<dyn MemoryBankController>),
            0x05..=0x06 => Mbc2::new(buffer, header.rom_banks(), header.has_battery()).map(|mbc| Box::new(mbc) as Box<dyn MemoryBankController>),
            0x0F..=0x13 => Mbc3::new(
                buffer,
                header.ram_banks(),
                header.has_ram(),
                header.has_battery(),
                header.has_real_time_clock(),
            )
            .map(|mbc| Box::new(mbc) as Box<dyn MemoryBankController>),
            0x19..=0x1E => Mbc5::new(buffer, header.rom_banks(), header.ram_banks(), header.has_battery())
                .map(|mbc| Box::new(mbc) as Box<dyn MemoryBankController>),
            _ => Err(CartridgeError::InvalidCatridgeType),
        }?;

        let ram_file = rom_file.with_extension("sav");
        if mbc.has_battery() {
            match File::open(&ram_file) {
                Ok(mut file) => {
                    let mut data = Vec::new();
                    match file.read_to_end(&mut data) {
                        Err(..) => return Err(CartridgeError::ReadInterrupted),
                        Ok(..) => {
                            mbc.load_ram(&data)?;
                        }
                    }
                }
                Err(ref error) if error.kind() == io::ErrorKind::NotFound || error.kind() == io::ErrorKind::Unsupported => {}
                Err(error) => return Err(CartridgeError::SaveFileFailure(format!("{}", error.kind()))),
            }
        }

        let cartridge = Cartridge {
            mbc,
            title: header.title().to_string(),
            mode: header.mode(),
            ram_file,
        };
        Ok(cartridge)
    }

    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    pub fn mode(&self) -> GbMode {
        self.mode
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

#[derive(Error, Debug)]
pub enum CartridgeError {
    #[error("Cartridge checksum not valid")]
    CheckSumFailure,
    #[error("Unsupported Cartridge type")]
    InvalidCatridgeType,
    #[error("Error reading save")]
    ReadInterrupted,
    #[error("Data with incorrect length being loaded")]
    IncorrectLengthLoaded,
    #[error("Save file failed with error: `{0}`")]
    SaveFileFailure(String),
    #[error("Invalid header data")]
    InvalidHeader,
}
