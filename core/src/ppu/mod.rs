use color::{AlphaColor, Color};
use object::{Object, ObjectSize};
use palette::{Palette, PaletteData};
use tile::{TileData, TileMap};

use crate::bus::Memory;

pub mod color;
pub mod object;
pub mod palette;
pub mod tile;

const VRAM_SIZE: usize = 0x4000;
const OAM_SIZE: usize = 40;
pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

#[derive(PartialEq, Copy, Clone)]
enum Mode {
    OamScan = 2,
    DrawingPixels = 3,
    HBlank = 0,
    VBlank = 1,
}

pub struct Ppu {
    mode: Mode,
    line_ticks: u32,
    line: u8,
    lyc: u8,
    lcd_enabled: bool,
    window_tile_map: TileMap,
    window_enabled: bool,
    tile_data: TileData,
    bg_tile_map: TileMap,
    object_size: ObjectSize,
    object_enabled: bool,
    bg_window_enabled: bool,
    lyc_interrupt: bool,
    mode0_interrupt: bool,
    mode1_interrupt: bool,
    mode2_interrupt: bool,
    scy: u8,
    scx: u8,
    wy: u8,
    wx: u8,
    bg_palette: PaletteData,
    obj0_palette: PaletteData,
    obj1_palette: PaletteData,
    pub vram: [u8; VRAM_SIZE],
    oam: [Object; OAM_SIZE],
    object_line_buffer: [[Option<u8>; 10]; 144],
    pub screen_buffer: Vec<u8>,
    pub screen_updated: bool,
    pub interrupt: u8,
    vrambank: usize,
}

