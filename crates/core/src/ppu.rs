use std::{cell::RefCell, rc::Rc};

use background::Background;
use getset::{CopyGetters, Getters, Setters};
use palette::{CgbPalette, Palette, color_index};
use registers::{LcdControl, LcdStatus, PpuMode};
use tile::{TILE_HEIGHT, TILE_WIDTH};
use window::Window;

use crate::{
    GbMode, T_CYCLES_PER_STEP,
    cpu::CPU_CLOCK_SPEED,
    ppu::{
        background::BgMapAttributes,
        oam::{OAM_SIZE, Oam},
    },
    system_bus::SystemMemoryAccess,
};

mod background;
mod oam;
mod palette;
pub mod registers;
mod tile;
mod window;

pub const VIEWPORT_WIDTH: usize = 160;
pub const VIEWPORT_HEIGHT: usize = 144;
const FULL_WIDTH: usize = 256;
const TOTAL_LINE_CYCLES: u16 = 456;
const NUMBER_OF_LINES: u8 = 154;
pub const FPS: f32 = CPU_CLOCK_SPEED as f32 / (NUMBER_OF_LINES as f32 * TOTAL_LINE_CYCLES as f32);

const VRAM_SIZE: usize = 0x4000;

const OAM_SCAN_CYCLES: u16 = 80;
const DRAWING_PIXELS_CYCLES: u16 = 172;
const HBLANK_CYCLES: u16 = 204;
const VBLANK_CYCLES: u16 = TOTAL_LINE_CYCLES;

#[derive(Getters, CopyGetters, Setters)]
pub struct Ppu {
    ly: u8,
    lyc: u8,
    lcd_control: LcdControl,
    lcd_status: LcdStatus,
    background: Background,
    window: Window,
    bg_palette: Palette,
    obj0_palette: Palette,
    obj1_palette: Palette,
    cgb_bg_palette: CgbPalette,
    cgb_obj_palette: CgbPalette,
    pub vram: [u8; VRAM_SIZE],
    oam: Oam,
    oam_buffer: Vec<(usize, u8)>,
    object_height: u8,
    line_priority: [(u8, bool); VIEWPORT_WIDTH],
    #[getset(get = "pub")]
    frame_buffer: Vec<(u8, u8, u8)>,
    vram_bank: usize,
    gb_mode: GbMode,
    interrupt_flag: Rc<RefCell<u8>>,
    mode_cycles: u16,
    #[getset(get_copy = "pub", set = "pub")]
    frame_ready: bool,
}

impl SystemMemoryAccess for Ppu {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0x8000..=0x9FFF => self.vram[(self.vram_bank * 0x2000) | (address as usize & 0x1FFF)],
            0xFE00..=0xFE9F => self.oam.read_8(address),
            0xFF40 => self.lcd_control.into(),
            0xFF41 => self.lcd_status.into(),
            0xFF42 | 0xFF43 => self.background.read_8(address),
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => 0,
            0xFF47 => self.bg_palette.read(),
            0xFF48 => self.obj0_palette.read(),
            0xFF49 => self.obj1_palette.read(),
            0xFF4A | 0xFF4B => self.window.read_8(address),
            0xFF4C => 0xFF,
            0xFF4E => 0xFF,
            0xFF4F..=0xFF6B if self.gb_mode != GbMode::Color => 0xFF,
            0xFF4F => self.vram_bank as u8 | 0xFE,
            0xFF68 => self.cgb_bg_palette.read_spec_and_index(),
            0xFF69 => self.cgb_bg_palette.read_palette(),
            0xFF6A => self.cgb_obj_palette.read_spec_and_index(),
            0xFF6B => self.cgb_obj_palette.read_palette(),
            _ => 0xFF,
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0x8000..=0x9FFF => self.vram[(self.vram_bank * 0x2000) | (address as usize & 0x1FFF)] = value,
            0xFE00..=0xFE9F => self.oam.write_8(address, value),
            0xFF40 => self.set_lcd_control(value),
            0xFF41 => self.lcd_status = value.into(),
            0xFF42 | 0xFF43 => self.background.write_8(address, value),
            0xFF44 => {}
            0xFF45 => self.set_lyc(value),
            0xFF47 => self.bg_palette.write(value),
            0xFF48 => self.obj0_palette.write(value),
            0xFF49 => self.obj1_palette.write(value),
            0xFF4A | 0xFF4B => self.window.write_8(address, value),
            0xFF4C => {}
            0xFF4E => {}
            0xFF4F..=0xFF6B if self.gb_mode != GbMode::Color => {}
            0xFF4F => self.vram_bank = (value & 0x01) as usize,
            0xFF68 => self.cgb_bg_palette.write_spec_and_index(value),
            0xFF69 => self.cgb_bg_palette.write_palette(value),
            0xFF6A => self.cgb_obj_palette.write_spec_and_index(value),
            0xFF6B => self.cgb_obj_palette.write_palette(value),
            _ => panic!("PPU does not handle write {:#04X}", address),
        }
    }
}

