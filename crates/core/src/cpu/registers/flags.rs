use crate::get_set;

#[derive(Debug, Copy, Clone)]
pub struct Flags {
    zero: bool,
    subtraction: bool,
    half_carry: bool,
    carry: bool,
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
    get_set!(zero, set_zero, bool);
    get_set!(subtraction, set_subtraction, bool);
    get_set!(half_carry, set_half_carry, bool);
    get_set!(carry, set_carry, bool);
}
