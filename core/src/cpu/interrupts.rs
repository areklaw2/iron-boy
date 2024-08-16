use super::Cpu;
use crate::bus::{MemoryAccess, IE_ADDRESS, IF_ADDRESS};

impl Cpu {
    pub fn update_interrupt_master_enable(&mut self) {
        self.disable_interrupt = match self.disable_interrupt {
            2 => 1,
            1 => {
                self.interrupt_master_enable = false;
                0
            }
            _ => 0,
        };

        self.enable_interrupt = match self.enable_interrupt {
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

        let mut interrupt_flag = self.read_8(IF_ADDRESS);
        let interrupt_enable = self.read_8(IE_ADDRESS);
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
        self.bus.write_8(IF_ADDRESS, interrupt_flag);

        let address = self.registers.pc;
        self.push_stack(address);
        self.registers.pc = 0x0040 | ((interrupt as u16) << 3);
        16
    }
}
