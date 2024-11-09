use desktop::{audio::Audio, GameBoy};
use ironboy_core::{
    apu::{AUDIO_BUFFER_THRESHOLD, SAMPLING_FREQUENCY, SAMPLING_RATE},
    cpu::CPU_CLOCK_SPEED,
    JoypadButton, FPS, VIEWPORT_HEIGHT, VIEWPORT_WIDTH,
};
use sdl2::{
    audio::{AudioDevice, AudioSpecDesired},
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::Canvas,
    video::Window,
    AudioSubsystem,
};
use std::{
    collections::VecDeque,
    env,
    fs::File,
    io::Read,
    sync::{Arc, Mutex},
};

const SCALE: u32 = 4;
const WINDOW_WIDTH: u32 = (VIEWPORT_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (VIEWPORT_HEIGHT as u32) * SCALE;
const FRAME_DURATION_MS: f32 = 1_000.0 / FPS;
const FRAME_DURATION_MICROS: f32 = FRAME_DURATION_MS * 1000.0;
const FRAME_DURATION: std::time::Duration = std::time::Duration::from_micros(FRAME_DURATION_MICROS as u64);

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
        .resizable()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().accelerated().build().unwrap();
    let mut audio_subsystem = sdl_context.audio().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut rom = File::open(&args[1]).expect("Unable to open file");
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer).expect("Issue while reading file");
    let mut game_boy = GameBoy::new_dmg(&args[1], buffer, false);

    let volume = game_boy.volume;
    let audio_device = create_audio_device(&mut game_boy, &mut audio_subsystem, &volume);
    audio_device.resume();

    'game: loop {
        let frame_start_time = std::time::Instant::now();
        let cycles_per_frame = CPU_CLOCK_SPEED as f32 / FPS;
        let mut cycles_passed = 0.0;
        while cycles_passed <= cycles_per_frame {
            let ticks = game_boy.cycle();
            if game_boy.update_ppu() {
                let data = game_boy.ppu_buffer().to_vec();
                recalculate_screen(&mut canvas, &data)
            }
            cycles_passed += (ticks) as f32;
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

fn recalculate_screen(canvas: &mut Canvas<Window>, data: &[(u8, u8, u8)]) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    for x in 0..VIEWPORT_WIDTH {
        for y in 0..VIEWPORT_HEIGHT {
            let i = y * VIEWPORT_WIDTH + x;
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

fn create_audio_device<'a, 'b: 'a>(game_boy: &'a mut GameBoy, audio_subsystem: &'a mut AudioSubsystem, volume: &'b u8) -> AudioDevice<Audio<'a>> {
    let audio_spec_desired = AudioSpecDesired {
        freq: Some(SAMPLING_FREQUENCY as i32),
        samples: Some(SAMPLING_RATE),
        channels: Some(2),
    };

    let left_volume = &game_boy.cpu.bus.apu.left_volume;
    let right_volume = &game_boy.cpu.bus.apu.right_volume;
    let audio = Audio::new(&mut game_boy.cpu.bus.apu.audio_buffer, left_volume, right_volume, volume);

    audio_subsystem.open_playback(None, &audio_spec_desired, |_spec| audio).unwrap()
}
