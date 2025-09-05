pub mod system_bus;

pub trait MemoryInterface {
    fn load_8(&mut self, address: u16) -> u8;

    fn load_16(&mut self, address: u16) -> u16 {
        let lo = self.load_8(address) as u16;
        let hi = self.load_8(address + 1) as u16;
        hi << 8 | lo
    }

    fn store_8(&mut self, address: u16, value: u8);

    fn store_16(&mut self, address: u16, value: u16) {
        let hi = (value >> 8) as u8;
        let lo = (value & 0xFF) as u8;
        self.store_8(address, lo);
        self.store_8(address + 1, hi);
    }

    fn cycle(&mut self, cycles: u32, cpu_halted: bool) -> u32;

    fn change_speed(&mut self);
}

pub trait SystemMemoryAccess {
    fn read_8(&mut self, address: u16) -> u8;

    fn write_8(&mut self, address: u16, value: u8);
}
