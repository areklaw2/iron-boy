use utils::GameBoyMode;

#[derive(Debug)]
pub struct Flags {
    pub z: bool,
    pub n: bool,
    pub h: bool,
    pub c: bool,
}

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Flags {
            z: (value & 0x80) != 0,
            n: (value & 0x40) != 0,
            h: (value & 0x20) != 0,
            c: (value & 0x10) != 0,
        }
    }
}

impl Flags {
    pub fn bits(&self) -> u8 {
        (self.z as u8) << 7 | (self.n as u8) << 6 | (self.h as u8) << 5 | (self.c as u8) << 4
    }
}

#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub f: Flags,
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
    pub fn new(mode: GameBoyMode, skip_boot: bool) -> Self {
        if !skip_boot {
            return Registers {
                a: 0x00,
                f: Flags::from(0b1011_0000),
                b: 0x00,
                c: 0x00,
                d: 0x00,
                e: 0x00,
                h: 0x00,
                l: 0x00,
                pc: 0x0000,
                sp: 0x0000,
            };
        }

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

    pub fn af(&self) -> u16 {
        (self.a as u16) << 8 | self.f.bits() as u16
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
