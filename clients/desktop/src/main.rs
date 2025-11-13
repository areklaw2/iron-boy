use desktop::Desktop;
use ironboy_core::{FPS, JoypadButton};
use sdl2::{
    event::{Event, WindowEvent},
    image::LoadTexture,
    keyboard::Keycode,
};
use std::env;

fn main() {
    let rom_path = env::args().nth(1);
    let mut desktop = Desktop::new(rom_path).unwrap();

    let main_window_id = desktop.window_manager().main_canvas().window().id();
    let texture_creator = desktop.window_manager_mut().main_canvas_mut().texture_creator();
    let texture = texture_creator.load_texture("media/ironboy_logo.png").unwrap();
    desktop.window_manager_mut().render_splash(&texture);

    //let mut canvas2 = video::create_canvas(&desktop.sdl_context);
    //let test_window_id = canvas2.window().id();
    //let mut canvas2 = Some(canvas2);

    let mut event_pump = desktop.sdl_context.event_pump().unwrap();
    let mut frame_clock = std::time::Instant::now();
    let mut fps_timer = std::time::Instant::now();
    let mut frame_count = 0;

    'game: loop {
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
                    // if window_id == test_window_id {
                    //     canvas2 = None
                    // }
                }
                Event::DropFile { window_id, filename, .. } => {
                    if desktop.game_boy_mut().is_none() && window_id == main_window_id {
                        desktop.load_rom(filename).unwrap();
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game,
                Event::KeyDown { keycode, .. } => {
                    if let Some(game_boy) = desktop.game_boy_mut() {
                        match keycode {
                            Some(Keycode::X) => game_boy.button_down(JoypadButton::A),
                            Some(Keycode::Z) => game_boy.button_down(JoypadButton::B),
                            Some(Keycode::Return) => game_boy.button_down(JoypadButton::Select),
                            Some(Keycode::Space) => game_boy.button_down(JoypadButton::Start),
                            Some(Keycode::Up) => game_boy.button_down(JoypadButton::Up),
                            Some(Keycode::Left) => game_boy.button_down(JoypadButton::Left),
                            Some(Keycode::Down) => game_boy.button_down(JoypadButton::Down),
                            Some(Keycode::Right) => game_boy.button_down(JoypadButton::Right),
                            _ => {}
                        }
                    }
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(game_boy) = desktop.game_boy_mut() {
                        match keycode {
                            Some(Keycode::X) => game_boy.button_up(JoypadButton::A),
                            Some(Keycode::Z) => game_boy.button_up(JoypadButton::B),
                            Some(Keycode::Return) => game_boy.button_up(JoypadButton::Select),
                            Some(Keycode::Space) => game_boy.button_up(JoypadButton::Start),
                            Some(Keycode::Up) => game_boy.button_up(JoypadButton::Up),
                            Some(Keycode::Left) => game_boy.button_up(JoypadButton::Left),
                            Some(Keycode::Down) => game_boy.button_up(JoypadButton::Down),
                            Some(Keycode::Right) => game_boy.button_up(JoypadButton::Right),
                            _ => {}
                        }
                    }
                }
                _ => {}
            };
        }

        desktop.run(&mut frame_clock, &mut fps_timer, &mut frame_count);
    }
}
