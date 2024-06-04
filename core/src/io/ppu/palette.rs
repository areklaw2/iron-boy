#[derive(Copy, Clone)]
pub struct Palette {
    data: [u8; 4],
    byte: u8,
}

impl Palette {
    pub fn new() -> Self {
        Palette { data: [0; 4], byte: 0 }
    }

    pub fn read_as_color_values(&self) -> &[u8; 4] {
        &self.data
    }

    pub fn read_as_byte(&self) -> u8 {
        self.byte
    }

    pub fn write(&mut self, byte: u8) {
        let mut data = [0; 4];
        for i in 0..=3 {
            let value = (byte >> 2 * i) & 0b11;
            let color = match value {
                0 => 0xFF,
                1 => 0xAA,
                2 => 0x55,
                _ => 0x00,
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
        pallete.write(0b0001_1011);
        let data = [0, 85, 170, 255];
        assert_eq!(*pallete.read_as_color_values(), data);
        assert_eq!(pallete.read_as_byte(), 0b0001_1011)
    }
}
