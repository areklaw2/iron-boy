use super::{
    VIEWPORT_HEIGHT, VIEWPORT_WIDTH,
    tile::{TILE_HEIGHT, TILE_WIDTH},
};

pub struct Window {
    wx: u8,
    wy: u8,
    line_counter: u8,
}

impl Window {
    pub fn new() -> Self {
        Window {
            wx: 0,
            wy: 0,
            line_counter: 0,
        }
    }

    pub fn wx(&self) -> u8 {
        self.wx
    }

    pub fn set_wx(&mut self, value: u8) {
        if value < 7 {
            return;
        }
        self.wx = value;
    }

    pub fn wy(&self) -> u8 {
        self.wy
    }

    pub fn set_wy(&mut self, value: u8) {
        self.wy = value;
    }

    pub fn inside_window(&self, window_enabled: bool, lx: u8, ly: u8) -> bool {
        (window_enabled && lx >= self.wx.wrapping_sub(7)) && (window_enabled && ly >= self.wy)
    }

    pub fn reset_line_counter(&mut self) {
        self.line_counter = 0;
    }

    pub fn increment_line_counter(&mut self, window_enabled: bool, ly: u8) {
        if window_enabled && self.wx - 7 < VIEWPORT_WIDTH as u8 && self.wy < VIEWPORT_HEIGHT as u8 && ly >= self.wy {
            self.line_counter = self.line_counter.saturating_add(1);
        }
    }

    pub fn tile_map_coordinates(&self, lx: u8) -> (u8, u8) {
        let x = lx.wrapping_sub(self.wx.wrapping_sub(7));
        let y = self.line_counter;
        (x, y)
    }

    pub fn pixel_offsets(&self, lx: u8, ly: u8) -> (u8, u8) {
        let x_offset = self.wx.wrapping_sub(lx) % TILE_WIDTH;
        let y_offset = 2 * ((ly - self.wy) % TILE_HEIGHT);
        (x_offset, y_offset)
    }
}
