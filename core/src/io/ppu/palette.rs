#[derive(Copy, Clone)]
pub struct Palette {
    data: [u8; 4],
    byte: u8,
}

impl Palette {
    pub fn new() -> Self {
        Palette { data: [0; 4], byte: 0 }
    }

    pub fn as_color_values(&self) -> &[u8; 4] {
        &self.data
    }

    pub fn as_byte(&self) -> &u8 {
        &self.byte
    }

    pub fn update_palette(&mut self, byte: u8) {
        let mut data = [0; 4];
        for i in 0..=3 {
            let value = (byte >> 2 * i) & 0b11;
            let color = match value {
                0 => 255,
                1 => 192,
                2 => 96,
                _ => 0,
            };
            data[i] = color;
        }
        self.data = data;
        self.byte = byte;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pallete_created() {
        let mut pallete = Palette::new();
        pallete.update_palette(0b0001_1011);
        let data = [0, 96, 192, 255];
        assert_eq!(*pallete.as_color_values(), data);
        assert_eq!(*pallete.as_byte(), 0b0001_1011)
    }
}
