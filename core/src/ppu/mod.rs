use palette::Palette;
use registers::PpuMode;
use std::cmp::Ordering;
use tile::{TileDataAddressingMode, TileMap};

use crate::bus::MemoryAccess;

mod palette;
mod registers;
mod tile;

const VRAM_SIZE: usize = 0x4000;
const OAM_SIZE: usize = 0xA0;
pub const VIEWPORT_WIDTH: usize = 160;
pub const VIEWPORT_HEIGHT: usize = 144;

#[derive(PartialEq, Copy, Clone)]
enum Priority {
    Blank,
    Normal,
}

pub struct Ppu {
    mode: PpuMode,
    line_ticks: u32,
    line: u8,
    lyc: u8,
    lcd_enabled: bool,
    window_tile_map: TileMap,
    window_enabled: bool,
    tile_data: TileDataAddressingMode,
    bg_tile_map: TileMap,
    object_size: u8,
    object_enabled: bool,
    bg_window_enabled: bool,
    bg_window_priority: [Priority; VIEWPORT_WIDTH],
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

impl MemoryAccess for Ppu {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000],
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00],
            0xFF40 => self.lcdc_read(),
            0xFF41 => self.stat_read(),
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.line,
            0xFF45 => self.lyc,
            0xFF46 => 0, // Write only
            0xFF47 => self.bg_palette.into(),
            0xFF48 => self.obj0_palette.into(),
            0xFF49 => self.obj1_palette.into(),
            0xFF4A => self.wy,
            0xFF4B => self.wx,
            0xFF4C => 0xFF,
            0xFF4E => 0xFF,
            _ => 0xFF,
        }
    }

    fn write_8(&mut self, address: u16, data: u8) {
        match address {
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000] = data,
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
            0xFF47 => self.bg_palette = data.into(),
            0xFF48 => self.obj0_palette = data.into(),
            0xFF49 => self.obj1_palette = data.into(),
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
            mode: PpuMode::HBlank,
            line_ticks: 0,
            line: 0,
            lyc: 0,
            lcd_enabled: false,
            window_tile_map: TileMap::High,
            window_enabled: false,
            tile_data: TileDataAddressingMode::Low,
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
            bg_palette: Palette::from(0),
            obj0_palette: Palette::from(0),
            obj1_palette: Palette::from(1),
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            screen_buffer: vec![(0, 0, 0); VIEWPORT_WIDTH * VIEWPORT_HEIGHT],
            bg_window_priority: [Priority::Normal; VIEWPORT_WIDTH],
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

            if self.line >= 144 && self.mode != PpuMode::VBlank {
                self.change_mode(PpuMode::VBlank);
            }
        }

        if self.line < 144 {
            match self.line_ticks {
                0..=80 => {
                    if self.mode != PpuMode::OamScan {
                        self.change_mode(PpuMode::OamScan)
                    }
                }
                81..=252 => {
                    if self.mode != PpuMode::DrawingPixels {
                        self.change_mode(PpuMode::DrawingPixels)
                    }
                }
                _ => {
                    if self.mode != PpuMode::HBlank {
                        self.change_mode(PpuMode::HBlank);
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

    fn change_mode(&mut self, mode: PpuMode) {
        self.mode = mode;

        if match self.mode {
            PpuMode::HBlank => {
                self.render_scanline();
                self.mode0_interrupt
            }
            PpuMode::VBlank => {
                self.wy_trigger = false;
                self.interrupt |= 0x01;
                self.screen_updated = true;
                self.mode1_interrupt
            }
            PpuMode::OamScan => self.mode2_interrupt,
            PpuMode::DrawingPixels => {
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
        for x in 0..VIEWPORT_WIDTH {
            self.set_pixel(x, (255, 255, 255));
        }
        self.draw_bg_and_window();
        self.draw_objects();
    }

    fn set_pixel(&mut self, x: usize, color: (u8, u8, u8)) {
        self.screen_buffer[self.line as usize * VIEWPORT_WIDTH + x] = color;
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

        for x in 0..VIEWPORT_WIDTH {
            let wx = -((self.wx as i32) - 7) + (x as i32);
            let bgx = self.scx as u32 + x as u32;

            let (tile_map, tile_y, tile_x, pixel_y, pixel_x) = if wy >= 0 && wx >= 0 {
                (self.window_tile_map, window_tile_y, (wx as u16 >> 3), wy as u16 & 0x07, wx as u8 & 0x07)
            } else if draw_background {
                (self.bg_tile_map, bg_tile_y, (bgx as u16 >> 3) & 0x1F, bgy as u16 & 0x07, bgx as u8 & 0x07)
            } else {
                continue;
            };

            let tile_map_address = tile_map.base_address() + tile_y * 32 + tile_x;
            let tile_index: u8 = self.read_8(tile_map_address);
            let tile_address = self.tile_data.tile_address(tile_index);
            let tile_pixel_row_address = tile_address + (2 * pixel_y);
            let (byte1, byte2) = (self.read_8(tile_pixel_row_address), self.read_8(tile_pixel_row_address + 1));

            let pixel = 7 - pixel_x;
            let hi = (byte2 >> pixel) & 1;
            let lo = (byte1 >> pixel) & 1;
            let color_index = hi << 1 | lo;

            self.bg_window_priority[x] = match color_index {
                0 => Priority::Blank,
                _ => Priority::Normal,
            };

            let color = self.bg_palette.pixel_color(color_index);
            self.set_pixel(x, color.into());
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
            if object_x < -7 || object_x >= (VIEWPORT_WIDTH as i32) {
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

            let tile_pixel_row_address = (tile_index * 16 + tile_y * 2) as usize;
            let (byte1, byte2) = (self.vram[tile_pixel_row_address], self.vram[tile_pixel_row_address + 1]);

            'colorloop: for x in 0..8 {
                if object_x + x < 0 || object_x + x >= (VIEWPORT_WIDTH as i32) {
                    continue;
                }

                let pixel = if x_flip { x } else { 7 - x };
                let hi = (byte2 >> pixel) & 1;
                let lo = (byte1 >> pixel) & 1;
                let color_index = hi << 1 | lo;
                if color_index == 0 {
                    continue;
                }

                if priority && self.bg_window_priority[(object_x + x) as usize] != Priority::Blank {
                    continue 'colorloop;
                }
                let color = match dmg_palette {
                    true => self.obj1_palette.pixel_color(color_index),
                    false => self.obj0_palette.pixel_color(color_index),
                };
                self.set_pixel((object_x + x) as usize, color.into());
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
