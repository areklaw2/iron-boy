use super::{Cpu, IE_ADDRESS, IF_ADDRESS};
use crate::bus::Memory;

#[derive(Debug)]
pub enum Interrupt {
    VBlank = 0b00001,
    LCD = 0b00010,
    Timer = 0b00100,
    Serial = 0b01000,
    Joypad = 0b10000,
}

impl Cpu {
    fn request_interrupt(&mut self) {}

    pub fn handle_interrupt(&mut self) -> u8 {
        let mut interrupt_flag = self.bus.mem_read(IF_ADDRESS);
        let interrupt_enable = self.bus.mem_read(IE_ADDRESS);
        let requested_interrupt = interrupt_flag & interrupt_enable;
        if requested_interrupt == 0 {
            return 0;
        }

        self.halted = false;
        self.interrupt_master_enable = false;

        // may not need this later
        interrupt_flag &= !(requested_interrupt);
        self.bus.mem_write(IF_ADDRESS, interrupt_flag);

        let address = self.registers.pc;
        self.push_stack(address);
        let interrupt_address = match requested_interrupt {
            0b00001 => 0x0040 | Interrupt::VBlank as u16,
            0b00010 => 0x0040 | Interrupt::LCD as u16,
            0b00100 => 0x0040 | Interrupt::Timer as u16,
            0b01000 => 0x0040 | Interrupt::Serial as u16,
            0b10000 => 0x0040 | Interrupt::Joypad as u16,
            _ => panic!("Not a valid interrupt"),
        };
        self.registers.pc = interrupt_address;
        20
    }
}
