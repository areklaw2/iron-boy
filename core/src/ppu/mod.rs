use std::cmp::Ordering;

use crate::bus::Memory;

const VRAM_SIZE: usize = 0x4000;
const OAM_SIZE: usize = 0xA0;
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
    tile_dara: u16,
    bg_tile_map: u16,
    object_size: u32,
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
    wy_pos: i32,
    bg_palette_register: u8,
    obj0_palette_register: u8,
    obj1_palette_register: u8,
    bg_palette: [u8; 4],
    obj0_palette: [u8; 4],
    obj1_palette: [u8; 4],
    pub vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
    vrambank: usize,
    pub screen_buffer: Vec<u8>,
    pub screen_updated: bool,
    pub interrupt: u8,
    hblanking: bool,
}

impl Memory for Ppu {
    fn mem_read(&mut self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => self.vram[(self.vrambank * 0x2000) | (address as usize & 0x1FFF)],
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00],
            0xFF40 => {
                (if self.lcd_enabled { 0x80 } else { 0 })
                    | (if self.window_tile_map == 0x9C00 { 0x40 } else { 0 })
                    | (if self.window_enabled { 0x20 } else { 0 })
                    | (if self.tile_dara == 0x8000 { 0x10 } else { 0 })
                    | (if self.bg_tile_map == 0x9C00 { 0x08 } else { 0 })
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
            0xFF47 => self.bg_palette_register,
            0xFF48 => self.obj0_palette_register,
            0xFF49 => self.obj1_palette_register,
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
                self.window_tile_map = if data & 0x40 == 0x40 { 0x9C00 } else { 0x9800 };
                self.window_enabled = data & 0x20 == 0x20;
                self.tile_dara = if data & 0x10 == 0x10 { 0x8000 } else { 0x8800 };
                self.bg_tile_map = if data & 0x08 == 0x08 { 0x9C00 } else { 0x9800 };
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
            window_tile_map: 0x9C00,
            window_enabled: false,
            tile_dara: 0x8000,
            bg_tile_map: 0x9C00,
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
            wy_pos: -1,
            bg_palette_register: 0,
            obj0_palette_register: 0,
            obj1_palette_register: 1,
            bg_palette: [0; 4],
            obj0_palette: [0; 4],
            obj1_palette: [0; 4],
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            screen_buffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
            screen_updated: false,
            interrupt: 0,
            vrambank: 0,
            hblanking: false,
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
                self.hblanking = true;
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
                    self.wy_pos = -1;
                }
                false
            }
        } {
            self.interrupt |= 0x02;
        }
    }

    fn rbvram0(&self, a: u16) -> u8 {
        if a < 0x8000 || a >= 0xA000 {
            panic!("Shouldn't have used rbvram0");
        }
        self.vram[a as usize & 0x1FFF]
    }

    fn clear_screen(&mut self) {
        for v in self.screen_buffer.iter_mut() {
            *v = 255;
        }
        self.screen_updated = true;
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

    fn render_scanline(&mut self) {
        for x in 0..SCREEN_WIDTH {
            self.set_color(x, 255);
        }
        self.draw_bg();
        self.draw_sprites();
    }

    fn set_color(&mut self, x: usize, color: u8) {
        self.screen_buffer[self.line as usize * SCREEN_WIDTH * 3 + x * 3 + 0] = color;
        self.screen_buffer[self.line as usize * SCREEN_WIDTH * 3 + x * 3 + 1] = color;
        self.screen_buffer[self.line as usize * SCREEN_WIDTH * 3 + x * 3 + 2] = color;
    }

    fn draw_bg(&mut self) {
        let drawbg = self.bg_window_enabled;

        let wx_trigger = self.wx <= 166;
        let winy = if self.window_enabled && self.wy_trigger && wx_trigger {
            self.wy_pos += 1;
            self.wy_pos
        } else {
            -1
        };

        if winy < 0 && drawbg == false {
            return;
        }

        let wintiley = (winy as u16 >> 3) & 31;

        let bgy = self.scy.wrapping_add(self.line);
        let bgtiley = (bgy as u16 >> 3) & 31;

        for x in 0..SCREEN_WIDTH {
            let winx = -((self.wx as i32) - 7) + (x as i32);
            let bgx = self.scx as u32 + x as u32;

            let (tilemapbase, tiley, tilex, pixely, pixelx) = if winy >= 0 && winx >= 0 {
                (self.window_tile_map, wintiley, (winx as u16 >> 3), winy as u16 & 0x07, winx as u8 & 0x07)
            } else if drawbg {
                (self.bg_tile_map, bgtiley, (bgx as u16 >> 3) & 31, bgy as u16 & 0x07, bgx as u8 & 0x07)
            } else {
                continue;
            };

            let tilenr: u8 = self.rbvram0(tilemapbase + tiley * 32 + tilex);

            let (xflip, yflip) = (false, false);

            let tileaddress = self.tile_dara
                + (if self.tile_dara == 0x8000 {
                    tilenr as u16
                } else {
                    (tilenr as i8 as i16 + 128) as u16
                }) * 16;

            let a0 = match yflip {
                false => tileaddress + (pixely * 2),
                true => tileaddress + (14 - (pixely * 2)),
            };

            let (b1, b2) = (self.rbvram0(a0), self.rbvram0(a0 + 1));

            let xbit = match xflip {
                true => pixelx,
                false => 7 - pixelx,
            } as u32;
            let colnr = if b1 & (1 << xbit) != 0 { 1 } else { 0 } | if b2 & (1 << xbit) != 0 { 2 } else { 0 };

            let color = self.bg_palette[colnr];
            self.set_color(x, color);
        }
    }

    fn draw_sprites(&mut self) {
        if !self.object_enabled {
            return;
        }

        let line = self.line as i32;
        let sprite_size = self.object_size as i32;

        let mut sprites_to_draw = [(0, 0, 0); 10];
        let mut sidx = 0;
        for index in 0..40 {
            let spriteaddr = 0xFE00 + (index as u16) * 4;
            let spritey = self.mem_read(spriteaddr + 0) as u16 as i32 - 16;
            if line < spritey || line >= spritey + sprite_size {
                continue;
            }
            let spritex = self.mem_read(spriteaddr + 1) as u16 as i32 - 8;
            sprites_to_draw[sidx] = (spritex, spritey, index);
            sidx += 1;
            if sidx >= 10 {
                break;
            }
        }

        sprites_to_draw[..sidx].sort_unstable_by(dmg_sprite_order);

        for &(spritex, spritey, i) in &sprites_to_draw[..sidx] {
            if spritex < -7 || spritex >= (SCREEN_WIDTH as i32) {
                continue;
            }

            let spriteaddr = 0xFE00 + (i as u16) * 4;
            let tilenum = (self.mem_read(spriteaddr + 2) & (if self.object_size == 16 { 0xFE } else { 0xFF })) as u16;
            let flags = self.mem_read(spriteaddr + 3) as usize;
            let usepal1: bool = flags & (1 << 4) != 0;
            let xflip: bool = flags & (1 << 5) != 0;
            let yflip: bool = flags & (1 << 6) != 0;
            let belowbg: bool = flags & (1 << 7) != 0;

            let tiley: u16 = if yflip {
                (sprite_size - 1 - (line - spritey)) as u16
            } else {
                (line - spritey) as u16
            };

            let tileaddress = 0x8000u16 + tilenum * 16 + tiley * 2;
            let (b1, b2) = { (self.rbvram0(tileaddress), self.rbvram0(tileaddress + 1)) };

            'xloop: for x in 0..8 {
                if spritex + x < 0 || spritex + x >= (SCREEN_WIDTH as i32) {
                    continue;
                }

                let xbit = 1 << (if xflip { x } else { 7 - x } as u32);
                let colnr = (if b1 & xbit != 0 { 1 } else { 0 }) | (if b2 & xbit != 0 { 2 } else { 0 });
                if colnr == 0 {
                    continue;
                }

                if belowbg {
                    continue 'xloop;
                }
                let color = if usepal1 { self.obj1_palette[colnr] } else { self.obj0_palette[colnr] };
                self.set_color((spritex + x) as usize, color);
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
