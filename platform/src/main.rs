use ironboy_core::{
    apu::{AUDIO_BUFFER_THRESHOLD, SAMPLING_FREQUENCY, SAMPLING_RATE},
    cpu::CPU_CLOCK_SPEED,
    game_boy::GameBoy,
    JoypadButton, VIEWPORT_HEIGHT, VIEWPORT_WIDTH,
};
use platform::audio::Audio;
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
use std::env;

const SCALE: u32 = 4;
const WINDOW_WIDTH: u32 = (VIEWPORT_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (VIEWPORT_HEIGHT as u32) * SCALE;
const FPS: f32 = 59.7275;

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
    //let mut audio_subsystem = sdl_context.audio().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut game_boy = GameBoy::new_dmg(&args[1], false);

    let frame_duration = std::time::Duration::from_millis((1_000.0 / FPS) as u64);
    let volume = game_boy.volume;
    //let audio_device = create_audio_device(&mut game_boy, &mut audio_subsystem, &volume);
    //audio_device.resume();

    'game: loop {
        let frame_start_time = std::time::Instant::now();
        let cycles_per_frame = CPU_CLOCK_SPEED as f32 / FPS;
        let mut cycles_passed = 0.0;
        while cycles_passed <= cycles_per_frame {
            let ticks = game_boy.cycle();
            if game_boy.get_ppu_update() {
                let data = game_boy.get_ppu_data().to_vec();
                recalculate_screen(&mut canvas, &data)
            }
            cycles_passed += (ticks) as f32;
        }

        while frame_start_time.elapsed() < frame_duration {
            std::hint::spin_loop();
        }

        // while game_boy.cpu.bus.apu.audio_buffer.len() > AUDIO_BUFFER_THRESHOLD {
        //     std::hint::spin_loop();
        // }

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
    let device = AudioSpecDesired {
        freq: Some(SAMPLING_FREQUENCY as i32),
        samples: Some(SAMPLING_RATE),
        channels: Some(2),
    };

    let left_volume = &game_boy.cpu.bus.apu.left_volume;
    let right_volume = &game_boy.cpu.bus.apu.right_volume;
    let audio = Audio::new(&mut game_boy.cpu.bus.apu.audio_buffer, left_volume, right_volume, volume);

    audio_subsystem.open_playback(None, &device, |_spec| audio).unwrap()
}
