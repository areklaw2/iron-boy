use std::{cell::RefCell, rc::Rc};

use bitfields::bitfield;

pub const IF_ADDRESS: u16 = 0xFF0F;
pub const IE_ADDRESS: u16 = 0xFFFF;

#[bitfield(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Interrupts {
    #[bits(3)]
    _reserved: u8,
    joypad: bool,
    serial: bool,
    timer: bool,
    lcd: bool,
    vblank: bool,
}

pub struct InterruptController {
    interrupt_flags: Rc<RefCell<Interrupts>>,
    interrupt_enable: Interrupts,
}

impl InterruptController {
    pub fn new(interrupt_flags: Rc<RefCell<Interrupts>>) -> Self {
        InterruptController {
            interrupt_flags,
            interrupt_enable: Interrupts::from_bits(0),
        }
    }

    pub fn interrupt_flags(&self) -> u8 {
        self.interrupt_flags.borrow().into_bits() | 0b11100000
    }

    pub fn set_interrupt_flags(&mut self, value: u8) {
        self.interrupt_flags.borrow_mut().set_bits(value);
    }

    pub fn interrupt_enable(&self) -> u8 {
        self.interrupt_enable.into_bits()
    }

    pub fn set_interrupt_enable(&mut self, value: u8) {
        self.interrupt_enable.set_bits(value);
    }
}
