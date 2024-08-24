use super::Cpu;
use crate::bus::MemoryAccess;

const IF_ADDRESS: u16 = 0xFF0F;
const IE_ADDRESS: u16 = 0xFFFF;

impl Cpu {
    pub fn update_ime(&mut self) {
        if self.disable_interrupt == 1 {
            self.interrupt_master_enable = false;
        }
        self.disable_interrupt = self.disable_interrupt.saturating_sub(1);

        if self.enable_interrupt == 1 {
            self.interrupt_master_enable = true;
        }
        self.enable_interrupt = self.enable_interrupt.saturating_sub(1);
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
        self.write_8(IF_ADDRESS, interrupt_flag);

        let address = self.registers.pc;
        self.push_stack(address);
        self.registers.pc = 0x0040 | ((interrupt as u16) << 3);
        16
    }
}
