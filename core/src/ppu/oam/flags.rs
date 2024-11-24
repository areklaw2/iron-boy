#[derive(Debug, Copy, Clone)]
pub struct Flags {
    priority: bool,
    y_flip: bool,
    x_flip: bool,
    dmg_palette: bool,
    bank: bool,
    cgb_palette: u8,
}

impl Flags {
    pub fn new() -> Self {
        Self {
            priority: false,
            y_flip: false,
            x_flip: false,
            dmg_palette: false,
            bank: false,
            cgb_palette: 0,
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

    pub fn dmg_palette(&self) -> bool {
        self.dmg_palette
    }

    pub fn bank(&self) -> bool {
        self.bank
    }

    pub fn cgb_palette(&self) -> u8 {
        self.cgb_palette
    }
}

impl From<&Flags> for u8 {
    fn from(flags: &Flags) -> u8 {
        (flags.priority as u8) << 7
            | (flags.y_flip as u8) << 6
            | (flags.x_flip as u8) << 5
            | (flags.dmg_palette as u8) << 4
            | (flags.bank as u8) << 3
            | (flags.cgb_palette as u8) << 2
    }
}

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Flags {
            priority: (value & 0x80) != 0,
            y_flip: (value & 0x40) != 0,
            x_flip: (value & 0x20) != 0,
            dmg_palette: (value & 0x10) != 0,
            bank: (value & 0x08) != 0,
            cgb_palette: value & 0x07,
        }
    }
}
