use crate::bus::Memory;

pub struct Apu {}

impl Memory for Apu {
    fn mem_read(&mut self, address: u16) -> u8 {
        todo!()
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        todo!()
    }
}

impl Apu {
    pub fn new() -> Self {
        Apu {}
    }
}
