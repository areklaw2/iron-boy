use std::cmp::Ordering;

use object::{Object, ObjectSize};

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
    window_tile_map: u16,
    window_enabled: bool,
    bg_window_tile_base: u16,
    bg_tile_map: u16,
    object_size: ObjectSize,
    object_enabled: bool,
    lcdc_0: bool,
    lyc_interrupt: bool,
    mode0_interrupt: bool,
    mode1_interrupt: bool,
    mode2_interrupt: bool,
    scy: u8,
    scx: u8,
    winy: u8,
    winx: u8,
    wy_trigger: bool,
    wy_pos: i32,
    bg_palette_register: u8,
    obj0_palette_register: u8,
    obj1_palette_register: u8,
    bg_palette: [u8; 4],
    obj0_palette: [u8; 4],
    obj1_palette: [u8; 4],
    pub vram: [u8; VRAM_SIZE],
    oam: [Object; OAM_SIZE],
    object_line_buffer: [[Option<u8>; 10]; 144],
    pub video_buffer: Vec<u8>,
    pub updated: bool,
    pub interrupt: u8,
    vrambank: usize,
}

impl Memory for Ppu {
    fn mem_read(&mut self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => self.vram[(self.vrambank * 0x2000) | (address as usize & 0x1FFF)],
            0xFE00..=0xFE9F => self.read_oam(address - 0xFE00),
            0xFF40 => self.read_lcdc(),
            0xFF41 => {
                0x80 | (if self.lyc_interrupt { 0x40 } else { 0 })
                    | (if self.mode2_interrupt { 0x20 } else { 0 })
                    | (if self.mode1_interrupt { 0x10 } else { 0 })
                    | (if self.mode0_interrupt { 0x08 } else { 0 })
                    | (if self.line == self.lyc { 0x04 } else { 0 })
                    | self.mode as u8
            }
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.line,
            0xFF45 => self.lyc,
            0xFF46 => 0, // Write only
            0xFF47 => self.bg_palette_register,
            0xFF48 => self.obj0_palette_register,
            0xFF49 => self.obj1_palette_register,
            0xFF4A => self.winy,
            0xFF4B => self.winx,
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
            0xFF41 => {
                self.lyc_interrupt = data & 0x40 == 0x40;
                self.mode2_interrupt = data & 0x20 == 0x20;
                self.mode1_interrupt = data & 0x10 == 0x10;
                self.mode0_interrupt = data & 0x08 == 0x08;
            }
            0xFF42 => self.scy = data,
            0xFF43 => self.scx = data,
            0xFF44 => {} // Read-only
            0xFF45 => {
                self.lyc = data;
                self.trigger_lyc_interrupt();
            }
            0xFF46 => panic!("0xFF46 should be handled by MMU"),
            0xFF47 => {
                self.bg_palette_register = data;
                self.update_pal();
            }
            0xFF48 => {
                self.obj0_palette_register = data;
                self.update_pal();
            }
            0xFF49 => {
                self.obj1_palette_register = data;
                self.update_pal();
            }
            0xFF4A => self.winy = data,
            0xFF4B => self.winx = data,
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
            window_tile_map: 0x9C00,
            window_enabled: false,
            bg_window_tile_base: 0x8000,
            bg_tile_map: 0x9C00,
            object_size: ObjectSize::Size8x8,
            object_enabled: false,
            lcdc_0: false,
            lyc_interrupt: false,
            mode2_interrupt: false,
            mode1_interrupt: false,
            mode0_interrupt: false,
            scy: 0,
            scx: 0,
            winy: 0,
            winx: 0,
            wy_trigger: false,
            wy_pos: -1,
            bg_palette_register: 0,
            obj0_palette_register: 0,
            obj1_palette_register: 1,
            bg_palette: [0; 4],
            obj0_palette: [0; 4],
            obj1_palette: [0; 4],
            vram: [0; VRAM_SIZE],
            oam: [Object::new(); OAM_SIZE],
            object_line_buffer: [[None; 10]; 144],
            video_buffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
            updated: false,
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

        if match self.mode {
            Mode::HBlank => self.mode0_interrupt,
            Mode::VBlank => {
                self.wy_trigger = false;
                self.interrupt |= 0x01;
                self.updated = true;
                self.mode1_interrupt
            }
            Mode::OamScan => self.mode2_interrupt,
            Mode::DrawingPixels => {
                if self.window_enabled && self.wy_trigger == false && self.line == self.winy {
                    self.wy_trigger = true;
                    self.wy_pos = -1;
                }
                false
            }
        } {
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
        (if self.lcd_enabled { 0x80 } else { 0 })
            | (if self.window_tile_map == 0x9C00 { 0x40 } else { 0 })
            | (if self.window_enabled { 0x20 } else { 0 })
            | (if self.bg_window_tile_base == 0x8000 { 0x10 } else { 0 })
            | (if self.bg_tile_map == 0x9C00 { 0x08 } else { 0 })
            | (if self.object_size == ObjectSize::Size8x16 { 0x04 } else { 0 })
            | (if self.object_enabled { 0x02 } else { 0 })
            | (if self.lcdc_0 { 0x01 } else { 0 })
    }

    pub fn write_lcdc(&mut self, data: u8) {
        let orig_lcd_on = self.lcd_enabled;
        self.lcd_enabled = data & 0x80 == 0x80;
        self.window_tile_map = if data & 0x40 == 0x40 { 0x9C00 } else { 0x9800 };
        self.window_enabled = data & 0x20 == 0x20;
        self.bg_window_tile_base = if data & 0x10 == 0x10 { 0x8000 } else { 0x8800 };
        self.bg_tile_map = if data & 0x08 == 0x08 { 0x9C00 } else { 0x9800 };
        self.object_size = if data & 0x04 == 0x04 {
            ObjectSize::Size8x16
        } else {
            ObjectSize::Size8x8
        };
        self.object_enabled = data & 0x02 == 0x02;
        self.lcdc_0 = data & 0x01 == 0x01;
        if orig_lcd_on && !self.lcd_enabled {
            self.line_ticks = 0;
            self.line = 0;
            self.mode = Mode::HBlank;
            self.wy_trigger = false;
            self.clear_screen();
        }
        if !orig_lcd_on && self.lcd_enabled {
            self.change_mode(Mode::OamScan);
            self.line_ticks = 4;
        }
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

    fn rbvram0(&self, a: u16) -> u8 {
        if a < 0x8000 || a >= 0xA000 {
            panic!("Shouldn't have used rbvram0");
        }
        self.vram[a as usize & 0x1FFF]
    }

    fn clear_screen(&mut self) {
        for v in self.video_buffer.iter_mut() {
            *v = 255;
        }
        self.updated = true;
    }

    fn update_pal(&mut self) {
        for i in 0..4 {
            self.bg_palette[i] = Ppu::get_monochrome_pal_val(self.bg_palette_register, i);
            self.obj0_palette[i] = Ppu::get_monochrome_pal_val(self.obj0_palette_register, i);
            self.obj1_palette[i] = Ppu::get_monochrome_pal_val(self.obj1_palette_register, i);
        }
    }

    fn get_monochrome_pal_val(value: u8, index: usize) -> u8 {
        match (value >> 2 * index) & 0x03 {
            0 => 255,
            1 => 192,
            2 => 96,
            _ => 0,
        }
    }
}
