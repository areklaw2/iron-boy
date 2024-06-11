#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White = 0,
    LightGrey = 1,
    DarkGrey = 2,
    Black = 3,
}

impl Color {
    pub fn from_byte(c: u8) -> Color {
        match c {
            0 => Color::White,
            1 => Color::LightGrey,
            2 => Color::DarkGrey,
            _ => Color::Black,
        }
    }

    pub fn value(&self) -> (u8, u8, u8) {
        match self {
            Color::White => (255, 255, 255),
            Color::LightGrey => (192, 192, 192),
            Color::DarkGrey => (96, 96, 96),
            Color::Black => (0, 0, 0),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Palette {
    data: [Color; 4],
}

impl Palette {
    pub fn from_byte(byte: u8) -> Palette {
        let mut pallete = Palette {
            data: [Color::White, Color::White, Color::White, Color::White],
        };
        for i in 0..pallete.data.len() {
            pallete.data[i] = Color::from_byte((byte >> (i * 2)) & 0b11)
        }
        pallete
    }

    pub fn into_byte(&self) -> u8 {
        let mut pallete = 0;
        for i in 0..self.data.len() {
            pallete |= (self.data[i] as u8) << (i * 2);
        }
        pallete
    }

    pub fn get_color(&self, c: u8) -> Color {
        self.data[c as usize]
    }
}
