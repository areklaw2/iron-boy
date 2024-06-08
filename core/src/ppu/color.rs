#[derive(Clone, Copy, PartialEq, Eq)]
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

    pub fn into_byte(&self) -> u8 {
        match self {
            Color::White => 0,
            Color::LightGrey => 1,
            Color::DarkGrey => 2,
            Color::Black => 3,
        }
    }
}

pub struct AlphaColor {
    pub color: Color,
    pub opaque: bool,
}
