use ironboy_core::{gb::GameBoy, JoypadButton, AUDIO_BUFFER_THRESHOLD, FPS};
use sdl2::{event::Event, keyboard::Keycode};
use std::{
    collections::VecDeque,
    env,
    fs::File,
    io::Read,
    sync::{Arc, Mutex},
};

pub mod audio;
pub mod video;

const FRAME_DURATION_NANOS: f32 = 1_000_000_000.0 / FPS;
const FRAME_DURATION: std::time::Duration = std::time::Duration::from_nanos(FRAME_DURATION_NANOS as u64);

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Please provide a file path as an argument");
    }

    let mut rom = File::open(&args[1]).expect("Unable to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).expect("Issue while reading file");

    let mut game_boy = GameBoy::new(&args[1], buffer, true);
    let sdl_context = sdl2::init().unwrap();
    let audio_device = audio::create_audio_device(&mut game_boy, &sdl_context);
    audio_device.resume();
    let mut canvas = video::create_canvas(&sdl_context);
    let mut event_pump = sdl_context.event_pump().unwrap();

    'game: loop {
        let frame_start_time = std::time::Instant::now();
        let frames = game_boy.run();
        for frame in frames {
            video::render_screen(&mut canvas, &frame);
        }
        while should_sync(frame_start_time, &game_boy.cpu.bus.apu.audio_buffer) {
            std::hint::spin_loop();
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

fn should_sync(frame_start_time: std::time::Instant, audio_buffer: &Arc<Mutex<VecDeque<u8>>>) -> bool {
    frame_start_time.elapsed().as_micros() < FRAME_DURATION.as_micros() && audio_buffer.lock().unwrap().len() > AUDIO_BUFFER_THRESHOLD
}
