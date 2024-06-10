use palette::Palette;
use std::cmp::Ordering;

use crate::bus::Memory;

pub mod palette;

const VRAM_SIZE: usize = 0x4000;
const OAM_SIZE: usize = 0xA0;
const TILE_MAP_LOW: u16 = 0x9800;
const TILE_MAP_HIGH: u16 = 0x9C00;
const TILE_DATA_BLOCK_0: u16 = 0x8000;
const TILE_DATA_BLOCK_1: u16 = 0x8800;
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
    tile_data: u16,
    bg_tile_map: u16,
    object_size: u8,
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
            0xFF40 => {
                (if self.lcd_enabled { 0x80 } else { 0 })
                    | (if self.window_tile_map == TILE_MAP_HIGH { 0x40 } else { 0 })
                    | (if self.window_enabled { 0x20 } else { 0 })
                    | (if self.tile_data == TILE_DATA_BLOCK_0 { 0x10 } else { 0 })
                    | (if self.bg_tile_map == TILE_MAP_HIGH { 0x08 } else { 0 })
                    | (if self.object_size == 16 { 0x04 } else { 0 })
                    | (if self.object_enabled { 0x02 } else { 0 })
                    | (if self.bg_window_enabled { 0x01 } else { 0 })
            }
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
            0xFF40 => {
                let orig_lcd_on = self.lcd_enabled;
                self.lcd_enabled = data & 0x80 == 0x80;
                self.window_tile_map = if data & 0x40 == 0x40 { TILE_MAP_HIGH } else { TILE_MAP_LOW };
                self.window_enabled = data & 0x20 == 0x20;
                self.tile_data = if data & 0x10 == 0x10 { TILE_DATA_BLOCK_0 } else { TILE_DATA_BLOCK_1 };
                self.bg_tile_map = if data & 0x08 == 0x08 { TILE_MAP_HIGH } else { TILE_MAP_LOW };
                self.object_size = if data & 0x04 == 0x04 { 16 } else { 8 };
                self.object_enabled = data & 0x02 == 0x02;
                self.bg_window_enabled = data & 0x01 == 0x01;
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
            0xFF47 => self.bg_palette = Palette::from_byte(data),
            0xFF48 => self.obj0_palette = Palette::from_byte(data),
            0xFF49 => self.obj1_palette = Palette::from_byte(data),
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
            mode: Mode::HBlank,
            line_ticks: 0,
            line: 0,
            lyc: 0,
            lcd_enabled: false,
            window_tile_map: TILE_MAP_HIGH,
            window_enabled: false,
            tile_data: TILE_DATA_BLOCK_0,
            bg_tile_map: TILE_MAP_HIGH,
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
            obj1_palette: Palette::from_byte(0),
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            screen_buffer: vec![(0, 0, 0); SCREEN_WIDTH * SCREEN_HEIGHT],
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

    fn read_vram(&self, address: u16) -> u8 {
        if address < 0x8000 || address >= 0xA000 {
            panic!("address used to access vram out of bounds");
        }
        self.vram[address as usize & 0x1FFF]
    }

    fn clear_screen(&mut self) {
        for v in self.screen_buffer.iter_mut() {
            *v = (255, 255, 255);
        }
        self.screen_updated = true;
    }

    fn render_scanline(&mut self) {
        for x in 0..SCREEN_WIDTH {
            self.set_color(x, (255, 255, 255));
        }
        self.draw_bg_and_window();
        self.draw_objects();
    }

    fn set_color(&mut self, x: usize, color: (u8, u8, u8)) {
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

            let (tile_map_base, tile_y, tile_x, pixel_y, pixel_x) = if wy >= 0 && wx >= 0 {
                (self.window_tile_map, window_tile_y, (wx as u16 >> 3), wy as u16 & 0x07, wx as u8 & 0x07)
            } else if draw_background {
                (self.bg_tile_map, bg_tile_y, (bgx as u16 >> 3) & 0x1F, bgy as u16 & 0x07, bgx as u8 & 0x07)
            } else {
                continue;
            };

            let tile_index: u8 = self.read_vram(tile_map_base as u16 + tile_y * 32 + tile_x);
            let tile_address = self.tile_data as u16
                + (if self.tile_data == TILE_DATA_BLOCK_0 {
                    tile_index as u16
                } else {
                    (tile_index as i8 as i16 + 128) as u16
                }) * 16;

            let address = tile_address + (pixel_y * 2);
            let (byte1, byte2) = (self.read_vram(address), self.read_vram(address + 1));

            let bit = 7 - pixel_x as u8;
            let hi = if byte2 & (1 << bit) != 0 { 2 } else { 0 };
            let lo = if byte1 & (1 << bit) != 0 { 1 } else { 0 };
            let color = self.bg_palette.get_color(hi | lo);
            self.set_color(x, color.value());
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
            let object_address = 0xFE00 + (index as u16) * 4;
            let object_y = self.mem_read(object_address + 0) as u16 as i32 - 16;
            if line < object_y || line >= object_y + object_size {
                continue;
            }
            let object_x = self.mem_read(object_address + 1) as u16 as i32 - 8;
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

            let object_address = 0xFE00 + (i as u16) * 4;
            let tile_index = (self.mem_read(object_address + 2) & (if self.object_size == 16 { 0xFE } else { 0xFF })) as u16;
            let flags = self.mem_read(object_address + 3) as usize;
            let use_obj_palette1: bool = flags & (1 << 4) != 0;
            let x_flip: bool = flags & (1 << 5) != 0;
            let y_flip: bool = flags & (1 << 6) != 0;
            let behind_bg: bool = flags & (1 << 7) != 0;

            let tile_y: u16 = if y_flip {
                (object_size - 1 - (line - object_y)) as u16
            } else {
                (line - object_y) as u16
            };

            let tile_address = 0x8000 + tile_index * 16 + tile_y * 2;
            let (byte1, byte2) = { (self.read_vram(tile_address), self.read_vram(tile_address + 1)) };

            'colorloop: for x in 0..8 {
                if object_x + x < 0 || object_x + x >= (SCREEN_WIDTH as i32) {
                    continue;
                }

                let xbit = 1 << (if x_flip { x } else { 7 - x } as u32);
                let hi = if byte2 & xbit != 0 { 2 } else { 0 };
                let lo = if byte1 & xbit != 0 { 1 } else { 0 };
                let color_byte = hi | lo;
                if color_byte == 0 {
                    continue;
                }

                if behind_bg {
                    continue 'colorloop;
                }
                let color = if use_obj_palette1 {
                    self.obj1_palette.get_color(color_byte)
                } else {
                    self.obj0_palette.get_color(color_byte)
                };
                self.set_color((object_x + x) as usize, color.value());
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
