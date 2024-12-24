#[derive(Debug, Copy, Clone)]
pub struct BgMapAttributes {
    priority: bool,
    y_flip: bool,
    x_flip: bool,
    bank: bool,
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

    pub fn priority(&self) -> bool {
        self.priority
    }

    pub fn y_flip(&self) -> bool {
        self.y_flip
    }

    pub fn x_flip(&self) -> bool {
        self.x_flip
    }

    pub fn bank(&self) -> bool {
        self.bank
    }

    pub fn color_palette(&self) -> u8 {
        self.color_palette
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
