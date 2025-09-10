use getset::{CopyGetters, Getters, MutGetters, Setters};

use crate::memory::MemoryInterface;

pub const IF_ADDRESS: u16 = 0xFF0F;
pub const IE_ADDRESS: u16 = 0xFFFF;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptKind {
    VBlank = 0,
    Lcd = 1,
    Timer = 2,
    Serial = 3,
    Joypad = 4,
}

impl InterruptKind {
    pub fn source_address(self) -> u16 {
        0x0040 | ((self as u16) << 3)
    }

    pub fn bit_mask(self) -> u8 {
        1 << (self as u8)
    }
}

#[derive(Getters, MutGetters, CopyGetters, Setters)]
pub struct Interrupts {
    #[getset(get_copy = "pub", set = "pub")]
    interrupt_master_enable: bool,
    ei: u8,
    di: u8,
}

impl Interrupts {
    pub fn new() -> Self {
        Interrupts {
            interrupt_master_enable: false,
            ei: 0,
            di: 0,
        }
    }

    pub fn set_ei(&mut self) {
        self.ei = 2;
    }

    pub fn set_di(&mut self) {
        self.di = 2;
    }

    pub fn update_interrupt_master_enable(&mut self) {
        if self.di == 1 {
            self.interrupt_master_enable = false;
        }
        self.di = self.di.saturating_sub(1);

        if self.ei == 1 {
            self.interrupt_master_enable = true;
        }
        self.ei = self.ei.saturating_sub(1);
    }

    pub fn handle_interrupt<I: MemoryInterface>(&mut self, bus: &mut I) -> Option<u16> {
        let interrupt_flag = bus.load_8(IF_ADDRESS);
        let interrupt_enable = bus.load_8(IE_ADDRESS);
        let requested_interrupt = interrupt_flag & interrupt_enable & 0x1F;

        if !self.interrupt_master_enable || requested_interrupt == 0 {
            return None;
        }

        self.interrupt_master_enable = false;
        let interrupt_bit = requested_interrupt.trailing_zeros() as u8;
        let interrupt_kind = match interrupt_bit {
            0 => InterruptKind::VBlank,
            1 => InterruptKind::Lcd,
            2 => InterruptKind::Timer,
            3 => InterruptKind::Serial,
            4 => InterruptKind::Joypad,
            _ => return None,
        };

        let updated_flags = interrupt_flag & !interrupt_kind.bit_mask();
        bus.store_8(IF_ADDRESS, updated_flags);

        Some(interrupt_kind.source_address())
    }

    pub fn pending_interrupt<I: MemoryInterface>(&self, bus: &I) -> bool {
        let interrupt_flag = bus.load_8(IF_ADDRESS);
        let interrupt_enable = bus.load_8(IE_ADDRESS);
        let requested_interrupt = interrupt_flag & interrupt_enable & 0x1F;
        requested_interrupt != 0
    }
}
