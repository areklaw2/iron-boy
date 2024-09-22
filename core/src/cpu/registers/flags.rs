#[derive(Debug)]
pub struct Flags {
    pub zero: bool,
    pub subtraction: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Flags {
            zero: (value & 0x80) != 0,
            subtraction: (value & 0x40) != 0,
            half_carry: (value & 0x20) != 0,
            carry: (value & 0x10) != 0,
        }
    }
}

impl From<&Flags> for u8 {
    fn from(flags: &Flags) -> Self {
        (flags.zero as u8) << 7 | (flags.subtraction as u8) << 6 | (flags.half_carry as u8) << 5 | (flags.carry as u8) << 4
    }
}

impl Flags {
    pub fn zero(&self) -> bool {
        self.zero
    }

    pub fn set_zero(&mut self, status: bool) {
        self.zero = status;
    }

    pub fn subtraction(&self) -> bool {
        self.subtraction
    }

    pub fn set_subtraction(&mut self, status: bool) {
        self.subtraction = status;
    }

    pub fn half_carry(&self) -> bool {
        self.half_carry
    }

    pub fn set_half_carry(&mut self, status: bool) {
        self.half_carry = status;
    }

    pub fn carry(&self) -> bool {
        self.carry
    }

    pub fn set_carry(&mut self, status: bool) {
        self.carry = status;
    }
}
