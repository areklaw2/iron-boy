use std::{env, time::Duration};

use cpu::{
    bus::Bus,
    cartridge::Cartridge,
    cpu::{registers::Registers, Cpu},
};
use sdl2::{event::Event, keyboard::Keycode, pixels::Color};
use utils::Mode;

const SCALE: u32 = 6;
const WINDOW_WIDTH: u32 = (160 as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (144 as u32) * SCALE;

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

    let mut i = 0;
    'cycle: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
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

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
