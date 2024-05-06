const RESTART_AND_INTERUPT_VECTORS: (u16, u16) = (0x0000, 0x00FF);
const CARTRIDGE_HEADER_AREA: (u16, u16) = (0x0100, 0x014F);
const CARTRIDGE_ROM_FIXED_BANK: (u16, u16) = (0x0150, 0x3FFF); // 0
const CARTRIDGE_ROM_SWITCHABLE_BANKS: (u16, u16) = (0x4000, 0x7FFF); // 1 - NN
const CHARACTER_ROM: (u16, u16) = (0x8000, 0x97FF);
const BG_MAP_DATA_1: (u16, u16) = (0x9800, 0x9BFF);
const BG_MAP_DATA_2: (u16, u16) = (0x9C00, 0x9FFF);
const CARTRIDGE_RAM: (u16, u16) = (0xA000, 0xBFFF);
const INTERNAL_RAM_FIXED: (u16, u16) = (0xC000, 0xCFFF); // 0
const INTERNAL_RAM_SWITCHABLE: (u16, u16) = (0xD000, 0xDFFF); // 1 - 7
const ECHO_RAM_RESERVED_DO_NOT_USE: (u16, u16) = (0xE000, 0xFDFF);
const OBJECT_ATTRIBUTE_MEMORY: (u16, u16) = (0xFE00, 0xFE9F);
const UNUSABLE_MEMORY: (u16, u16) = (0xFEA0, 0xFEFF);
const IO_REGISTERS: (u16, u16) = (0xFF00, 0xFF7F);
const ZERO_PAGE: (u16, u16) = (0xFF80, 0xFFFE); // Zero Page
const INTERRUPT_ENABLE_FLAG: u16 = 0xFFFF;

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

pub struct Mmu {
    interrupt_flag: u8,
    interrupt_enable: u8,
    memory: [u8; 0xFFFF],
}

impl Mmu {
    pub fn new() -> Self {
        Mmu { interrupt_flag: 0, interrupt_enable: 0, memory: [0; 0xFFFF] }
    }
}

impl Memory for Mmu {
    fn mem_read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x00FF => todo!(),
            0x0100..=0x014F => todo!(),
            0x0150..=0x3FFF => todo!(),
            0x4000..=0x7FFF => todo!(),
            0x8000..=0x97FF => todo!(),
            0x9800..=0x9BFF => todo!(),
            0x9C00..=0x9FFF => todo!(),
            0xA000..=0xBFFF => todo!(),
            0xC000..=0xCFFF => todo!(),
            0xD000..=0xDFFF => todo!(),
            0xE000..=0xFDFF => todo!(),
            0xFE00..=0xFE9F => todo!(),
            0xFEA0..=0xFEFF => todo!(),
            0xFF0F => self.interrupt_flag,
            0xFF00..=0xFF7F => todo!(), // This will get broken up
            0xFF80..=0xFFFE => todo!(),
            0xFFFF => self.interrupt_enable,
            _ => 0xFF,
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0x00FF => todo!(),
            0x0100..=0x014F => todo!(),
            0x0150..=0x3FFF => todo!(),
            0x4000..=0x7FFF => todo!(),
            0x8000..=0x97FF => todo!(),
            0x9800..=0x9BFF => todo!(),
            0x9C00..=0x9FFF => todo!(),
            0xA000..=0xBFFF => todo!(),
            0xC000..=0xCFFF => todo!(),
            0xD000..=0xDFFF => todo!(),
            0xE000..=0xFDFF => todo!(),
            0xFE00..=0xFE9F => todo!(),
            0xFEA0..=0xFEFF => todo!(),
            0xFF0F => self.interrupt_flag = data,
            0xFF00..=0xFF7F => todo!(), // This will get broken up
            0xFF80..=0xFFFE => todo!(),
            0xFFFF => self.interrupt_enable = data,
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mem_read_write() {
        let mut bus = Mmu::new();
        bus.mem_write(0x01, 0x55);
        assert_eq!(bus.mem_read(0x01), 0x55);
    }
}
