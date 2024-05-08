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

pub struct Bus {}

impl Memory for Bus {
    fn mem_read(&self, address: u16) -> u8 {
        0
    }

    fn mem_write(&mut self, address: u16, data: u8) {}
}

impl Bus {
    pub fn new() -> Self {
        Bus {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_read_write() {
        let mut bus = Bus::new();
        bus.mem_write(0x01, 0x55);
        assert_eq!(bus.mem_read(0x01), 0x55);
    }
}
