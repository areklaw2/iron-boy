use core::GameBoy;
use std::{fs::File, io::Read};

use sdl2::Sdl;

pub struct Desktop {
    pub sdl_context: Sdl,
    pub game_boy: GameBoy,
}

impl Desktop {
    pub fn new(rom_path: String) -> anyhow::Result<Desktop> {
        let desktop = Self {
            sdl_context: sdl2::init().unwrap(),
            game_boy: GameBoy::new(&rom_path, read_rom(&rom_path)),
        };

        Ok(desktop)
    }
}

fn read_rom(rom_path: &str) -> Vec<u8> {
    let mut rom = File::open(rom_path).expect("Unable to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).expect("Issue while reading file");
    buffer
}
