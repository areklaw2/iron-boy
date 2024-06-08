use super::palette::Palette;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ObjectSize {
    Size8x8,
    Size8x16,
}

impl ObjectSize {
    pub fn height(self) -> usize {
        match self {
            ObjectSize::Size8x8 => 8,
            ObjectSize::Size8x16 => 16,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Object {
    y: u8,
    x: u8,
    tile_index: u8,
    behind_background: bool,
    y_flip: bool,
    x_flip: bool,
    dmg_pallete: Palette,
}

impl Object {
    pub fn new() -> Self {
        Object {
            y: 0,
            x: 0,
            tile_index: 0,
            behind_background: false,
            y_flip: false,
            x_flip: false,
            dmg_pallete: Palette::Obp0,
        }
    }

    pub fn set_x_position(&mut self, x: u8) {
        self.x = x;
    }

    pub fn x_position(&self) -> u8 {
        self.x
    }

    pub fn set_y_position(&mut self, y: u8) {
        self.y = y;
    }

    pub fn y_position(&self) -> u8 {
        self.y
    }

    pub fn top_line(&self) -> i32 {
        (self.y as i32) - 16
    }

    pub fn left_column(&self) -> i32 {
        (self.x as i32) - 8
    }

    pub fn set_tile_index(&mut self, tile: u8) {
        self.tile_index = tile;
    }

    pub fn tile_index(&self) -> u8 {
        self.tile_index
    }

    pub fn is_behind_background(&self) -> bool {
        self.behind_background
    }

    pub fn x_flip(&self) -> bool {
        self.x_flip
    }

    pub fn y_flip(&self) -> bool {
        self.y_flip
    }

    pub fn palette(&self) -> Palette {
        self.dmg_pallete
    }

    pub fn set_flags(&mut self, data: u8) {
        self.behind_background = data & 0x80 != 0;
        self.y_flip = data & 0x40 != 0;
        self.x_flip = data & 0x20 != 0;
        self.dmg_pallete = match data & 0x10 != 0 {
            false => Palette::Obp0,
            true => Palette::Obp1,
        }
    }

    pub fn flags(&self) -> u8 {
        let mut data = 0;

        data |= (self.behind_background as u8) << 7;
        data |= (self.y_flip as u8) << 6;
        data |= (self.x_flip as u8) << 5;
        data |= match self.dmg_pallete {
            Palette::Obp0 => 0,
            Palette::Obp1 => 1,
            Palette::Bg => panic!("Bg pallete not handled in object"),
        } << 4;

        data
    }
}
