use std::time::Duration;

use sdl2::{
    pixels::Color,
    rect::Rect,
    render::{Canvas, WindowCanvas},
    video::Window,
    Sdl, VideoSubsystem,
};

use crate::{
    bus::Memory,
    cpu::Cpu,
    io::ppu::{SCREEN_HEIGHT, SCREEN_WIDTH},
};

const SCALE: u32 = 4;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

pub struct Renderer {
    canvas: WindowCanvas,
    #[allow(unused)]
    video_subsystem: VideoSubsystem, // holds a reference to the video subsystem
}

pub fn init<'a>(sdl_context: &'a Sdl) -> Result<Renderer, Box<dyn std::error::Error>> {
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Iron Boy", 16 * 8 * (SCALE) + 16 * SCALE, 24 * 8 * (SCALE) + 24 * SCALE)
        .position_centered()
        .resizable()
        .opengl()
        .build()?;

    let canvas = window.into_canvas().present_vsync().accelerated().build()?;

    Ok(Renderer { canvas, video_subsystem })
}

impl Renderer {
    pub fn set_window_title(&mut self, title: &str) {
        self.canvas.window_mut().set_title(&title).unwrap();
    }

    pub fn render(&mut self, cpu: &Cpu) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        let mut x_draw = 0;
        let mut y_draw = 0;
        let mut tile_index = 0;
        let address = 0x8000;

        for y in 0..24 {
            for x in 0..16 {
                display_tile(cpu, &mut self.canvas, address, tile_index, x_draw + (x * SCALE), y_draw + (y * SCALE));
                x_draw += 8 * SCALE;
                tile_index += 1;
            }
            y_draw += 8 * SCALE;
            x_draw = 0
        }

        self.canvas.present();
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn display_tile(cpu: &Cpu, canvas: &mut Canvas<Window>, address: u16, tile_index: u16, x: u32, y: u32) {
    let mut tile_y = 0;
    while tile_y < 16 {
        let byte1 = cpu.bus.mem_read(address + (tile_index * 16) + tile_y);
        let byte2 = cpu.bus.mem_read(address + (tile_index * 16) + tile_y + 1);
        for bit in (0..=7).rev() {
            let hi = !!(byte1 & (1 << bit)) << 1;
            let lo = !!(byte2 & (1 << bit));
            let color = hi | lo;
            canvas.set_draw_color(get_color(color));

            let rect = Rect::new(
                (x + (7 - bit) * SCALE) as i32,
                (y + ((tile_y / 2) as u32 * SCALE)) as i32,
                SCALE + 2,
                SCALE + 2,
            );
            canvas.fill_rect(rect).unwrap();
        }
        tile_y += 2
    }
}

fn get_color(color: u8) -> Color {
    match color {
        0 => Color::RGB(0xFF, 0xFF, 0xFF),
        1 => Color::RGB(0xAA, 0xAA, 0xAA),
        2 => Color::RGB(0x55, 0x55, 0x55),
        _ => Color::RGB(0x00, 0x00, 0x00),
    }
}
