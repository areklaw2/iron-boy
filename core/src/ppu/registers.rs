use super::{
    tile::{TileDataAddressingMode, TileMap},
    Ppu,
};

#[derive(Copy, Clone)]
pub struct LcdControl {
    lcd_enabled: bool,
    window_tile_map: TileMap,
    window_enabled: bool,
    tile_data_addressing_mode: TileDataAddressingMode,
    bg_tile_map: TileMap,
    object_size: bool,
    object_enabled: bool,
    bg_window_enabled: bool,
}

impl LcdControl {
    pub fn new() -> Self {
        LcdControl {
            lcd_enabled: true,
            window_tile_map: TileMap::Low,
            window_enabled: false,
            tile_data_addressing_mode: TileDataAddressingMode::Low,
            bg_tile_map: TileMap::Low,
            object_size: false,
            object_enabled: false,
            bg_window_enabled: true,
        }
    }

    pub fn lcd_enabled(&self) -> bool {
        self.lcd_enabled
    }

    pub fn window_tile_map(&self) -> TileMap {
        self.window_tile_map
    }

    pub fn window_enabled(&self) -> bool {
        self.window_enabled
    }

    pub fn tile_data(&self) -> TileDataAddressingMode {
        self.tile_data_addressing_mode
    }

    pub fn bg_tile_map(&self) -> TileMap {
        self.bg_tile_map
    }

    pub fn object_size(&self) -> bool {
        self.object_size
    }

    pub fn object_enabled(&self) -> bool {
        self.object_enabled
    }

    pub fn bg_window_enabled(&self) -> bool {
        self.bg_window_enabled
    }
}

impl From<&LcdControl> for u8 {
    fn from(lcd_control: &LcdControl) -> Self {
        (lcd_control.lcd_enabled as u8) << 7
            | (lcd_control.window_tile_map as u8) << 6
            | (lcd_control.window_enabled as u8) << 5
            | (lcd_control.tile_data_addressing_mode as u8) << 4
            | (lcd_control.bg_tile_map as u8) << 3
            | (lcd_control.object_size as u8) << 2
            | (lcd_control.object_enabled as u8) << 1
            | (lcd_control.bg_window_enabled as u8)
    }
}

impl From<u8> for LcdControl {
    fn from(value: u8) -> Self {
        LcdControl {
            lcd_enabled: (value & 0x80) != 0,
            window_tile_map: ((value & 0x40) != 0).into(),
            window_enabled: (value & 0x20) != 0,
            tile_data_addressing_mode: ((value & 0x10) != 0).into(),
            bg_tile_map: ((value & 0x08) != 0).into(),
            object_size: (value & 0x04) != 0,
            object_enabled: (value & 0x02) != 0,
            bg_window_enabled: (value & 0x01) != 0,
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum PpuMode {
    HBlank = 0,
    VBlank = 1,
    OamScan = 2,
    DrawingPixels = 3,
}

impl From<u8> for PpuMode {
    fn from(value: u8) -> Self {
        match value {
            0 => PpuMode::HBlank,
            1 => PpuMode::VBlank,
            2 => PpuMode::OamScan,
            _ => PpuMode::DrawingPixels,
        }
    }
}

pub struct LcdStatus {
    lyc_interrupt: bool,
    mode2_interrupt: bool,
    mode1_interrupt: bool,
    mode0_interrupt: bool,
    lyc_equals_ly: bool,
    pub mode: PpuMode,
}

impl LcdStatus {
    pub fn new() -> Self {
        LcdStatus {
            lyc_interrupt: false,
            mode2_interrupt: false,
            mode1_interrupt: false,
            mode0_interrupt: false,
            lyc_equals_ly: false,
            mode: PpuMode::HBlank,
        }
    }

    pub fn lyc_interrupt(&self) -> bool {
        self.lyc_interrupt
    }

    pub fn lyc_equals_ly(&self) -> bool {
        self.lyc_equals_ly
    }

    pub fn set_lyc_equals_ly(&mut self, status: bool) {
        self.lyc_equals_ly = status
    }

    pub fn mode(&self) -> PpuMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: PpuMode) -> bool {
        self.mode = mode;
        match self.mode {
            PpuMode::HBlank => self.mode0_interrupt,
            PpuMode::VBlank => self.mode1_interrupt,
            PpuMode::OamScan => self.mode1_interrupt,
            PpuMode::DrawingPixels => false,
        }
    }
}

impl From<&LcdStatus> for u8 {
    fn from(lcd_status: &LcdStatus) -> Self {
        (lcd_status.lyc_interrupt as u8) << 6
            | (lcd_status.mode2_interrupt as u8) << 5
            | (lcd_status.mode1_interrupt as u8) << 4
            | (lcd_status.mode0_interrupt as u8) << 3
            | (lcd_status.lyc_equals_ly as u8) << 2
            | (lcd_status.mode as u8)
    }
}

impl From<u8> for LcdStatus {
    fn from(value: u8) -> Self {
        LcdStatus {
            lyc_interrupt: (value & 0x40) != 0,
            mode2_interrupt: (value & 0x20) != 0,
            mode1_interrupt: (value & 0x10) != 0,
            mode0_interrupt: (value & 0x08) != 0,
            lyc_equals_ly: (value & 0x04) != 0,
            mode: (value & 0x03).into(),
        }
    }
}

// TODO: make registers structs ??
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
        let previous_lcd_enabled = self.lcd_enabled;

        self.lcd_enabled = data & 0x80 == 0x80;
        self.window_tile_map = match data & 0x40 == 0x40 {
            true => TileMap::High,
            false => TileMap::Low,
        };
        self.window_enabled = data & 0x20 == 0x20;
        self.tile_data = match data & 0x10 == 0x10 {
            true => TileDataAddressingMode::Low,
            false => TileDataAddressingMode::High,
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

        if previous_lcd_enabled && !self.lcd_enabled {
            self.line_ticks = 0;
            self.ly = 0;
            self.mode = PpuMode::HBlank;
            self.wy_trigger = false;
            self.clear_screen();
        }

        if !previous_lcd_enabled && self.lcd_enabled {
            self.change_mode(PpuMode::OamScan);
            self.line_ticks = 4;
        }
    }

    pub fn stat_read(&self) -> u8 {
        let mut data = 0x80;

        data |= (self.lyc_interrupt as u8) << 6;
        data |= (self.mode2_interrupt as u8) << 5;
        data |= (self.mode1_interrupt as u8) << 4;
        data |= (self.mode0_interrupt as u8) << 3;
        data |= ((self.ly == self.lyc) as u8) << 2;
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
