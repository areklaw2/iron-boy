use background::Background;
use oam::Oam;
use palette::{color_index, Palette};
use registers::{lcd_control::LcdControl, lcd_status::LcdStatus, PpuMode};
use tile::{TILE_HEIGHT, TILE_WIDTH};
use window::Window;

use crate::bus::MemoryAccess;

mod background;
mod oam;
mod palette;
mod registers;
mod tile;
mod window;

const VRAM_SIZE: usize = 0x4000;
const OAM_SIZE: usize = 40;
pub const VIEWPORT_WIDTH: usize = 160;
pub const VIEWPORT_HEIGHT: usize = 144;
pub const FULL_WIDTH: usize = 256;

pub struct Ppu {
    line_ticks: u32,
    ly: u8,
    lyc: u8,
    lcd_control: LcdControl,
    lcd_status: LcdStatus,
    background: Background,
    window: Window,
    bg_palette: Palette,
    obj0_palette: Palette,
    obj1_palette: Palette,
    pub vram: [u8; VRAM_SIZE],
    oam: [Oam; OAM_SIZE],
    oam_buffer: Vec<(usize, i16)>,
    object_height: u8,
    priority_map: [bool; FULL_WIDTH * FULL_WIDTH],
    pub screen_buffer: Vec<(u8, u8, u8)>,
    pub screen_updated: bool,
    pub interrupt: u8,
    //vrambank: usize,
}

