use std::{fs::File, io::Read};

use crate::bus::Memory;

use self::header::Header;

mod header;

pub struct Cartridge {
    header: Header,
    _filename: String,
    _rom_size: u32,
    buffer: Vec<u8>,
}

impl Default for Cartridge {
    fn default() -> Self {
        Cartridge {
            header: Header::default(),
            _filename: "".into(),
            _rom_size: 0,
            buffer: vec![0; 0xFFFF],
        }
    }
}

impl Memory for Cartridge {
    fn mem_read(&mut self, address: u16) -> u8 {
        // rom only for now
        return self.buffer[address as usize];
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        // no writes on rom only
        //self.buffer[address as usize] = data;
        return;
    }
}

impl Cartridge {
    pub fn load(filename: &str) -> Result<Cartridge, &'static str> {
        let mut rom = File::open(filename).expect("Unable to open file");
        let mut buffer = Vec::new();
        rom.read_to_end(&mut buffer).expect("Issue while reading file");

        let header = Header::load(&buffer[0x000..=0x014F]);
        let cartridge = Cartridge {
            header,
            _filename: String::from(filename),
            _rom_size: buffer.len() as u32,
            buffer: buffer,
        };

        let mut checksum: u8 = 0;
        for address in 0x0134..=0x014C {
            checksum = checksum.wrapping_sub(cartridge.buffer[address]).wrapping_sub(1)
        }

        match checksum == cartridge.header.checksum {
            true => Ok(()),
            false => Err("Cartridge checksum not valid"),
        }?;

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

        let mut checksum: u8 = 0;
        for address in 0x0134..=0x014C {
            checksum = checksum.wrapping_sub(self.buffer[address]).wrapping_sub(1)
        }

        println!(
            "\t Checksum : {:#02X} {}",
            self.header.checksum,
            if checksum == self.header.checksum { "PASSED" } else { "FAILED" }
        );
    }
}

fn verify_checksum(data: &[u8]) -> bool {
    let mut checksum: u8 = 0;
    for address in 0x0134..=0x014C {
        checksum = checksum.wrapping_sub(data[address]).wrapping_sub(1)
    }
    data[0x14D] == checksum
}
