use getset::CopyGetters;

#[derive(Debug, Copy, Clone, CopyGetters)]
pub struct OamAttributes {
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

impl OamAttributes {
    pub fn new() -> OamAttributes {
        OamAttributes {
            priority: false,
            y_flip: false,
            x_flip: false,
            dmg_palette: false,
            bank: false,
            cgb_palette: 0,
        }
    }
}

impl From<&OamAttributes> for u8 {
    fn from(flags: &OamAttributes) -> u8 {
        (flags.priority as u8) << 7
            | (flags.y_flip as u8) << 6
            | (flags.x_flip as u8) << 5
            | (flags.dmg_palette as u8) << 4
            | (flags.bank as u8) << 3
            | (flags.cgb_palette as u8) << 2
    }
}

impl From<u8> for OamAttributes {
    fn from(value: u8) -> Self {
        OamAttributes {
            priority: (value & 0x80) != 0,
            y_flip: (value & 0x40) != 0,
            x_flip: (value & 0x20) != 0,
            dmg_palette: (value & 0x10) != 0,
            bank: (value & 0x08) != 0,
            cgb_palette: value & 0x07,
        }
    }
}

#[derive(Debug, Copy, Clone, CopyGetters)]
pub struct BgMapAttributes {
    #[getset(get_copy = "pub")]
    priority: bool,
    #[getset(get_copy = "pub")]
    y_flip: bool,
    #[getset(get_copy = "pub")]
    x_flip: bool,
    #[getset(get_copy = "pub")]
    bank: bool,
    #[getset(get_copy = "pub")]
    color_palette: u8,
}

impl BgMapAttributes {
    pub fn new() -> BgMapAttributes {
        BgMapAttributes {
            priority: false,
            y_flip: false,
            x_flip: false,
            bank: false,
            color_palette: 0,
        }
    }
}

impl From<&BgMapAttributes> for u8 {
    fn from(flags: &BgMapAttributes) -> u8 {
        (flags.priority as u8) << 7
            | (flags.y_flip as u8) << 6
            | (flags.x_flip as u8) << 5
            | (flags.bank as u8) << 3
            | (flags.color_palette as u8) << 2
    }
}

impl From<u8> for BgMapAttributes {
    fn from(value: u8) -> Self {
        BgMapAttributes {
            priority: (value & 0x80) != 0,
            y_flip: (value & 0x40) != 0,
            x_flip: (value & 0x20) != 0,
            bank: (value & 0x08) != 0,
            color_palette: value & 0x07,
        }
    }
}
