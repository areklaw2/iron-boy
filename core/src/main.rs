use std::{env, time::Duration};

use cpu::{
    bus::{Bus, Memory},
    cartridge::Cartridge,
    cpu::{registers::Registers, Cpu},
    io::ppu::{SCREEN_HEIGHT, SCREEN_WIDTH},
};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window};
use utils::Mode;

const SCALE: u32 = 6;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Invalid input");
        return;
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Iron Boy", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let cartridge = Cartridge::load(&args[1]);
    println!("Cartridge loaded..");

    // refactor this
    let registers = Registers::new(Mode::Monochrome);
    let bus = Bus::new(cartridge);
    let mut cpu = Cpu::new(bus, registers);

    'cycle: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'cycle,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        cpu.cycle();

        draw_screen(&cpu, &mut canvas)
    }
}

fn draw_screen(cpu: &Cpu, canvas: &mut Canvas<Window>) {
    canvas.clear();

    let mut x_draw = 0;
    let mut y_draw = 0;
    let mut tile_index = 0;
    let address = 0x8000;

    for y in 0..24 {
        for x in 0..16 {
            display_tile(cpu, canvas, address, tile_index, x_draw + (x * SCALE), y_draw + (y * SCALE));
            x_draw += 8 * SCALE;
            tile_index += 1;
        }
        y_draw += 8 * SCALE;
        x_draw = 0
    }

    //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
}

fn display_tile(cpu: &Cpu, canvas: &mut Canvas<Window>, address: u16, tile_index: u16, x: u32, y: u32) {
    let mut tile_y = 0;
    while tile_y < 16 {
        let byte1 = cpu.bus.mem_read(address + (tile_index * 16) + tile_y);
        let byte2 = cpu.bus.mem_read(address + (tile_index * 16) + tile_y + 1);
        for bit in 7..=0 {
            let hi = !!(byte1 & (1 << bit)) << 1;
            let lo = !!(byte2 & (1 << bit));
            let color = hi | lo;
            canvas.set_draw_color(get_color(color));

            let rect = Rect::new((x + (7 - bit) * SCALE) as i32, (y + (tile_y as u32 / 2 * SCALE)) as i32, SCALE, SCALE);
            canvas.fill_rect(rect).unwrap();
            canvas.present();
            canvas.clear();
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
