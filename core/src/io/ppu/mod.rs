use palette::Palette;
use registers::{LcdControl, LcdStatus};

use crate::bus::Memory;

pub mod object_attribute;
pub mod palette;
pub mod registers;
pub mod tile;

const VRAM_SIZE: usize = 0x4000;
const OAM_SIZE: usize = 160;
pub const SCREEN_WIDTH: usize = 160; //x
pub const SCREEN_HEIGHT: usize = 144; //y
pub const LINES_PER_FRAME: u8 = 154;
pub const TICKS_PER_LINE: u32 = 456;

#[derive(Clone, Copy, PartialEq)]
enum PpuMode {
    OamScan = 2,
    DrawingPixels = 3,
    HBlank = 0,
    VBlank = 1,
}

pub struct Ppu {
    pub vram: [u8; VRAM_SIZE],
    oam: [u8; OAM_SIZE],
    vram_bank: usize,
    clock: u32,
    ppu_mode: PpuMode,
    lcd_control: LcdControl,
    lcd_status: LcdStatus,
    scroll_y: u8,
    scroll_x: u8,
    lcd_y: u8,
    lcd_y_compare: u8,
    //dma: u8,
    bg_palette_data: Palette,
    obj_palette_data0: Palette,
    obj_palette_data1: Palette,
    window_y: u8,
    window_x: u8,

    //background: [[Tile; 32]; 32],
    //window: [[Tile; 32]; 32],
    pub interrupt: u8,
    pub updated: bool,
    pub video_buffer: Vec<u8>,
    wy_trigger: bool, // not sure yet
    wy_pos: i32,      // not sure yet
}

impl Memory for Ppu {
    fn mem_read(&mut self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => self.vram[(self.vram_bank * 0x2000) | (address as usize & 0x1FFF)],
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00],
            0xFF40 => self.lcd_control.read(),
            0xFF41 => self.lcd_status.read(self.lcd_y, self.lcd_y_compare, self.ppu_mode as u8),
            0xFF42 => self.scroll_y,
            0xFF43 => self.scroll_x,
            0xFF44 => self.lcd_y,
            0xFF45 => self.lcd_y_compare,
            0xFF46 => 0, //write-only
            0xFF47 => self.bg_palette_data.read_as_byte(),
            0xFF48 => self.obj_palette_data0.read_as_byte(),
            0xFF49 => self.obj_palette_data1.read_as_byte(),
            0xFF4A => self.window_y,
            0xFF4B => self.window_x,
            0xFF4D..=0xFF4F => todo!("CGB registers for speed switch and VRAM bank select"),
            0xFF68..=0xFF6C => todo!("CGB registers for BF and OBJ palettes"),
            _ => panic!("PPU does not handle read to address {:4X}", address),
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        match address {
            0x8000..=0x9FFF => self.vram[(self.vram_bank * 0x2000) | (address as usize & 0x1FFF)] = data,
            0xFE00..=0xFE9F => self.oam[address as usize - 0xFE00] = data,
            0xFF40 => {
                let was_lcd_enabled = self.lcd_control.lcd_enabled;
                self.lcd_control.write(data);
                if was_lcd_enabled && !self.lcd_control.lcd_enabled {
                    self.clock = 0;
                    self.lcd_y = 0;
                    self.ppu_mode = PpuMode::HBlank;
                    self.wy_trigger = false;
                    self.clear_screen();
                }

                if !was_lcd_enabled && self.lcd_control.lcd_enabled {
                    self.clock = 4;
                    self.change_mode(PpuMode::OamScan);
                }
            }
            0xFF41 => self.lcd_status.write(data),
            0xFF42 => self.scroll_y = data,
            0xFF43 => self.scroll_x = data,
            0xFF44 => {} //read_only
            0xFF45 => {
                self.lcd_y_compare = data;
                self.handle_lyc_interrupt();
            }
            0xFF46 => panic!("0xFF46 should be handled by Bus"),
            0xFF47 => self.bg_palette_data.write(data),
            0xFF48 => self.obj_palette_data0.write(data),
            0xFF49 => self.obj_palette_data1.write(data),
            0xFF4A => self.window_y = data,
            0xFF4B => self.window_x = data,
            0xFF4D..=0xFF4F => todo!("CGB registers for speed switch and VRAM bank select"),
            0xFF68..=0xFF6C => todo!("CGB registers for BF and OBJ palettes"),
            _ => panic!("PPU does not handle write to address {:4X}", address),
        }
    }
}

