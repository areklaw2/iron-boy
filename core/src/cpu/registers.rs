use bitflags::bitflags;
use utils::Mode;

bitflags! {
    ///
    ///  7 6 5 4 3 2 1 0
    ///  Z N H C _ _ _ _
    ///  | | | |
    ///  | | | +----------- Carry Flag
    ///  | | +------------- Half Carry Flag (BCD)
    ///  | +--------------- Subtraction Flag (BCD)
    ///  +----------------- Zero Flag
    ///

    #[derive(Debug)]
    pub struct CpuFlag: u8 {
        const C = 0b0001_0000;
        const H = 0b0010_0000;
        const N = 0b0100_0000;
        const Z = 0b1000_0000;
    }
}

pub struct Registers {
    pub a: u8,
    pub f: CpuFlag,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16,
}

impl Registers {
    pub fn new(mode: Mode) -> Self {
        match mode {
            Mode::Monochrome => Registers {
                a: 0x01,
                f: CpuFlag::from_bits_truncate(0b1011_0000),
                b: 0x00,
                c: 0x13,
                d: 0x00,
                e: 0xD8,
                h: 0x01,
                l: 0x4D,
                pc: 0x0100,
                sp: 0xFFFE,
            },
            Mode::Color => Registers {
                a: 0x11,
                f: CpuFlag::from_bits_truncate(0b1000_0000),
                b: 0x00,
                c: 0x00,
                d: 0xFF,
                e: 0x56,
                h: 0x00,
                l: 0x0D,
                pc: 0x0100,
                sp: 0xFFFE,
            },
            Mode::ColorAsMonochrome => Registers {
                a: 0x11,
                f: CpuFlag::from_bits_truncate(0b1000_0000),
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
        (self.a as u16) << 8 | self.f.bits() as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = CpuFlag::from_bits_truncate((value & 0x00F0) as u8)
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
        self.set_hl(self.hl() - 1);
        self.hl()
    }

    pub fn increment_hl(&mut self) -> u16 {
        self.set_hl(self.hl() + 1);
        self.hl()
    }

    pub fn set_flag(&mut self, flag: CpuFlag, status: bool) {
        if status {
            self.f.insert(flag)
        } else {
            self.f.remove(flag)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registors_initialization() {
        let registers = Registers::new(Mode::Monochrome);
        assert_eq!(registers.a, 0x01);
        assert_eq!(registers.f.bits(), 0b1011_0000);
        assert_eq!(registers.b, 0x00);
        assert_eq!(registers.c, 0x13);
        assert_eq!(registers.d, 0x00);
        assert_eq!(registers.e, 0xD8);
        assert_eq!(registers.h, 0x01);
        assert_eq!(registers.l, 0x4D);
        assert_eq!(registers.pc, 0x0100);
        assert_eq!(registers.sp, 0xFFFE);
    }

    #[test]
    fn set_and_read_registers() {
        let mut registers = Registers::new(Mode::Monochrome);
        registers.a = 0x35;
        registers.f = CpuFlag::from_bits_truncate(0b1111_0000);
        registers.b = 0x77;
        registers.c = 0x11;
        registers.d = 0x56;
        registers.e = 0xC7;
        registers.h = 0x01;
        registers.l = 0x4D;

        assert_eq!(registers.af(), 0x35F0);
        assert_eq!(registers.bc(), 0x7711);
        assert_eq!(registers.de(), 0x56C7);
        assert_eq!(registers.hl(), 0x014D);

        registers.set_af(0x7710);
        registers.set_bc(0xDDDD);
        registers.set_de(0xABCD);
        registers.set_hl(0xFEAC);

        assert_eq!(registers.af(), 0x7710);
        assert_eq!(registers.bc(), 0xDDDD);
        assert_eq!(registers.de(), 0xABCD);
        assert_eq!(registers.hl(), 0xFEAC);
    }

    #[test]
    fn increment_and_decrement_hl() {
        let mut registers = Registers::new(Mode::Monochrome);
        assert_eq!(registers.increment_hl(), 0x014E);
        assert_eq!(registers.decrement_hl(), 0x014D);
    }
}
