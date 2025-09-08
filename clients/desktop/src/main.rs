use core::{FPS, GameBoy, JoypadButton};
use sdl2::{event::Event, keyboard::Keycode};
use std::{
    env,
    fs::{File, OpenOptions},
    io::Read,
};
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub mod audio;
pub mod video;

const FRAME_DURATION_NANOS: f32 = 1_000_000_000.0 / FPS;
const FRAME_DURATION: std::time::Duration = std::time::Duration::from_nanos(FRAME_DURATION_NANOS as u64);

fn main() {
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("ironboy.log")
        .expect("Failed to create log file");

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_writer(log_file)
                .with_ansi(false)
                .with_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"))),
        )
        .init();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please provide a file path as an argument");
    }

    let mut rom = File::open(&args[1]).expect("Unable to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).expect("Issue while reading file");

    let mut game_boy = GameBoy::new(&args[1], buffer);
    let sdl_context = sdl2::init().unwrap();
    let audio_device = audio::create_audio_device(&mut game_boy, &sdl_context);
    audio_device.resume();
    let mut canvas = video::create_canvas(&sdl_context);
    let mut event_pump = sdl_context.event_pump().unwrap();

    'game: loop {
        let frame_clock = std::time::Instant::now();
        if game_boy.run_until_frame() {
            let frame = game_boy.current_frame();
            video::render_screen(&mut canvas, &frame);

            let time_elapsed = frame_clock.elapsed();
            if time_elapsed < FRAME_DURATION {
                std::thread::sleep(FRAME_DURATION - time_elapsed);
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::X) => game_boy.button_down(JoypadButton::A),
                    Some(Keycode::Z) => game_boy.button_down(JoypadButton::B),
                    Some(Keycode::Return) => game_boy.button_down(JoypadButton::Select),
                    Some(Keycode::Space) => game_boy.button_down(JoypadButton::Start),
                    Some(Keycode::Up) => game_boy.button_down(JoypadButton::Up),
                    Some(Keycode::Left) => game_boy.button_down(JoypadButton::Left),
                    Some(Keycode::Down) => game_boy.button_down(JoypadButton::Down),
                    Some(Keycode::Right) => game_boy.button_down(JoypadButton::Right),
                    Some(Keycode::Num1) => game_boy.decrease_volume(),
                    Some(Keycode::Num2) => game_boy.increase_volume(),
                    _ => {}
                },
                Event::KeyUp { keycode, .. } => match keycode {
                    Some(Keycode::X) => game_boy.button_up(JoypadButton::A),
                    Some(Keycode::Z) => game_boy.button_up(JoypadButton::B),
                    Some(Keycode::Return) => game_boy.button_up(JoypadButton::Select),
                    Some(Keycode::Space) => game_boy.button_up(JoypadButton::Start),
                    Some(Keycode::Up) => game_boy.button_up(JoypadButton::Up),
                    Some(Keycode::Left) => game_boy.button_up(JoypadButton::Left),
                    Some(Keycode::Down) => game_boy.button_up(JoypadButton::Down),
                    Some(Keycode::Right) => game_boy.button_up(JoypadButton::Right),
                    _ => {}
                },
                _ => {}
            };
        }
    }
}
