use super::color::Color;

#[derive(Copy, Clone)]
pub enum Palette {
    Obp0,
    Obp1,
}

#[derive(Copy, Clone)]
pub struct PaletteData {
    data: [Color; 4],
}

impl PaletteData {
    pub fn from_byte(byte: u8) -> PaletteData {
        let mut pallete = PaletteData {
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

    pub fn get_color(&self, c: Color) -> Color {
        self.data[c as usize]
    }

    pub fn get_color_u8(&self, c: u8) -> Color {
        self.data[c as usize]
    }
}
