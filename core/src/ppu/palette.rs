#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White = 0,
    LightGrey = 1,
    DarkGrey = 2,
    Black = 3,
}

impl From<u8> for Color {
    fn from(value: u8) -> Self {
        match value {
            0 => Color::White,
            1 => Color::LightGrey,
            2 => Color::DarkGrey,
            _ => Color::Black,
        }
    }
}

impl From<Color> for (u8, u8, u8) {
    fn from(color: Color) -> Self {
        match color {
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

impl From<u8> for Palette {
    fn from(value: u8) -> Self {
        let mut pallete = Palette { data: [Color::White; 4] };
        for i in 0..pallete.data.len() {
            pallete.data[i] = Color::from((value >> (i * 2)) & 0b11)
        }
        pallete
    }
}

impl From<Palette> for u8 {
    fn from(palette: Palette) -> Self {
        let mut pallete = 0;
        for i in 0..palette.data.len() {
            pallete |= (palette.data[i] as u8) << (i * 2);
        }
        pallete
    }
}

impl Palette {
    pub fn color(&self, color: u8) -> Color {
        self.data[color as usize]
    }
}
