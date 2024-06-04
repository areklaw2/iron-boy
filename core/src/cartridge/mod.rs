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

    fn mem_write(&mut self, _address: u16, _data: u8) {
        // no writes on rom only
        //self.buffer[address as usize] = data;
        return;
    }
}

impl Cartridge {
    pub fn load(filename: &str) -> Self {
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

        println!("Cartridge Loaded:");
        println!("\t Title    : {}", cartridge.header.title);
        println!(
            "\t Type     : {:#04X}, {}",
            cartridge.header.cartridge_type,
            cartridge.header.get_cartridge_type()
        );
        println!("\t ROM Size : {} KB", 32 << cartridge.header.rom_size);
        println!("\t RAM Size : {:#04X}", cartridge.header.ram_size);
        println!(
            "\t LIC Code : {:#04X} {}",
            cartridge.header.old_licensee_code,
            cartridge.header.get_license_code()
        );
        println!("\t ROM Vers : {:#04X}", cartridge.header.version);

        let mut checksum: u8 = 0;
        for address in 0x0134..=0x014C {
            checksum = checksum.wrapping_sub(cartridge.buffer[address]).wrapping_sub(1)
        }

        println!(
            "\t Checksum : {:#02X} {}",
            cartridge.header.checksum,
            if checksum == cartridge.header.checksum { "PASSED" } else { "FAILED" }
        );

        cartridge
    }
}