impl Memory for Ppu {
    fn mem_read(&mut self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => self.vram[(self.vrambank * 0x2000) | (address as usize & 0x1FFF)],
            0xFE00..=0xFE9F => self.read_oam(address - 0xFE00),
            0xFF40 => self.read_lcdc(),
            0xFF41 => self.read_stat(),
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.line,
            0xFF45 => self.lyc,
            0xFF46 => 0, // Write only
            0xFF47 => self.bg_palette.into_byte(),
            0xFF48 => self.obj0_palette.into_byte(),
            0xFF49 => self.obj1_palette.into_byte(),
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            0xFF4C => 0xFF,
            0xFF4E => 0xFF,
            _ => 0xFF,
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        match address {
            0x8000..=0x9FFF => self.vram[(self.vrambank * 0x2000) | (address as usize & 0x1FFF)] = data,
            0xFE00..=0xFE9F => self.write_oam(address - 0xFE00, data),
            0xFF40 => self.write_lcdc(data),
            0xFF41 => self.write_stat(data),
            0xFF42 => self.scy = data,
            0xFF43 => self.scx = data,
            0xFF44 => {} // Read-only
            0xFF45 => {
                self.lyc = data;
                self.trigger_lyc_interrupt();
            }
            0xFF46 => panic!("0xFF46 should be handled by MMU"),
            0xFF47 => self.bg_palette = PaletteData::from_byte(data),
            0xFF48 => self.obj0_palette = PaletteData::from_byte(data),
            0xFF49 => self.obj1_palette = PaletteData::from_byte(data),
            0xFF4A => self.wy = data,
            0xFF4B => self.wx = data,
            0xFF4C => {}
            0xFF4E => {}
            _ => panic!("Ppu does not handle write {:04X}", address),
        }
    }
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            mode: Mode::OamScan,
            line_ticks: 0,
            line: 0,
            lyc: 0,
            lcd_enabled: false,
            window_tile_map: TileMap::Low,
            window_enabled: false,
            tile_data: TileData::Block0,
            bg_tile_map: TileMap::Low,
            object_size: ObjectSize::Size8x8,
            object_enabled: false,
            bg_window_enabled: false,
            lyc_interrupt: false,
            mode2_interrupt: false,
            mode1_interrupt: false,
            mode0_interrupt: false,
            scy: 0,
            scx: 0,
            wy: 0,
            wx: 0,
            bg_palette: PaletteData::from_byte(0),
            obj0_palette: PaletteData::from_byte(0),
            obj1_palette: PaletteData::from_byte(0),
            vram: [0; VRAM_SIZE],
            oam: [Object::new(); OAM_SIZE],
            object_line_buffer: [[None; 10]; SCREEN_HEIGHT],
            screen_buffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT],
            screen_updated: false,
            interrupt: 0,
            vrambank: 0,
        }
    }

    pub fn cycle(&mut self, ticks: u32) {
        if !self.lcd_enabled {
            return;
        }

        if ticks == 0 {
            return;
        }

        self.line_ticks += ticks;
        if self.line_ticks >= 456 as u32 {
            self.line_ticks -= 456 as u32;
            self.line = (self.line + 1) % 154;
            self.trigger_lyc_interrupt();

            if self.line >= 144 && self.mode != Mode::VBlank {
                self.change_mode(Mode::VBlank)
            }
        }

        if self.line < 144 {
            match self.line_ticks {
                0..=80 => {
                    if self.mode != Mode::OamScan {
                        self.change_mode(Mode::OamScan)
                    }
                }
                81..=252 => {
                    if self.mode != Mode::DrawingPixels {
                        self.change_mode(Mode::DrawingPixels)
                    }
                }
                _ => {
                    if self.mode != Mode::HBlank {
                        self.change_mode(Mode::HBlank);
                    }
                }
            }
        }
    }

    fn trigger_lyc_interrupt(&mut self) {
        if self.lyc_interrupt && self.line == self.lyc {
            self.interrupt |= 0x02;
        }
    }

    fn change_mode(&mut self, mode: Mode) {
        self.mode = mode;

        let interrupt_triggered = match self.mode {
            Mode::HBlank => self.mode0_interrupt,
            Mode::VBlank => {
                self.interrupt |= 0x01;
                self.screen_updated = true;
                self.mode1_interrupt
            }
            Mode::OamScan => self.mode2_interrupt,
            Mode::DrawingPixels => {
                self.render_pixels();
                false
            }
        };

        if interrupt_triggered {
            self.interrupt |= 0x02;
        }
    }

    pub fn read_oam(&self, address: u16) -> u8 {
        let index = (address / 4) as usize;
        let attribute = address % 4;
        let object = self.oam[index];
        match attribute {
            0 => object.y_position(),
            1 => object.x_position(),
            2 => object.tile_index(),
            3 => object.flags(),
            _ => panic!("No such attribute exists"),
        }
    }

    pub fn write_oam(&mut self, address: u16, data: u8) {
        let index = (address / 4) as usize;
        let attribute = address % 4;
        let object = &mut self.oam[index];
        match attribute {
            0 => {
                if object.y_position() != data {
                    object.set_y_position(data);
                    self.rebuild_line_objects()
                }
            }
            1 => {
                if object.x_position() != data {
                    object.set_x_position(data);
                    self.rebuild_line_objects()
                }
            }
            2 => {
                if object.tile_index() != data {
                    object.set_tile_index(data);
                    self.rebuild_line_objects()
                }
            }
            3 => {
                if object.flags() != data {
                    object.set_flags(data);
                    self.rebuild_line_objects()
                }
            }
            _ => panic!("No such attribute exists"),
        };
    }

    pub fn read_lcdc(&self) -> u8 {
        let mut data = 0;

        data |= (self.lcd_enabled as u8) << 7;
        data |= match self.window_tile_map {
            TileMap::High => 1,
            TileMap::Low => 0,
        } << 6;
        data |= (self.window_enabled as u8) << 5;
        data |= match self.tile_data {
            TileData::Block1 => 1,
            TileData::Block0 => 0,
        } << 4;
        data |= match self.bg_tile_map {
            TileMap::High => 1,
            TileMap::Low => 0,
        } << 3;
        data |= match self.object_size {
            ObjectSize::Size8x16 => 1,
            ObjectSize::Size8x8 => 0,
        } << 2;
        data |= (self.object_enabled as u8) << 1;
        data |= (self.bg_window_enabled as u8) << 0;

        data
    }

    pub fn write_lcdc(&mut self, data: u8) {
        let lcd_enabled = self.lcd_enabled;
        self.lcd_enabled = data & 0x80 == 0x80;
        self.window_tile_map = match data & 0x40 == 0x40 {
            true => TileMap::High,
            false => TileMap::Low,
        };
        self.window_enabled = data & 0x20 == 0x20;
        self.tile_data = match data & 0x10 == 0x10 {
            true => TileData::Block1,
            false => TileData::Block0,
        };
        self.bg_tile_map = match data & 0x08 == 0x08 {
            true => TileMap::High,
            false => TileMap::Low,
        };
        let new_object_size = match data & 0x04 == 0x04 {
            true => ObjectSize::Size8x16,
            false => ObjectSize::Size8x8,
        };
        self.object_enabled = data & 0x02 == 0x02;
        self.bg_window_enabled = data & 0x01 == 0x01;

        if !self.lcd_enabled {
            self.line_ticks = 0;
            self.line = 0;
            self.mode = Mode::HBlank;
            self.clear_screen()
        }

        if !lcd_enabled && self.lcd_enabled {
            self.change_mode(Mode::OamScan);
            self.line_ticks = 4;
        }

        if new_object_size != self.object_size {
            self.object_size = new_object_size;
            self.rebuild_line_objects();
        }
    }

    pub fn read_stat(&self) -> u8 {
        let mut data = 0;
        data |= 1 << 7;
        data |= (self.lyc_interrupt as u8) << 6;
        data |= (self.mode2_interrupt as u8) << 5;
        data |= (self.mode1_interrupt as u8) << 4;
        data |= (self.mode0_interrupt as u8) << 3;
        data |= ((self.line == self.lyc) as u8) << 2;
        data |= self.mode as u8;
        data
    }

    pub fn write_stat(&mut self, data: u8) {
        self.lyc_interrupt = data & 0x40 == 0x40;
        self.mode2_interrupt = data & 0x20 == 0x20;
        self.mode1_interrupt = data & 0x10 == 0x10;
        self.mode0_interrupt = data & 0x08 == 0x08;
    }

    fn rebuild_line_objects(&mut self) {
        self.object_line_buffer = [[None; 10]; 144];
        for object_index in 0..self.oam.len() {
            let object = self.oam[object_index as usize];
            let height = self.object_size.height();
            let start = object.top_line();
            let end = start + (height as i32);

            for y in start..end {
                if y < 0 || y >= 144 {
                    continue;
                }

                let y = y as usize;
                let lines = self.object_line_buffer[y].len();
                if self.object_line_buffer[y][lines - 1].is_some() {
                    continue;
                }

                for i in 0..lines {
                    match self.object_line_buffer[y][i] {
                        None => {
                            self.object_line_buffer[y][i] = Some(object_index as u8);
                            break;
                        }
                        Some(other) => {
                            let other_object = &self.oam[other as usize];
                            if object.x_position() < other_object.x_position()
                                || (object.x_position() == other_object.x_position() && object_index < other as usize)
                            {
                                for j in (i..(lines - 1)).rev() {
                                    self.object_line_buffer[y][j + 1] = self.object_line_buffer[y][j];
                                }

                                self.object_line_buffer[y][i] = Some(object_index as u8);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    fn clear_screen(&mut self) {
        self.screen_buffer = vec![255; SCREEN_WIDTH * SCREEN_HEIGHT];
        self.screen_updated = true;
    }

    fn render_pixels(&mut self) {
        let y = self.line;
        for x in 0..SCREEN_WIDTH as u8 {
            let bg_window_color = if self.window_enabled && self.inside_window(x, y) {
                self.window_color(x, y)
            } else if self.bg_window_enabled {
                self.background_color(x, y)
            } else {
                AlphaColor {
                    color: Color::White,
                    opaque: false,
                }
            };
            let color = if self.object_enabled {
                self.object_color(x, y, bg_window_color)
            } else {
                bg_window_color.color
            };

            self.screen_buffer[y as usize * SCREEN_WIDTH + x as usize] = color.value();
        }
    }

    fn inside_window(&self, x: u8, y: u8) -> bool {
        x as i32 >= (self.wx as i32) - 7 && y as i32 >= self.wy as i32
    }

    fn window_color(&self, x: u8, y: u8) -> AlphaColor {
        let wx = self.wx.wrapping_sub(7);
        let x = x.wrapping_sub(wx);
        let y = y.wrapping_sub(self.wy);
        let tile_map = self.window_tile_map;
        let tile_data = self.tile_data;
        self.bg_window_color(x, y, tile_map, tile_data)
    }

    fn background_color(&self, x: u8, y: u8) -> AlphaColor {
        let x = x.wrapping_add(self.scx);
        let y = y.wrapping_add(self.scy);
        let tile_map = self.window_tile_map;
        let tile_data = self.tile_data;
        self.bg_window_color(x, y, tile_map, tile_data)
    }

    fn bg_window_color(&self, x: u8, y: u8, tile_map: TileMap, tile_data: TileData) -> AlphaColor {
        let tile_map_x = x / 8;
        let tile_map_y = y / 8;
        let tile_x = x % 8;
        let tile_y = y % 8;

        let tile_index = self.tile_index(tile_map_x, tile_map_y, tile_map);
        let tile_color = self.pixel_color(tile_index, tile_x, tile_y, tile_data);

        AlphaColor {
            color: self.bg_palette.get_color(tile_color),
            opaque: tile_color != Color::White,
        }
    }

    fn tile_index(&self, tx: u8, ty: u8, tile_map: TileMap) -> u8 {
        let base = tile_map.base_offset();
        let tx = tx as u16;
        let ty = ty as u16;
        let address = base + (ty * 32) + tx;
        self.vram[address as usize]
    }

    fn pixel_color(&self, tile_index: u8, x: u8, y: u8, tile_data: TileData) -> Color {
        if x >= 8 || y >= 16 {
            panic!("Tile positon out of range ({}, {})", x, y);
        }

        let base = tile_data.address(tile_index);
        let address = base + 2 * (y as u16);
        let address = address as usize;
        let x = (7 - x) as usize;
        let lo = (self.vram[address] >> x) & 1;
        let hi = (self.vram[address + 1] >> x) & 1;

        Color::from_byte(hi << 1 | lo)
    }

    fn object_color(&self, x: u8, y: u8, bg_window_color: AlphaColor) -> Color {
        for &line in self.object_line_buffer[y as usize].iter() {
            match line {
                None => break,
                Some(index) => {
                    let object = &self.oam[index as usize];
                    let object_x = (x as i32) - object.left_column();
                    if object_x >= 8 {
                        continue;
                    }

                    if object_x < 0 {
                        break;
                    }

                    if object.is_behind_background() && bg_window_color.opaque {
                        continue;
                    }

                    let object_y = (y as i32) - object.top_line();
                    let (height, tile_index) = match self.object_size {
                        ObjectSize::Size8x8 => (7, object.tile_index()),
                        ObjectSize::Size8x16 => (15, object.tile_index() & 0xFE),
                    };

                    let object_y = match object.y_flip() {
                        true => height - object_y,
                        false => object_y,
                    };

                    let object_x = match object.x_flip() {
                        true => 7 - object_x,
                        false => object_x,
                    };

                    let pixel = self.pixel_color(tile_index, object_x as u8, object_y as u8, TileData::Block1);
                    if pixel != Color::White {
                        let palette = match object.palette() {
                            Palette::Obp0 => self.obj0_palette,
                            Palette::Obp1 => self.obj1_palette,
                        };
                        return palette.get_color(pixel);
                    }
                }
            }
        }

        bg_window_color.color
    }
}
