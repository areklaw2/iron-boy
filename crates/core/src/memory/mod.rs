use crate::{GbSpeed, T_CYCLES_PER_STEP};

pub mod system_bus;

pub trait MemoryInterface {
    fn load_8(&mut self, address: u16, with_cycles: bool) -> u8;

    fn load_16(&mut self, address: u16, with_cycles: bool) -> u16 {
        let lo = self.load_8(address, with_cycles) as u16;
        let hi = self.load_8(address + 1, with_cycles) as u16;
        hi << 8 | lo
    }

    fn store_8(&mut self, address: u16, value: u8, with_cycles: bool);

    fn store_16(&mut self, address: u16, value: u16, with_cycles: bool) {
        self.store_8(address, (value & 0xFF) as u8, with_cycles);
        self.store_8(address + 1, (value >> 8) as u8, with_cycles);
    }

    fn m_cycle(&mut self);

    fn total_m_cycles(&self) -> u64;

    fn pending_interrupt(&self) -> u8;

    fn clear_interrupt(&mut self, mask: u8);

    fn change_speed(&mut self);

    fn speed(&self) -> GbSpeed;
}

pub trait SystemMemoryAccess {
    fn read_8(&self, address: u16) -> u8;

    fn write_8(&mut self, address: u16, value: u8);
}

pub fn t_cycles(speed: GbSpeed) -> u8 {
    match speed {
        GbSpeed::Double => T_CYCLES_PER_STEP / 2,
        GbSpeed::Normal => T_CYCLES_PER_STEP,
    }
}
