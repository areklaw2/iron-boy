use ironboy_core::{apu::Apu, gb::GameBoy, JoypadButton, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::{
    env,
    sync::mpsc::{self},
    thread, time,
};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window};

const SCALE: u32 = 4;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;
const GRANULARITY: i64 = 65536 * 4;
const SYSTEM_CLOCK_FREQUENCY: i64 = 4194304;
const AUDIO_ADJUST_SEC: i64 = 1;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Invalid input");
        return;
    }

    let mut cpu = build_game_boy(&args[1], true, false);

    let sdl_context = sdl2::init().expect("failed to init");
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Iron Boy", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().accelerated().build().unwrap();

    let batch_duration_ns = GRANULARITY * (1_000_000_000 / SYSTEM_CLOCK_FREQUENCY);
    let batch_duration_ms = (batch_duration_ns / 1_000_000) as u64;
    let (tick_tx, tick_rx) = mpsc::channel();

    thread::spawn(move || loop {
        thread::sleep(time::Duration::from_millis(batch_duration_ms));
        if tick_tx.send(()).is_err() {
            return;
        }
    });

    let mut cycles = 0;
    let mut audio_sync_count = 0;

    'game: loop {
        while cycles < GRANULARITY {
            cycles += cpu.cycle() as i64;
            if cpu.get_ppu_update() {
                let data = cpu.get_ppu_data().to_vec();
                recalculate_screen(&mut canvas, &data)
            }
        }

        cycles -= GRANULARITY;

        let mut event_pump = sdl_context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,
                Event::KeyDown {
                    keycode: Some(Keycode::A), ..
                } => cpu.button_down(JoypadButton::Select),
                Event::KeyUp {
                    keycode: Some(Keycode::A), ..
                } => cpu.button_up(JoypadButton::Select),
                Event::KeyDown {
                    keycode: Some(Keycode::S), ..
                } => cpu.button_down(JoypadButton::Start),
                Event::KeyUp {
                    keycode: Some(Keycode::S), ..
                } => cpu.button_up(JoypadButton::Start),
                Event::KeyDown {
                    keycode: Some(Keycode::Z), ..
                } => cpu.button_down(JoypadButton::B),
                Event::KeyUp {
                    keycode: Some(Keycode::Z), ..
                } => cpu.button_up(JoypadButton::B),
                Event::KeyDown {
                    keycode: Some(Keycode::X), ..
                } => cpu.button_down(JoypadButton::A),
                Event::KeyUp {
                    keycode: Some(Keycode::X), ..
                } => cpu.button_up(JoypadButton::A),
                Event::KeyDown {
                    keycode: Some(Keycode::Up), ..
                } => cpu.button_down(JoypadButton::Up),
                Event::KeyUp {
                    keycode: Some(Keycode::Up), ..
                } => cpu.button_up(JoypadButton::Up),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => cpu.button_down(JoypadButton::Down),
                Event::KeyUp {
                    keycode: Some(Keycode::Down),
                    ..
                } => cpu.button_up(JoypadButton::Down),
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => cpu.button_down(JoypadButton::Left),
                Event::KeyUp {
                    keycode: Some(Keycode::Left),
                    ..
                } => cpu.button_up(JoypadButton::Left),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => cpu.button_down(JoypadButton::Right),
                Event::KeyUp {
                    keycode: Some(Keycode::Right),
                    ..
                } => cpu.button_up(JoypadButton::Right),
                _ => {}
            }
        }

        if let Err(e) = tick_rx.recv() {
            panic!("Timer died: {:?}", e)
        }

        audio_sync_count += GRANULARITY;
        if audio_sync_count >= SYSTEM_CLOCK_FREQUENCY * AUDIO_ADJUST_SEC {
            audio_sync_count = 0;
        }
    }
}

fn build_game_boy(filename: &str, dmg: bool, skip_boot: bool) -> Box<GameBoy> {
    let game_boy = match dmg {
        true => GameBoy::new_dmg(filename, skip_boot),
        false => GameBoy::new_cgb(filename),
    };
    Box::new(game_boy)
}

fn recalculate_screen(canvas: &mut Canvas<Window>, data: &[(u8, u8, u8)]) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for x in 0..SCREEN_WIDTH {
        for y in 0..SCREEN_HEIGHT {
            let i = y * SCREEN_WIDTH + x;
            let color = data[i as usize];
            canvas.set_draw_color(Color::RGB(color.0, color.1, color.2));
            let rect = Rect::new(
                (x as u32 * SCALE) as i32,
                (y as u32 * SCALE) as i32,
                SCALE + 4, // change this if you want line speration
                SCALE + 4, // change this if you want line speration
            );
            canvas.fill_rect(rect).unwrap();
        }
    }

    canvas.present();
}
