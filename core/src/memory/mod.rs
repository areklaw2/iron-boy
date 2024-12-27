pub mod registers;
pub mod system_bus;

pub trait MemoryInterface {
    fn load_8(&self, address: u16) -> u8;

    fn load_16(&self, address: u16) -> u16 {
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

pub trait IoMemoryAccess {
    fn read_8(&self, address: u16) -> u8;

    fn write_8(&mut self, address: u16, value: u8);
}

pub struct SimpleBus {
    data: Vec<u8>,
}

impl SimpleBus {
    #[allow(dead_code)]
    pub fn new() -> SimpleBus {
        SimpleBus { data: vec![0; 0x10000] }
    }
}

impl MemoryInterface for SimpleBus {
    fn load_8(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn store_8(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value
    }

    fn cycle(&mut self, cycles: u32, _cpu_halted: bool) -> u32 {
        cycles
    }

    fn change_speed(&mut self) {}
}
