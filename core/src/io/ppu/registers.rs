#[derive(Debug)]
pub struct LcdStatus {
    pub lyc_interrupt: bool,
    pub mode2_interrupt: bool,
    pub mode1_interrupt: bool,
    pub mode0_interrupt: bool,
}

impl LcdStatus {
    pub fn new() -> Self {
        Self {
            lyc_interrupt: false,
            mode2_interrupt: false,
            mode1_interrupt: false,
            mode0_interrupt: false,
        }
    }

    pub fn read(&self, ly: u8, lyc: u8, mode: u8) -> u8 {
        0x80 | (if self.lyc_interrupt { 0x40 } else { 0 })
            | (if self.mode2_interrupt { 0x20 } else { 0 })
            | (if self.mode1_interrupt { 0x10 } else { 0 })
            | (if self.mode0_interrupt { 0x08 } else { 0 })
            | (if ly == lyc { 0x04 } else { 0 })
            | mode
    }

    pub fn write(&mut self, data: u8) {
        self.lyc_interrupt = data & 0x40 == 0x40;
        self.mode2_interrupt = data & 0x20 == 0x20;
        self.mode1_interrupt = data & 0x10 == 0x10;
        self.mode0_interrupt = data & 0x08 == 0x08;
    }
}

#[derive(Debug)]
pub struct LcdControl {
    pub lcd_enabled: bool,
    window_tile_map_location: u16,
    pub window_enabled: bool,
    bg_and_window_tiles_location: u16,
    bg_tile_map_location: u16,
    obj_size: u8,
    obj_enabled: bool,
    last_bit: bool, // Flag for BG and window enable on DMG and priority on CGB
}

impl LcdControl {
    pub fn new() -> Self {
        Self {
            lcd_enabled: false,
            window_tile_map_location: 0,
            window_enabled: false,
            bg_and_window_tiles_location: 0,
            bg_tile_map_location: 0,
            obj_size: 0,
            obj_enabled: false,
            last_bit: false,
        }
    }

    pub fn read(&self) -> u8 {
        (if self.lcd_enabled { 0x80 } else { 0 })
            | (if self.window_tile_map_location == 0x9C00 { 0x40 } else { 0 })
            | (if self.window_enabled { 0x20 } else { 0 })
            | (if self.bg_and_window_tiles_location == 0x8000 { 0x10 } else { 0 })
            | (if self.bg_tile_map_location == 0x9C00 { 0x08 } else { 0 })
            | (if self.obj_size == 16 { 0x04 } else { 0 })
            | (if self.obj_enabled { 0x02 } else { 0 })
            | (if self.last_bit { 0x01 } else { 0 })
    }

    pub fn write(&mut self, data: u8) {
        self.lcd_enabled = data & 0x80 == 0x80;
        self.window_tile_map_location = if data & 0x40 == 0x40 { 0x9C00 } else { 0x9800 };
        self.window_enabled = data & 0x20 == 0x20;
        self.bg_and_window_tiles_location = if data & 0x10 == 0x10 { 0x8000 } else { 0x8800 };
        self.bg_tile_map_location = if data & 0x08 == 0x08 { 0x9C00 } else { 0x9800 };
        self.obj_size = if data & 0x04 == 0x04 { 16 } else { 8 };
        self.obj_enabled = data & 0x02 == 0x02;
        self.last_bit = data & 0x01 == 0x01;
    }
}
