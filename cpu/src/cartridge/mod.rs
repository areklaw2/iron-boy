use std::{fs::File, io::Read};

use crate::cartridge;

use self::cartridge_header::CartridgeHeader;

mod cartridge_header;

pub struct Cartridge {
    header: CartridgeHeader,
    filename: String,
    rom_size: u32,
    buffer: Vec<u8>,
}

impl Cartridge {
    pub fn load(filename: &str) -> Self {
        let mut rom = File::open(filename).expect("Unable to open file");
        let mut buffer = Vec::new();
        rom.read_to_end(&mut buffer).expect("Issue while reading file");

        let header = CartridgeHeader::load(&buffer[0x000..=0x014F]);
        let cartridge = Cartridge {
            header,
            filename: String::from(filename),
            rom_size: buffer.len() as u32,
            buffer: buffer,
        };

        println!("Cartridge Loaded:");
        println!("\t Title    : {}", cartridge.header.title);
        println!("\t Type     : {:#04X}, {}", cartridge.header.cartridge_type, cartridge.header.get_cartridge_type());
        println!("\t ROM Size : {} KB", 32 << cartridge.header.rom_size);
        println!("\t RAM Size : {:#04X}", cartridge.header.ram_size);
        println!("\t LIC Code : {:#04X} {}", cartridge.header.old_licensee_code, cartridge.header.get_license_code());
        println!("\t ROM Vers : {:#04X}", cartridge.header.version);

        let mut checksum: u16 = 0;
        for address in 0x0134..=0x014C {
            checksum -= (cartridge.buffer[address] - 1) as u16
        }

        println!(
            "\t Checksum : {:#02X} {}",
            cartridge.header.checksum,
            if checksum & 0xFF == 1 {
                "PASSED"
            } else {
                "FAILED"
            }
        );

        cartridge
    }
}
