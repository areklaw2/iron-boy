#[derive(Copy, Clone)]
pub struct Palette {
    data: [u8; 4],
}

impl Palette {
    pub fn new(value: u8) -> Self {
        let mut pallete = Palette { data: [0; 4] };
        for i in 0..pallete.data.len() {
            pallete.data[i] = (value >> (i * 2)) & 0b11
        }
        pallete
    }

    pub fn pixel_color(&self, color: u8) -> (u8, u8, u8) {
        match self.data[color as usize] {
            0 => (255, 255, 255), // white
            1 => (192, 192, 192), // light gray
            2 => (96, 96, 96),    // dark gray
            _ => (0, 0, 0),       // black
        }
    }

    pub fn write(&mut self, value: u8) {
        for i in 0..self.data.len() {
            self.data[i] = (value >> (i * 2)) & 0b11
        }
    }

    pub fn read(&self) -> u8 {
        let mut pallete = 0;
        for i in 0..self.data.len() {
            pallete |= (self.data[i] as u8) << (i * 2);
        }
        pallete
    }
}

pub fn color_index(byte1: u8, byte2: u8, pixel_index: u8) -> u8 {
    let lsb = (byte1 >> pixel_index) & 0b1;
    let msb = ((byte2 >> pixel_index) & 0b1) << 1;
    msb | lsb
}

#[derive(Copy, Clone)]
pub struct CgbPalette {
    increment: bool,
    address: u8,
    data: [[[u8; 3]; 4]; 8],
}

impl CgbPalette {
    pub fn new() -> Self {
        CgbPalette {
            increment: false,
            address: 0,
            data: [[[0u8; 3]; 4]; 8],
        }
    }

    pub fn pixel_color(&self, palette: u8, color: u8) -> (u8, u8, u8) {
        let r = self.data[palette as usize][color as usize][0] as u32;
        let g = self.data[palette as usize][color as usize][1] as u32;
        let b = self.data[palette as usize][color as usize][2] as u32;

        let r_out = ((r * 13 + g * 2 + b) >> 1) as u8;
        let g_out = ((g * 3 + b) << 1) as u8;
        let b_out = ((r * 3 + g * 2 + b * 11) >> 1) as u8;
        (r_out, g_out, b_out)
    }

    pub fn write_spec_and_index(&mut self, value: u8) {
        self.increment = value & 0x80 == 0x80;
        self.address = value & 0x3F;
    }

    pub fn write_palette(&mut self, value: u8) {
        let palette = (self.address >> 3) as usize;
        let color = ((self.address >> 1) & 0x03) as usize;
        if self.address & 0x01 == 0x00 {
            self.data[palette][color][0] = value & 0x1F;
            self.data[palette][color][1] = (self.data[palette][color][1] & 0x18) | (value >> 5);
        } else {
            self.data[palette][color][1] = (self.data[palette][color][1] & 0x07) | ((value & 0x3) << 3);
            self.data[palette][color][2] = (value >> 2) & 0x1F;
        }
        if self.increment {
            self.address = (self.address + 1) & 0x3F;
        };
    }

    pub fn read_spec_and_index(&self) -> u8 {
        (self.increment as u8) << 7 | 0x40 | self.address
    }

    pub fn read_palette(&self) -> u8 {
        let palette = (self.address >> 3) as usize;
        let color = ((self.address >> 1) & 0x3) as usize;
        if self.address & 0x01 == 0x00 {
            self.data[palette][color][0] | ((self.data[palette][color][1] & 0x07) << 5)
        } else {
            ((self.data[palette][color][1] & 0x18) >> 3) | (self.data[palette][color][2] << 2)
        }
    }
}