impl Ppu {
    pub fn new(mode: GbMode, interrupt_flag: Rc<RefCell<u8>>) -> Ppu {
        Ppu {
            ly: 0,
            lyc: 0,
            lcd_control: LcdControl::new(),
            lcd_status: LcdStatus::new(),
            background: Background::new(),
            window: Window::new(),
            bg_palette: Palette::new(0),
            obj0_palette: Palette::new(0),
            obj1_palette: Palette::new(1),
            cgb_bg_palette: CgbPalette::new(),
            cgb_obj_palette: CgbPalette::new(),
            vram: [0; VRAM_SIZE],
            oam: Oam::new(),
            oam_buffer: Vec::new(),
            object_height: TILE_HEIGHT,
            line_priority: [(0, false); VIEWPORT_WIDTH],
            frame_buffer: vec![(0, 0, 0); VIEWPORT_WIDTH * VIEWPORT_HEIGHT],
            vram_bank: 0,
            gb_mode: mode,
            interrupt_flag,
            mode_cycles: 0,
            frame_ready: false,
        }
    }

    pub fn cycle(&mut self) {
        if !self.lcd_control.lcd_enabled() {
            return;
        }

        self.mode_cycles += T_CYCLES_PER_STEP as u16;
        match self.lcd_status.mode() {
            PpuMode::OamScan => {
                if self.mode_cycles >= OAM_SCAN_CYCLES {
                    self.mode_cycles = 0;
                    self.lcd_status.set_mode(PpuMode::DrawingPixels);
                }
            }
            PpuMode::DrawingPixels => {
                if self.mode_cycles >= DRAWING_PIXELS_CYCLES {
                    self.mode_cycles = 0;
                    self.render_scanline();
                    if self.set_mode(PpuMode::HBlank) {
                        *self.interrupt_flag.borrow_mut() |= 0x02;
                    }
                }
            }
            PpuMode::HBlank => {
                if self.mode_cycles >= HBLANK_CYCLES {
                    self.mode_cycles = 0;
                    self.window.increment_line_counter(self.lcd_control.window_enabled(), self.ly);
                    if self.ly == VIEWPORT_HEIGHT as u8 - 1 {
                        self.frame_ready = true;
                        *self.interrupt_flag.borrow_mut() |= 0x01;
                        if self.set_mode(PpuMode::VBlank) {
                            *self.interrupt_flag.borrow_mut() |= 0x02;
                        }
                    } else {
                        if self.set_mode(PpuMode::OamScan) {
                            *self.interrupt_flag.borrow_mut() |= 0x02;
                        }
                    }
                    self.set_ly(self.ly + 1);
                }
            }
            PpuMode::VBlank => {
                if self.mode_cycles >= VBLANK_CYCLES {
                    self.mode_cycles = 0;
                    self.set_ly(self.ly + 1);
                    if self.ly == 0 {
                        self.window.reset_line_counter();
                        if self.set_mode(PpuMode::OamScan) {
                            *self.interrupt_flag.borrow_mut() |= 0x02;
                        }
                    }
                }
            }
        }
    }

    pub fn mode(&self) -> PpuMode {
        self.lcd_status.mode()
    }

    fn set_mode(&mut self, mode: PpuMode) -> bool {
        if self.lcd_status.mode() == mode {
            return false;
        }

        self.lcd_status.set_mode(mode);
        match self.lcd_status.mode() {
            PpuMode::HBlank => self.lcd_status.mode0_interrupt(),
            PpuMode::VBlank => self.lcd_status.mode1_interrupt(),
            PpuMode::OamScan => self.lcd_status.mode1_interrupt(),
            PpuMode::DrawingPixels => false,
        }
    }

    fn clear_screen(&mut self) {
        self.line_priority.fill((0, false));
        self.frame_buffer.fill((255, 255, 255));
        self.frame_ready = true;
    }

    pub fn ly(&self) -> u8 {
        self.ly
    }

