use getset::CopyGetters;

#[derive(Debug, Copy, Clone, CopyGetters)]
pub struct Attributes {
    #[getset(get_copy = "pub")]
    priority: bool,
    #[getset(get_copy = "pub")]
    y_flip: bool,
    #[getset(get_copy = "pub")]
    x_flip: bool,
    #[getset(get_copy = "pub")]
    dmg_palette: bool,
    #[getset(get_copy = "pub")]
    bank: bool,
    #[getset(get_copy = "pub")]
    cgb_palette: u8,
}

impl Attributes {
    pub fn new() -> Attributes {
        Attributes {
            priority: false,
            y_flip: false,
            x_flip: false,
            dmg_palette: false,
            bank: false,
            cgb_palette: 0,
        }
    }
}

impl From<&Attributes> for u8 {
    fn from(flags: &Attributes) -> u8 {
        (flags.priority as u8) << 7
            | (flags.y_flip as u8) << 6
            | (flags.x_flip as u8) << 5
            | (flags.dmg_palette as u8) << 4
            | (flags.bank as u8) << 3
            | (flags.cgb_palette as u8) << 2
    }
}

impl From<u8> for Attributes {
    fn from(value: u8) -> Self {
        Attributes {
            priority: (value & 0x80) != 0,
            y_flip: (value & 0x40) != 0,
            x_flip: (value & 0x20) != 0,
            dmg_palette: (value & 0x10) != 0,
            bank: (value & 0x08) != 0,
            cgb_palette: value & 0x07,
        }
    }
}
