use getset::{CopyGetters, Setters};

#[derive(Debug, Copy, Clone, CopyGetters, Setters)]
pub struct Flags {
    #[getset(get_copy = "pub", set = "pub")]
    zero: bool,
    #[getset(get_copy = "pub", set = "pub")]
    subtraction: bool,
    #[getset(get_copy = "pub", set = "pub")]
    half_carry: bool,
    #[getset(get_copy = "pub", set = "pub")]
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
