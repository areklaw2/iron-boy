use getset::{CopyGetters, MutGetters, Setters};

use crate::GbMode;

use bitfields::bitfield;

#[bitfield(u8, order = msb)]
#[derive(Copy, Clone)]
pub struct Flags {
    zero: bool,
    subtraction: bool,
    half_carry: bool,
    carry: bool,
    #[bits(4)]
    _reserved: u8,
}

#[derive(Debug, CopyGetters, MutGetters, Setters)]
pub struct Registers {
    #[getset(get_copy = "pub", set = "pub")]
    a: u8,
    #[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
    f: Flags,
    #[getset(get_copy = "pub", set = "pub")]
    b: u8,
    #[getset(get_copy = "pub", set = "pub")]
    c: u8,
    #[getset(get_copy = "pub", set = "pub")]
    d: u8,
    #[getset(get_copy = "pub", set = "pub")]
    e: u8,
    #[getset(get_copy = "pub", set = "pub")]
    h: u8,
    #[getset(get_copy = "pub", set = "pub")]
    l: u8,
    #[getset(get_copy = "pub", set = "pub")]
    pc: u16,
    #[getset(get_copy = "pub", set = "pub")]
    sp: u16,
}

impl Registers {
    pub fn new(mode: GbMode) -> Self {
        match mode {
            GbMode::Monochrome => Registers {
                a: 0x01,
                f: Flags::from(0b1011_0000),
                b: 0x00,
                c: 0x13,
                d: 0x00,
                e: 0xD8,
                h: 0x01,
                l: 0x4D,
                pc: 0x0100,
                sp: 0xFFFE,
            },
            GbMode::Color => Registers {
                a: 0x11,
                f: Flags::from(0b1000_0000),
                b: 0x00,
                c: 0x00,
                d: 0xFF,
                e: 0x56,
                h: 0x00,
                l: 0x0D,
                pc: 0x0100,
                sp: 0xFFFE,
            },
            GbMode::ColorAsMonochrome => Registers {
                a: 0x11,
                f: Flags::from(0b1000_0000),
                b: 0x00,
                c: 0x00,
                d: 0x00,
                e: 0x08,
                h: 0x00,
                l: 0x7C,
                pc: 0x0100,
                sp: 0xFFFE,
            },
        }
    }

    pub fn af(&self) -> u16 {
        (self.a as u16) << 8 | self.f.into_bits() as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = Flags::from((value & 0x00F0) as u8)
    }

    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0x00FF) as u8
    }

    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0x00FF) as u8
    }

    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0x00FF) as u8
    }

    pub fn decrement_hl(&mut self) -> u16 {
        let hl = self.hl();
        self.set_hl(hl - 1);
        hl
    }

    pub fn increment_hl(&mut self) -> u16 {
        let hl = self.hl();
        self.set_hl(hl + 1);
        hl
    }
}