impl Ppu {
    pub fn new() -> Self {
        Ppu {
            vram: [0; VRAM_SIZE],
            oam: [0; OAM_SIZE],
            vram_bank: 0,
            clock: 0,
            ppu_mode: PpuMode::OamScan,
            lcd_control: LcdControl::new(),
            lcd_status: LcdStatus::new(),
            scroll_y: 0,
            scroll_x: 0,
            lcd_y: 0,
            lcd_y_compare: 0,
            //dma: 0,
            bg_palette_data: Palette::new(),
            obj_palette_data0: Palette::new(),
            obj_palette_data1: Palette::new(),
            window_y: 0,
            window_x: 0,

            //background: [[Tile::default(); 32]; 32],
            //window: [[Tile::default(); 32]; 32],
            interrupt: 0,
            updated: false,
            video_buffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT * 3],
            wy_trigger: false,
            wy_pos: -1,
        }
    }

    pub fn cycle(&mut self, ticks: u32) {
        if !self.lcd_control.lcd_enabled {
            return;
        }

        let mut ticks_remaining = ticks;

        while ticks_remaining > 0 {
            let current_ticks = if ticks_remaining >= 80 { 80 } else { ticks_remaining };
            self.clock += current_ticks;
            ticks_remaining -= current_ticks;

            // Full line takes 114 ticks
            if self.clock >= TICKS_PER_LINE {
                self.clock -= TICKS_PER_LINE;
                self.lcd_y = (self.lcd_y + 1) % LINES_PER_FRAME;
                self.handle_lyc_interrupt();

                // This is a VBlank line
                if self.lcd_y >= SCREEN_HEIGHT as u8 && self.ppu_mode != PpuMode::VBlank {
                    self.change_mode(PpuMode::VBlank);
                }
            }

            // This is a normal line
            if self.lcd_y < SCREEN_HEIGHT as u8 {
                if self.clock <= 80 {
                    if self.ppu_mode != PpuMode::OamScan {
                        self.change_mode(PpuMode::OamScan);
                    }
                } else if self.clock <= (80 + 172) {
                    // 252 cycles
                    if self.ppu_mode != PpuMode::DrawingPixels {
                        self.change_mode(PpuMode::DrawingPixels);
                    }
                } else {
                    // the remaining 204
                    if self.ppu_mode != PpuMode::HBlank {
                        self.change_mode(PpuMode::HBlank);
                    }
                }
            }
        }
    }

    fn change_mode(&mut self, ppu_mode: PpuMode) {
        self.ppu_mode = ppu_mode;

        let interrupt_trigger = match self.ppu_mode {
            PpuMode::HBlank => {
                self.render_scan_line();
                self.lcd_status.mode0_interrupt
            }
            PpuMode::VBlank => {
                self.wy_trigger = false;
                self.interrupt |= 0b1;
                self.updated = true;
                self.lcd_status.mode1_interrupt
            }
            PpuMode::OamScan => self.lcd_status.mode1_interrupt,
            PpuMode::DrawingPixels => {
                if self.lcd_control.window_enabled && self.wy_trigger == false && self.lcd_y == self.window_y {
                    self.wy_trigger = true;
                    self.wy_pos = -1;
                }
                false
            }
        };

        if interrupt_trigger {
            self.interrupt |= 0b10;
        }
    }

    fn handle_lyc_interrupt(&mut self) {
        if self.lcd_status.lyc_interrupt && self.lcd_y == self.lcd_y_compare {
            self.interrupt |= 0b10;
        }
    }

    fn clear_screen(&mut self) {
        for data in self.video_buffer.iter_mut() {
            *data = 255;
        }
        self.updated = true;
    }

    fn render_scan_line(&mut self) {
        for x in 0..SCREEN_WIDTH {
            self.set_color(x, 255);
        }
        self.draw_background();
        //self.draw_sprites();
    }

    fn set_color(&mut self, x: usize, color: u8) {
        self.video_buffer[self.lcd_y as usize * SCREEN_WIDTH * 3 + x * 3 + 0] = color;
        self.video_buffer[self.lcd_y as usize * SCREEN_WIDTH * 3 + x * 3 + 1] = color;
        self.video_buffer[self.lcd_y as usize * SCREEN_WIDTH * 3 + x * 3 + 2] = color;
    }

    fn draw_background(&mut self) {
        let draw_background = self.lcd_control.window_and_bg_enabled;

        let wx_trigger = self.window_x <= 166;
        let window_y = if self.lcd_control.window_enabled && self.wy_trigger && wx_trigger {
            self.wy_pos += 1;
            self.wy_pos
        } else {
            -1
        };

        if window_y < 0 && draw_background == false {
            return;
        }

        let window_tile_y = (window_y as u16 >> 3) & 31;

        let background_y = self.scroll_y.wrapping_add(self.lcd_y);
        let background_tile_y = (background_y as u16 >> 3) & 31;

        for x in 0..SCREEN_WIDTH {
            let window_x = -((self.window_x as i32) - 7) + (x as i32);
            let background_x = self.scroll_x as u32 + x as u32;

            let (tile_map_location, tiley, tilex, pixely, pixelx) = if window_y >= 0 && window_x >= 0 {
                (
                    self.lcd_control.window_tile_map_location,
                    window_tile_y,
                    (window_x as u16 >> 3),
                    window_y as u16 & 0x07,
                    window_x as u8 & 0x07,
                )
            } else if draw_background {
                (
                    self.lcd_control.bg_tile_map_location,
                    background_tile_y,
                    (background_x as u16 >> 3) & 31,
                    background_y as u16 & 0x07,
                    background_x as u8 & 0x07,
                )
            } else {
                continue;
            };

            let tilenr: u8 = self.rbvram0(tile_map_location + tiley * 32 + tilex);

            let tile_address = self.lcd_control.bg_and_window_tiles_location
                + (if self.lcd_control.bg_and_window_tiles_location == 0x8000 {
                    tilenr as u16
                } else {
                    (tilenr as i8 as i16 + 128) as u16
                }) * 16;

            let a0 = tile_address + (pixely * 2);
            let (b1, b2) = (self.rbvram0(a0), self.rbvram0(a0 + 1));
            let xbit = 7 - pixelx as u32;
            let colnr = if b1 & (1 << xbit) != 0 { 1 } else { 0 } | if b2 & (1 << xbit) != 0 { 2 } else { 0 };

            let color = self.bg_palette_data.read_as_color_values()[colnr];
            self.set_color(x, color);
        }
    }

    fn rbvram0(&self, a: u16) -> u8 {
        if a < 0x8000 || a >= 0xA000 {
            panic!("Shouldn't have used rbvram0");
        }
        self.vram[a as usize & 0x1FFF]
    }
}
