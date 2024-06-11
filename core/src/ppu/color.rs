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
}
