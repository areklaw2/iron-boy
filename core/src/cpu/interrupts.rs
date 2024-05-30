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
    pub fn update_ime(&mut self) {
        self.set_di = match self.set_di {
            2 => 1,
            1 => {
                self.interrupt_master_enable = false;
                0
            }
            _ => 0,
        };
        self.set_ei = match self.set_ei {
            2 => 1,
            1 => {
                self.interrupt_master_enable = true;
                0
            }
            _ => 0,
        };
    }

    pub fn handle_interrupt(&mut self) -> u8 {
        if !self.interrupt_master_enable && !self.halted {
            return 0;
        }

        let mut interrupt_flag = self.bus.mem_read(IF_ADDRESS);
        let interrupt_enable = self.bus.mem_read(IE_ADDRESS);
        let requested_interrupt = interrupt_flag & interrupt_enable;
        if requested_interrupt == 0 {
            return 0;
        }

        self.halted = false;
        if !self.interrupt_master_enable {
            return 0;
        }
        self.interrupt_master_enable = false;

        let interrupt = requested_interrupt.trailing_zeros();
        if interrupt >= 5 {
            panic!("Invalid interrupt triggered");
        }

        interrupt_flag &= !(1 << interrupt);
        self.bus.mem_write(IF_ADDRESS, interrupt_flag);

        let address = self.registers.pc;
        self.push_stack(address);
        self.registers.pc = 0x0040 | ((interrupt as u16) << 3);
        16
    }
}
