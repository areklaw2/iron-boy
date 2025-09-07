use bitfields::bitfield;
use getset::{Getters, MutGetters};

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

#[derive(Getters, MutGetters)]
pub struct InterruptController {
    #[getset(get = "pub", get_mut = "pub")]
    interrupt_flags: Interrupts,
    #[getset(get = "pub", get_mut = "pub")]
    interrupt_enable: Interrupts,
}

impl InterruptController {
    pub fn new() -> Self {
        InterruptController {
            interrupt_flags: Interrupts::from_bits(0),
            interrupt_enable: Interrupts::from_bits(0),
        }
    }
}
