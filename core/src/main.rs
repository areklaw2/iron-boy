use std::{env, thread, time::Duration};

use cpu::{
    bus::Bus,
    cartridge::Cartridge,
    cpu::{registers::Registers, Cpu},
    video,
};
use sdl2::{event::Event, keyboard::Keycode};
use utils::Mode;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Invalid input");
        return;
    }

    let cartridge = Cartridge::load(&args[1]);
    println!("Cartridge loaded..");
    let registers = Registers::new(Mode::Monochrome);
    let bus = Bus::new(cartridge);
    let mut cpu = Cpu::new(bus, registers);

    let sdl_context = sdl2::init().expect("failed to init");
    let mut renderer = video::init(&sdl_context).unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
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
        //need to put cpu on its on thread and loop
        // renderer is taking time
        //renderer.render(&cpu);
    }
}