    fn set_ly(&mut self, value: u8) {
        self.ly = value % NUMBER_OF_LINES;
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
            *self.interrupt_flag.borrow_mut() |= 0x02;
        }
    }

    fn set_lcd_control(&mut self, value: u8) {
        self.lcd_control = value.into();

        if !self.lcd_control.lcd_enabled() {
            self.clear_screen();
            self.window.reset_line_counter();
            self.set_ly(0);
            self.lcd_status.set_mode(PpuMode::HBlank);
            self.mode_cycles = 0;
        }
    }

    fn render_scanline(&mut self) {
        if self.lcd_control.bg_window_enabled() || self.gb_mode == GbMode::Color {
            self.render_bg_window_line();
        }

        if self.lcd_control.object_enabled() {
            self.render_object_line();
        }
    }

    fn render_bg_window_line(&mut self) {
        for lx in 0..VIEWPORT_WIDTH as u8 {
            let (tile_index_address, x_offset, y_offset) = self.bg_window_tile_data(lx);

            let tile_index = self.read_vram_bank_0(tile_index_address);
            let bg_map_attributes = if self.gb_mode == GbMode::Color {
                BgMapAttributes::from(self.read_vram_bank_1(tile_index_address))
            } else {
                BgMapAttributes::from(0)
            };

            let tile_address = self.lcd_control.tile_data_area().tile_address(tile_index);
            let (byte1, byte2) = match bg_map_attributes.y_flip() {
                false => self.get_tile_bytes(tile_address + y_offset as u16, bg_map_attributes.bank()),
                true => self.get_tile_bytes(tile_address + (14 - y_offset) as u16, bg_map_attributes.bank()),
            };

            let x_offset = match bg_map_attributes.x_flip() {
                false => x_offset,
                true => 7 - x_offset,
            };

            let color_index = color_index(byte1, byte2, x_offset);
            self.line_priority[lx as usize] = (color_index, bg_map_attributes.priority());

            let color = if self.gb_mode == GbMode::Color {
                self.cgb_bg_palette.pixel_color(bg_map_attributes.color_palette(), color_index)
            } else {
                self.bg_palette.pixel_color(color_index)
            };
            let offset = lx as usize + self.ly as usize * VIEWPORT_WIDTH;
            self.frame_buffer[offset] = color
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
        self.read_objects_from_oam();
        for (oam_index, x_offset) in self.oam_buffer.iter() {
            let oam_entry = self.oam.oam_entry(*oam_index);
            let y_offset = oam_entry.y_position().wrapping_sub(16);

            let mut tile_index = oam_entry.tile_index();
            if self.object_height == 2 * TILE_HEIGHT {
                tile_index &= 0xFE;
            }

            let tile_base_address = 0x8000 + (tile_index as u16 * 16);
            let line_offset = if oam_entry.attributes().y_flip() {
                self.object_height - 1 - (self.ly - y_offset)
            } else {
                self.ly - y_offset
            };
            let tile_address = tile_base_address + line_offset as u16 * 2;

            let bank = oam_entry.attributes().bank();
            let (byte1, byte2) = self.get_tile_bytes(tile_address, bank);
            let color_palette_index = oam_entry.attributes().cgb_palette();

            for pixel_index in 0..TILE_WIDTH {
                let lx = x_offset.wrapping_add(pixel_index);
                if !(0..VIEWPORT_WIDTH).contains(&(lx as usize)) {
                    continue;
                }

                let oam_pixel_index = if oam_entry.attributes().x_flip() { pixel_index } else { 7 - pixel_index };
                let color_index = color_index(byte1, byte2, oam_pixel_index);
                if color_index == 0 {
                    continue;
                }

                let offset = lx as usize + self.ly as usize * VIEWPORT_WIDTH;
                if self.gb_mode == GbMode::Color {
                    if self.line_priority[lx as usize].0 != 0
                        && self.lcd_control.bg_window_enabled()
                        && (self.line_priority[lx as usize].1 || oam_entry.attributes().priority())
                    {
                        continue;
                    }

                    let color = self.cgb_obj_palette.pixel_color(color_palette_index, color_index);
                    self.frame_buffer[offset] = color;
                } else {
                    if oam_entry.attributes().priority() && self.line_priority[lx as usize].0 != 0 {
                        continue;
                    }

                    let object_pallete = if oam_entry.attributes().dmg_palette() {
                        self.obj1_palette
                    } else {
                        self.obj0_palette
                    };
                    let color = object_pallete.pixel_color(color_index);
                    self.frame_buffer[offset] = color;
                }
            }
        }
    }

    fn read_objects_from_oam(&mut self) {
        self.oam_buffer.clear();
        self.object_height = if self.lcd_control.object_size() { 2 * TILE_HEIGHT } else { TILE_HEIGHT };

        for i in 0..OAM_SIZE {
            let oam_entry = self.oam.oam_entry(i);
            let object_y = oam_entry.y_position().wrapping_sub(16);
            let object_x = oam_entry.x_position().wrapping_sub(8);
            if self.ly >= object_y && self.ly < object_y.wrapping_add(self.object_height) {
                self.oam_buffer.push((i, object_x));
            }
        }

        if self.gb_mode == GbMode::Color {
            self.oam_buffer.sort_by(|a, b| a.0.cmp(&b.0));
        } else {
            self.oam_buffer.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
        }
        self.oam_buffer.truncate(10);
        self.oam_buffer.reverse();
    }

    fn get_tile_bytes(&self, address: u16, bank: bool) -> (u8, u8) {
        match bank {
            false => (self.read_vram_bank_0(address), self.read_vram_bank_0(address + 1)),
            true => (self.read_vram_bank_1(address), self.read_vram_bank_1(address + 1)),
        }
    }

    fn read_vram_bank_0(&self, address: u16) -> u8 {
        self.vram[address as usize - 0x8000]
    }

    fn read_vram_bank_1(&self, address: u16) -> u8 {
        self.vram[0x2000 + address as usize - 0x8000]
    }
}
