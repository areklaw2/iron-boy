use std::{cell::RefCell, rc::Rc};

use getset::{Getters, Setters};

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
}

#[derive(Getters, Setters)]
pub struct Interrupts {
    #[getset(get = "pub", set = "pub")]
    interrupt_flag: Rc<RefCell<u8>>,
    #[getset(get = "pub", set = "pub")]
    interrupt_enable: u8,
}

impl Interrupts {
    pub fn new(interrupt_flag: Rc<RefCell<u8>>) -> Self {
        Interrupts {
            interrupt_flag,
            interrupt_enable: 0,
        }
    }
}
