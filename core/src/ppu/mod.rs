use palette::Palette;
use std::cmp::Ordering;
use tile::{TileData, TileMap};

use crate::bus::Memory;

pub mod palette;
pub mod registers;
pub mod tile;

const VRAM_SIZE: usize = 0x4000;
const OAM_SIZE: usize = 0xA0;
pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

#[derive(PartialEq, Copy, Clone)]
enum Priority {
    Blank,
    Normal,
}

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
    object_size: u8,
    object_enabled: bool,
    bg_window_enabled: bool,
    bg_window_priority: [Priority; SCREEN_WIDTH],
    lyc_interrupt: bool,
    mode0_interrupt: bool,
    mode1_interrupt: bool,
    mode2_interrupt: bool,
    scy: u8,
    scx: u8,
    wy: u8,
    wx: u8,
    wy_trigger: bool,
    wy_position: i32,
    bg_palette: Palette,
    obj0_palette: Palette,
    obj1_palette: Palette,
    pub vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
    vrambank: usize,
    pub screen_buffer: Vec<(u8, u8, u8)>,
    pub screen_updated: bool,
    pub interrupt: u8,
}

impl Memory for Ppu {
    fn mem_read(&mut self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => self.vram[(self.vrambank * 0x2000) | (address as usize & 0x1FFF)],
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00],
            0xFF40 => self.lcdc_read(),
            0xFF41 => self.stat_read(),
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
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00] = data,
            0xFF40 => self.lcdc_write(data),
            0xFF41 => self.stat_write(data),
            0xFF42 => self.scy = data,
            0xFF43 => self.scx = data,
            0xFF44 => {} // Read-only
            0xFF45 => {
                self.lyc = data;
                self.trigger_lyc_interrupt();
            }
            0xFF46 => panic!("0xFF46 should be handled by Bus"),
            0xFF47 => self.bg_palette = Palette::from_byte(data),
            0xFF48 => self.obj0_palette = Palette::from_byte(data),
            0xFF49 => self.obj1_palette = Palette::from_byte(data),
            0xFF4A => self.wy = data,
            0xFF4B => self.wx = data,
            0xFF4C => {}
            0xFF4E => {}
            _ => panic!("PPU does not handle write {:04X}", address),
        }
    }
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            mode: Mode::HBlank,
            line_ticks: 0,
            line: 0,
            lyc: 0,
            lcd_enabled: false,
            window_tile_map: TileMap::High,
            window_enabled: false,
            tile_data: TileData::Area0,
            bg_tile_map: TileMap::High,
            object_size: 8,
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
            wy_trigger: false,
            wy_position: -1,
            bg_palette: Palette::from_byte(0),
            obj0_palette: Palette::from_byte(0),
            obj1_palette: Palette::from_byte(1),
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            screen_buffer: vec![(0, 0, 0); SCREEN_WIDTH * SCREEN_HEIGHT],
            bg_window_priority: [Priority::Normal; SCREEN_WIDTH],
            screen_updated: false,
            interrupt: 0,
            vrambank: 0,
        }
    }

    pub fn cycle(&mut self, ticks: u32) {
        if !self.lcd_enabled {
            return;
        }

        if ticks <= 0 {
            return;
        }

        self.line_ticks += ticks;
        if self.line_ticks >= 456 {
            self.line_ticks -= 456;
            self.line = (self.line + 1) % 154;
            self.trigger_lyc_interrupt();

            if self.line >= 144 && self.mode != Mode::VBlank {
                self.change_mode(Mode::VBlank);
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
            Mode::HBlank => {
                self.render_scanline();
                self.mode0_interrupt
            }
            Mode::VBlank => {
                self.wy_trigger = false;
                self.interrupt |= 0x01;
                self.screen_updated = true;
                self.mode1_interrupt
            }
            Mode::OamScan => self.mode2_interrupt,
            Mode::DrawingPixels => {
                if self.window_enabled && self.wy_trigger == false && self.line == self.wy {
                    self.wy_trigger = true;
                    self.wy_position = -1;
                }
                false
            }
        } {
            self.interrupt |= 0x02;
        }
    }

    fn clear_screen(&mut self) {
        for v in self.screen_buffer.iter_mut() {
            *v = (255, 255, 255);
        }
        self.screen_updated = true;
    }

    fn render_scanline(&mut self) {
        for x in 0..SCREEN_WIDTH {
            self.set_pixel(x, (255, 255, 255));
        }
        self.draw_bg_and_window();
        self.draw_objects();
    }

    fn set_pixel(&mut self, x: usize, color: (u8, u8, u8)) {
        self.screen_buffer[self.line as usize * SCREEN_WIDTH + x] = color;
    }

    fn draw_bg_and_window(&mut self) {
        let draw_background = self.bg_window_enabled;

        let wx_trigger = self.wx <= 166;
        let wy = if self.window_enabled && self.wy_trigger && wx_trigger {
            self.wy_position += 1;
            self.wy_position
        } else {
            -1
        };

        if wy < 0 && draw_background == false {
            return;
        }
        let window_tile_y = (wy as u16 >> 3) & 0x1F;

        let bgy = self.scy.wrapping_add(self.line);
        let bg_tile_y = (bgy as u16 >> 3) & 0x1F;

        for x in 0..SCREEN_WIDTH {
            let wx = -((self.wx as i32) - 7) + (x as i32);
            let bgx = self.scx as u32 + x as u32;

            let (tile_map, tile_y, tile_x, pixel_y, pixel_x) = if wy >= 0 && wx >= 0 {
                (self.window_tile_map, window_tile_y, (wx as u16 >> 3), wy as u16 & 0x07, wx as u8 & 0x07)
            } else if draw_background {
                (self.bg_tile_map, bg_tile_y, (bgx as u16 >> 3) & 0x1F, bgy as u16 & 0x07, bgx as u8 & 0x07)
            } else {
                continue;
            };

            let map_address = tile_map.base_address() + tile_y * 32 + tile_x;
            let tile_index: u8 = self.vram[map_address as usize];
            let base_address = self.tile_data.tile_address(tile_index);
            let tile_address = (base_address + (2 * pixel_y)) as usize;
            let (byte1, byte2) = (self.vram[tile_address], self.vram[tile_address + 1]);

            let bit = 7 - pixel_x;
            let hi = (byte2 >> bit) & 1;
            let lo = (byte1 >> bit) & 1;
            let color = hi << 1 | lo;

            self.bg_window_priority[x] = match color {
                0 => Priority::Blank,
                _ => Priority::Normal,
            };

            let color = self.bg_palette.get_color(color);
            self.set_pixel(x, color.rgb());
        }
    }

    fn draw_objects(&mut self) {
        if !self.object_enabled {
            return;
        }

        let line = self.line as i32;
        let object_size = self.object_size as i32;

        let mut objects_to_draw = [(0, 0, 0); 10];
        let mut object_index = 0;
        for index in 0..40 {
            let object_address = ((index as u16) * 4) as usize;
            let object_y = self.oam[object_address + 0] as u16 as i32 - 16;
            if line < object_y || line >= object_y + object_size {
                continue;
            }
            let object_x = self.oam[object_address + 1] as u16 as i32 - 8;
            objects_to_draw[object_index] = (object_x, object_y, index);
            object_index += 1;
            if object_index >= 10 {
                break;
            }
        }

        objects_to_draw[..object_index].sort_by(dmg_sprite_order);

        for &(object_x, object_y, i) in &objects_to_draw[..object_index] {
            if object_x < -7 || object_x >= (SCREEN_WIDTH as i32) {
                continue;
            }

            let object_address = ((i as u16) * 4) as usize;
            let tile_index = (self.oam[object_address + 2] & (if self.object_size == 16 { 0xFE } else { 0xFF })) as u16;
            let flags = self.oam[object_address + 3] as usize;
            let dmg_palette: bool = flags & (1 << 4) != 0;
            let x_flip: bool = flags & (1 << 5) != 0;
            let y_flip: bool = flags & (1 << 6) != 0;
            let priority: bool = flags & (1 << 7) != 0;

            let tile_y: u16 = if y_flip {
                (object_size - 1 - (line - object_y)) as u16
            } else {
                (line - object_y) as u16
            };

            let tile_address = (tile_index * 16 + tile_y * 2) as usize;
            let (byte1, byte2) = (self.vram[tile_address], self.vram[tile_address + 1]);

            'colorloop: for x in 0..8 {
                if object_x + x < 0 || object_x + x >= (SCREEN_WIDTH as i32) {
                    continue;
                }

                let bit = if x_flip { x } else { 7 - x };
                let hi = (byte2 >> bit) & 1;
                let lo = (byte1 >> bit) & 1;
                let color = hi << 1 | lo;
                if color == 0 {
                    continue;
                }

                if priority && self.bg_window_priority[(object_x + x) as usize] != Priority::Blank {
                    continue 'colorloop;
                }
                let color = match dmg_palette {
                    true => self.obj1_palette.get_color(color),
                    false => self.obj0_palette.get_color(color),
                };
                self.set_pixel((object_x + x) as usize, color.rgb());
            }
        }
    }
}

fn dmg_sprite_order(a: &(i32, i32, u8), b: &(i32, i32, u8)) -> Ordering {
    if a.0 != b.0 {
        return b.0.cmp(&a.0);
    }
    return b.2.cmp(&a.2);
}
