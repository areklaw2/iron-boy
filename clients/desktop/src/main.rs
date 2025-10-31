use core::{FPS, GameBoy, JoypadButton, SAMPLES_PER_FRAME};
use desktop::Desktop;
use sdl2::{
    event::{Event, WindowEvent},
    image::{self, InitFlag, LoadTexture},
    keyboard::Keycode,
    render,
};
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
                .without_time() // remove this
                .with_target(false) // remove this
                .with_level(false) // remove this
                .with_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"))),
        )
        .init();

    let rom_path = env::args().nth(1).expect("Please provide a file path as an argument");
    let desktop = Desktop::new(rom_path).unwrap();
    let sdl_context = desktop.sdl_context;
    let mut game_boy = desktop.game_boy;

    let mut audio_device = audio::create_audio_device(&sdl_context);
    audio_device.resume();

    let _image_context = image::init(InitFlag::PNG).unwrap();

    let mut canvas = video::create_canvas(&sdl_context);
    let main_window_id = canvas.window().id();

    let mut canvas2 = video::create_canvas(&sdl_context);
    let test_window_id = canvas2.window().id();
    let texture_creator = canvas2.texture_creator();
    let texture = texture_creator.load_texture("media/ironboy_logo.png").unwrap();
    video::render_splash(&mut canvas2, &texture);

    let mut canvas2 = Some(canvas2);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let frame_clock = std::time::Instant::now();

    'game: loop {
        let audio_lock = audio_device.lock();
        let sample_count = audio_lock.sample_count();
        drop(audio_lock);

        if sample_count < SAMPLES_PER_FRAME {
            let (left_samples, right_samples) = game_boy.run_until_audio_buffer_full();
            let mut audio_lock = audio_device.lock();
            audio_lock.queue_samples(left_samples, right_samples);
            drop(audio_lock)
        }

        let time_elapsed = frame_clock.elapsed();
        if time_elapsed < FRAME_DURATION {
            std::thread::sleep(FRAME_DURATION - time_elapsed);
        } else {
            video::render_screen(&mut canvas, game_boy.current_frame());
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Window {
                    win_event: WindowEvent::Close,
                    window_id,
                    ..
                } => {
                    println!("{}", window_id);
                    if window_id == main_window_id {
                        break 'game;
                    }
                    if window_id == test_window_id {
                        canvas2 = None
                    }
                }
                Event::KeyDown {
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
