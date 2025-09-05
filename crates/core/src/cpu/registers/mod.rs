use flags::Flags;

use crate::{GameBoyMode, get_set};

pub mod flags;

#[derive(Debug)]
pub struct Registers {
    a: u8,
    f: Flags,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
}

impl Registers {
    pub fn new(mode: GameBoyMode) -> Self {
        match mode {
            GameBoyMode::Monochrome => Registers {
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
            GameBoyMode::Color => Registers {
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
            GameBoyMode::ColorAsMonochrome => Registers {
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

    get_set!(a, set_a, u8);
    get_set!(b, set_b, u8);
    get_set!(c, set_c, u8);
    get_set!(d, set_d, u8);
    get_set!(e, set_e, u8);
    get_set!(h, set_h, u8);
    get_set!(l, set_l, u8);
    get_set!(pc, set_pc, u16);
    get_set!(sp, set_sp, u16);

    pub fn f(&mut self) -> &mut Flags {
        &mut self.f
    }

    pub fn set_f(&mut self, flags: Flags) {
        self.f = flags
    }

    pub fn af(&self) -> u16 {
        (self.a as u16) << 8 | u8::from(&self.f) as u16
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
