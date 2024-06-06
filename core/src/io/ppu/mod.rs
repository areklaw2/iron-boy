use palette::Palette;
use registers::{LcdControl, LcdStatus};

use crate::bus::Memory;

pub mod object;
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
        todo!("{}", ticks)
    }
}