impl MemoryAccess for Ppu {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000],
            0xFE00..=0xFE9F => self.read_oam(address - 0xFE00),
            0xFF40 => (&self.lcd_control).into(),
            0xFF41 => (&self.lcd_status).into(),
            0xFF42 => self.background.scy(),
            0xFF43 => self.background.scx(),
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => 0, // Write only
            0xFF47 => self.bg_palette.into(),
            0xFF48 => self.obj0_palette.into(),
            0xFF49 => self.obj1_palette.into(),
            0xFF4A => self.window.wy(),
            0xFF4B => self.window.wx(),
            0xFF4C => 0xFF,
            0xFF4E => 0xFF,
            _ => 0xFF,
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0x8000..=0x9FFF => self.vram[address as usize - 0x8000] = value,
            0xFE00..=0xFE9F => self.write_oam(address - 0xFE00, value),
            0xFF40 => self.set_lcd_control(value),
            0xFF41 => self.lcd_status = value.into(),
            0xFF42 => self.background.set_scy(value),
            0xFF43 => self.background.set_scx(value),
            0xFF44 => {} // Read-only
            0xFF45 => self.set_lyc(value),
            0xFF46 => panic!("0xFF46 should be handled by Bus"),
            0xFF47 => self.bg_palette = value.into(),
            0xFF48 => self.obj0_palette = value.into(),
            0xFF49 => self.obj1_palette = value.into(),
            0xFF4A => self.window.set_wy(value),
            0xFF4B => self.window.set_wx(value),
            0xFF4C => {}
            0xFF4E => {}
            _ => panic!("PPU does not handle write {:04X}", address),
        }
    }
}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            line_ticks: 0,
            ly: 0,
            lyc: 0,
            lcd_control: LcdControl::new(),
            lcd_status: LcdStatus::new(),
            background: Background::new(),
            window: Window::new(),
            bg_palette: Palette::from(0),
            obj0_palette: Palette::from(0),
            obj1_palette: Palette::from(1),
            vram: [0; VRAM_SIZE],
            oam: [Oam::new(); OAM_SIZE],
            oam_buffer: Vec::new(),
            object_height: TILE_HEIGHT,
            priority_map: [false; FULL_WIDTH * FULL_WIDTH],
            screen_buffer: vec![(0, 0, 0); VIEWPORT_WIDTH * VIEWPORT_HEIGHT],
            screen_updated: false,
            interrupt: 0,
            //vrambank: 0,
        }
    }

    pub fn cycle(&mut self, ticks: u32) {
        if !self.lcd_control.lcd_enabled() {
            return;
        }

        self.line_ticks += ticks;
        match self.lcd_status.mode() {
            PpuMode::OamScan => {
                if self.line_ticks < 80 {
                    return;
                }

                self.oam_buffer.clear();
                let ly = self.ly as i16;
                self.object_height = if self.lcd_control.object_size() { 2 * TILE_HEIGHT } else { TILE_HEIGHT };

                for i in 0..OAM_SIZE {
                    let oam_entry = self.oam[i];
                    let object_y = oam_entry.y_position() as i16 - 16;
                    let object_x = oam_entry.x_position() as i16 - 8;
                    if ly >= object_y && ly < object_y + self.object_height as i16 {
                        self.oam_buffer.push((i, object_x));
                    }
                }

                self.oam_buffer.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
                self.oam_buffer.truncate(10);
                self.oam_buffer.reverse();

                self.lcd_status.set_mode(PpuMode::DrawingPixels);
                self.line_ticks -= 80
            }
            PpuMode::DrawingPixels => {
                if self.line_ticks < 172 {
                    return;
                }

                self.render_scanline();
                if self.lcd_status.set_mode(PpuMode::HBlank) {
                    self.interrupt |= 0x02;
                }
                self.line_ticks -= 172
            }
            PpuMode::HBlank => {
                if self.line_ticks < 204 {
                    return;
                }

                if self.ly >= 143 {
                    self.screen_updated = true;
                    self.interrupt |= 0x01;
                    if self.lcd_status.set_mode(PpuMode::VBlank) {
                        self.interrupt |= 0x02;
                    }
                } else {
                    self.window.increment_line_counter(self.lcd_control.window_enabled(), self.ly);
                    self.set_ly(self.ly + 1);
                    if self.lcd_status.set_mode(PpuMode::OamScan) {
                        self.interrupt |= 0x02;
                    }
                }

                self.line_ticks -= 204
            }
            PpuMode::VBlank => {
                if self.line_ticks < 456 {
                    return;
                }

                self.set_ly(self.ly + 1);
                if self.ly > 154 {
                    self.window.reset_line_counter();
                    self.set_ly(0);
                    if self.lcd_status.set_mode(PpuMode::OamScan) {
                        self.interrupt |= 0x02;
                    }
                }

                self.line_ticks -= 456
            }
        }
    }

    fn read_oam(&self, address: u16) -> u8 {
        let index = (address / 4) as usize;
        let attribute = (address % 4) as usize;
        match attribute {
            0 => self.oam[index].y_position(),
            1 => self.oam[index].x_position(),
            2 => self.oam[index].tile_index(),
            3 => self.oam[index].attributes(),
            _ => unreachable!(),
        }
    }

    fn write_oam(&mut self, address: u16, value: u8) {
        let index = (address / 4) as usize;
        let attribute = (address % 4) as usize;

        match attribute {
            0 => self.oam[index].set_y_position(value),
            1 => self.oam[index].set_x_position(value),
            2 => self.oam[index].set_tile_index(value),
            3 => self.oam[index].set_attributes(value),
            _ => unreachable!(),
        }
    }

    fn set_ly(&mut self, value: u8) {
        self.ly = value;
        self.compare_line();
    }

    pub fn set_lyc(&mut self, value: u8) {
        self.lyc = value;
        self.compare_line();
    }

    fn compare_line(&mut self) {
        self.lcd_status.set_lyc_equals_ly(false);
        if self.lyc != self.ly {
            return;
        }

        self.lcd_status.set_lyc_equals_ly(true);
        if self.lcd_status.lyc_interrupt() {
            self.interrupt |= 0x02;
        }
    }

    fn set_lcd_control(&mut self, value: u8) {
        self.lcd_control = value.into();
        if !self.lcd_control.lcd_enabled() {
            self.clear_screen();
            self.window.reset_line_counter();
            self.set_ly(0);
            self.lcd_status.mode = PpuMode::HBlank;
            self.line_ticks = 0;
        }
    }

    fn clear_screen(&mut self) {
        self.screen_buffer.fill((255, 255, 255));
        self.priority_map.fill(false);
        self.screen_updated = true;
    }

    fn render_scanline(&mut self) {
        if self.lcd_control.bg_window_enabled() {
            self.render_bg_window_line();
        }

        if self.lcd_control.object_enabled() {
            self.render_object_line();
        }
    }

    fn render_bg_window_line(&mut self) {
        for lx in 0..VIEWPORT_WIDTH as u8 {
            let (tile_index_address, x_offset, y_offset) = self.bg_window_tile_data(lx);

            let tile_index = self.read_8(tile_index_address);
            let tile_address = self.lcd_control.tile_data().tile_address(tile_index);

            let (byte1, byte2) = self.get_tile_bytes(tile_address + y_offset as u16);

            let color_index = color_index(byte1, byte2, x_offset);
            let priority_offset = self.ly as usize + FULL_WIDTH * lx as usize;
            self.priority_map[priority_offset] = color_index != 0;

            let color = self.bg_palette.pixel_color(color_index);
            let offset = lx as usize + self.ly as usize * VIEWPORT_WIDTH;
            self.screen_buffer[offset] = color.into();
        }
    }

    fn bg_window_tile_data(&self, lx: u8) -> (u16, u8, u8) {
        if self.window.inside_window(self.lcd_control.window_enabled(), lx, self.ly) {
            let (x, y) = self.window.tile_map_coordinates(lx);
            let tile_index_address = self.lcd_control.window_tile_map().tile_index_address(x, y);

            let (x_offset, y_offset) = self.window.pixel_offsets(lx, self.ly);
            (tile_index_address, x_offset, y_offset)
        } else {
            let (x, y) = self.background.tile_map_coordinates(lx, self.ly);
            let tile_index_address = self.lcd_control.bg_tile_map().tile_index_address(x, y);

            let (x_offset, y_offset) = self.background.pixel_offsets(x, y);
            (tile_index_address, x_offset, y_offset)
        }
    }

    fn render_object_line(&mut self) {
        let ly = self.ly as i16;
        for (oam_index, x_offset) in self.oam_buffer.iter() {
            let oam_entry = self.oam[*oam_index];
            let y_offset = oam_entry.y_position() as i16 - 16;

            let mut tile_index = oam_entry.tile_index();
            if self.object_height == 2 * TILE_HEIGHT {
                tile_index &= 0xFE;
            }

            let tile_base_address = 0x8000 + (tile_index as u16 * 16);
            let line_offset = if oam_entry.flags.y_flip() {
                self.object_height as i16 - 1 - (ly - y_offset)
            } else {
                self.ly as i16 - y_offset
            };

            let tile_address = tile_base_address + line_offset as u16 * 2;
            let (byte1, byte2) = self.get_tile_bytes(tile_address);

            for pixel_index in 0..TILE_WIDTH {
                let x_offset = x_offset + pixel_index as i16;
                if !(0..VIEWPORT_WIDTH).contains(&(x_offset as usize)) {
                    continue;
                }

                let priority = ly as usize + FULL_WIDTH * x_offset as usize;
                if self.is_overlapping(&oam_entry, priority) {
                    continue;
                }

                let oam_pixel_index = if oam_entry.flags.x_flip() { pixel_index } else { 7 - pixel_index };
                let color_index = color_index(byte1, byte2, oam_pixel_index);
                if color_index == 0 {
                    continue;
                }

                let object_pallete = if oam_entry.flags.dmg_palette() {
                    self.obj1_palette
                } else {
                    self.obj0_palette
                };

                let color = object_pallete.pixel_color(color_index);
                let offset = x_offset as usize + ly as usize * VIEWPORT_WIDTH;
                self.screen_buffer[offset] = color.into();
            }
        }
    }

    fn get_tile_bytes(&self, address: u16) -> (u8, u8) {
        let byte1 = self.read_8(address);
        let byte2 = self.read_8(address + 1);
        (byte1, byte2)
    }

    fn is_overlapping(&self, oam: &Oam, offset: usize) -> bool {
        if !oam.flags.priority() {
            return false;
        }
        self.priority_map[offset]
    }
}
