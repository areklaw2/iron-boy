pub struct Tile {
    pixels: [[u8; 8]; 8],
}

impl Tile {
    pub fn new(bytes: &[u8; 16]) -> Self {
        let mut pairs = Vec::new();
        let mut i = 0;
        while i < 16 {
            pairs.push((bytes[i], bytes[i + 1]));
            i += 2
        }
        let mut pixels = [[0; 8]; 8];
        for (i, pair) in pairs.iter().enumerate() {
            for j in 0..=7 {
                let a = (pair.0 & (1 << j)) >> j;
                let b = (pair.1 & (1 << j)) >> j;
                pixels[i][7 - j] = b << 1 | a;
            }
        }
        Tile { pixels }
    }

    pub fn get_tile_pixels_2d(&mut self) -> [[u8; 8]; 8] {
        self.pixels
    }

    pub fn get_tile_pixels_1d(&mut self) -> [u8; 64] {
        let mut pixels = [0; 64];
        for i in 0..8 {
            for j in 0..8 {
                pixels[i * 8 + j] = self.pixels[i][j];
            }
        }
        pixels
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile_created() {
        let bytes = [
            0x3C, 0x7E, 0x42, 0x42, 0x42, 0x42, 0x42, 0x42, 0x7E, 0x5E, 0x7E, 0x0A, 0x7C, 0x56, 0x38, 0x7C,
        ];

        let actual = [
            [0b00, 0b10, 0b11, 0b11, 0b11, 0b11, 0b10, 0b00],
            [0b00, 0b11, 0b00, 0b00, 0b00, 0b00, 0b11, 0b00],
            [0b00, 0b11, 0b00, 0b00, 0b00, 0b00, 0b11, 0b00],
            [0b00, 0b11, 0b00, 0b00, 0b00, 0b00, 0b11, 0b00],
            [0b00, 0b11, 0b01, 0b11, 0b11, 0b11, 0b11, 0b00],
            [0b00, 0b01, 0b01, 0b01, 0b11, 0b01, 0b11, 0b00],
            [0b00, 0b11, 0b01, 0b11, 0b01, 0b11, 0b10, 0b00],
            [0b00, 0b10, 0b11, 0b11, 0b11, 0b10, 0b00, 0b00],
        ];

        let tile = Tile::new(&bytes);
        assert_eq!(tile.pixels, actual)
    }
}
