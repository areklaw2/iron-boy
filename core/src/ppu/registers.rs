use super::{
    tile::{TileData, TileMap},
    Mode, Ppu,
};

impl Ppu {
    pub fn lcdc_read(&self) -> u8 {
        let mut data = 0;

        data |= (self.lcd_enabled as u8) << 7;
        data |= (self.window_tile_map as u8) << 6;
        data |= (self.window_enabled as u8) << 5;
        data |= (self.tile_data as u8) << 4;
        data |= (self.bg_tile_map as u8) << 3;
        data |= match self.object_size {
            16 => 1,
            _ => 0,
        } << 2;
        data |= (self.object_enabled as u8) << 1;
        data |= self.bg_window_enabled as u8;

        data
    }

    pub fn lcdc_write(&mut self, data: u8) {
        let previous_lcd_on = self.lcd_enabled;

        self.lcd_enabled = data & 0x80 == 0x80;
        self.window_tile_map = match data & 0x40 == 0x40 {
            true => TileMap::High,
            false => TileMap::Low,
        };
        self.window_enabled = data & 0x20 == 0x20;
        self.tile_data = match data & 0x10 == 0x10 {
            true => TileData::Area1,
            false => TileData::Area0,
        };
        self.bg_tile_map = match data & 0x08 == 0x08 {
            true => TileMap::High,
            false => TileMap::Low,
        };
        self.object_size = match data & 0x04 == 0x04 {
            true => 16,
            false => 8,
        };
        self.object_enabled = data & 0x02 == 0x02;
        self.bg_window_enabled = data & 0x01 == 0x01;

        if previous_lcd_on && !self.lcd_enabled {
            self.line_ticks = 0;
            self.line = 0;
            self.mode = Mode::HBlank;
            self.wy_trigger = false;
            self.clear_screen();
        }

        if !previous_lcd_on && self.lcd_enabled {
            self.change_mode(Mode::OamScan);
            self.line_ticks = 4;
        }
    }

    pub fn stat_read(&self) -> u8 {
        let mut data = 0x80;

        data |= (self.lyc_interrupt as u8) << 6;
        data |= (self.mode2_interrupt as u8) << 5;
        data |= (self.mode1_interrupt as u8) << 4;
        data |= (self.mode0_interrupt as u8) << 3;
        data |= ((self.line == self.lyc) as u8) << 2;
        data |= self.mode as u8;

        data
    }

    pub fn stat_write(&mut self, data: u8) {
        self.lyc_interrupt = data & 0x40 == 0x40;
        self.mode2_interrupt = data & 0x20 == 0x20;
        self.mode1_interrupt = data & 0x10 == 0x10;
        self.mode0_interrupt = data & 0x08 == 0x08;
    }
}
