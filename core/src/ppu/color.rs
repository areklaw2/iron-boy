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

pub struct AlphaColor {
    pub color: Color,
    pub opaque: bool,
}
