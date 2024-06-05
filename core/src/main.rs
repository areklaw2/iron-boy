use core::{game_boy::GameBoy, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::{
    env,
    sync::mpsc::{self, Receiver, SyncSender, TryRecvError, TrySendError},
    thread,
};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, video::Window};

const SCALE: u32 = 4;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

enum GameBoyEvent {
    ButtonUp(core::JoypadButton),
    ButtonDown(core::JoypadButton),
    SpeedUp,
    SpeedDown,
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Invalid input");
        return;
    }

    let cpu = build_game_boy(&args[1], true);
    let (sender1, receiver1) = mpsc::channel();
    let (sender2, receiver2) = mpsc::sync_channel(1);
    let (sender3, receiver3) = mpsc::sync_channel(1);

    let sdl_context = sdl2::init().expect("failed to init");
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Iron Boy", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position(2 * (WINDOW_WIDTH as i32) - 220, (WINDOW_HEIGHT as i32) / 2 + 120)
        .resizable()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().accelerated().build().unwrap();

    let tile_map_window = video_subsystem
        .window("Tile Map", 16 * 8 * (SCALE) + 16 * SCALE, 24 * 8 * (SCALE) + 24 * SCALE)
        .position((WINDOW_WIDTH as i32) - 200, (WINDOW_HEIGHT as i32) / 2)
        .resizable()
        .opengl()
        .build()
        .unwrap();

    let mut tile_map_canvas = tile_map_window.into_canvas().present_vsync().accelerated().build().unwrap();

    let cpu_thread = thread::spawn(move || run(cpu, sender2, sender3, receiver1));
    'eventloop: loop {
        let event_option = sdl_context.event_pump().unwrap().poll_event();
        match event_option {
            Some(event) => match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'eventloop,
                Event::KeyDown {
                    keycode: Some(Keycode::A), ..
                } => println!("A"),
                _ => {}
            },
            None => {}
        }

        match receiver2.recv() {
            Ok(data) => recalculate_screen(&mut canvas, &*data),
            Err(..) => {
                print!("crashed");
                break 'eventloop;
            } // Remote end has hung-up
        }

        match receiver3.recv() {
            Ok(data) => recalculate_tile_map_screen(&mut tile_map_canvas, &*data),
            Err(..) => {
                print!("crashed");
                break 'eventloop;
            } // Remote end has hung-up
        }
    }

    drop(receiver2); // Stop CPU thread by disconnecting
    drop(receiver3);
    let _ = cpu_thread.join();
}

fn build_game_boy(filename: &str, dmg: bool) -> Box<GameBoy> {
    let game_boy = match dmg {
        true => GameBoy::new_dmg(filename),
        false => GameBoy::new_cgb(filename),
    };
    Box::new(game_boy)
}

fn run(mut cpu: Box<GameBoy>, sender2: SyncSender<Vec<u8>>, sender3: SyncSender<Vec<u8>>, receiver: Receiver<GameBoyEvent>) {
    let periodic = timer_periodic(16);
    let mut limit_speed = true;

    let wait_ticks = (4194304f64 / 10.0 * 16.0).round() as u32;
    let mut ticks = 0;

    'outer: loop {
        while ticks < wait_ticks {
            ticks += cpu.cycle();
            if cpu.get_ppu_update() {
                let data = cpu.get_ppu_data().to_vec();
                if let Err(TrySendError::Disconnected(..)) = sender2.try_send(data) {
                    break 'outer;
                }
                let data = cpu.get_vram().to_vec();
                if let Err(TrySendError::Disconnected(..)) = sender3.try_send(data) {
                    break 'outer;
                }
            }
        }

        ticks -= wait_ticks;

        'recv: loop {
            match receiver.try_recv() {
                Ok(event) => match event {
                    GameBoyEvent::ButtonUp(button) => cpu.button_up(button),
                    GameBoyEvent::ButtonDown(button) => cpu.button_down(button),
                    GameBoyEvent::SpeedUp => limit_speed = false,
                    GameBoyEvent::SpeedDown => {
                        limit_speed = true;
                        cpu.sync_audio();
                    }
                },
                Err(TryRecvError::Empty) => break 'recv,
                Err(TryRecvError::Disconnected) => break 'outer,
            }
        }

        if limit_speed {
            let _ = periodic.recv();
        }
    }

    // let line = cpu.as_ref().lines().join("\n");
    // let mut output = File::create("iron_boy.csv").unwrap();
    // output.write_all(line.as_bytes()).unwrap();
}

fn timer_periodic(ms: u64) -> Receiver<()> {
    let (tx, rx) = std::sync::mpsc::sync_channel(1);
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_millis(ms));
        if tx.send(()).is_err() {
            break;
        }
    });
    rx
}

fn recalculate_screen(canvas: &mut Canvas<Window>, data: &[u8]) {
    canvas.set_draw_color(Color::RGB(255, 123, 0));
    canvas.clear();
    canvas.present();
}

fn recalculate_tile_map_screen(canvas: &mut Canvas<Window>, data: &[u8]) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let mut x_draw = 0;
    let mut y_draw = 0;
    let mut tile_index = 0;
    let address = 0;

    for y in 0..24 {
        for x in 0..16 {
            display_tile(canvas, data, address, tile_index, x_draw + (x * SCALE), y_draw + (y * SCALE));
            x_draw += 8 * SCALE;
            tile_index += 1;
        }
        y_draw += 8 * SCALE;
        x_draw = 0
    }

    canvas.present();
}

fn display_tile(canvas: &mut Canvas<Window>, data: &[u8], address: u16, tile_index: u16, x: u32, y: u32) {
    let mut tile_y = 0;
    while tile_y < 16 {
        let byte1 = data[(address + (tile_index * 16) + tile_y) as usize];
        let byte2 = data[(address + (tile_index * 16) + tile_y + 1) as usize];
        for bit in (0..=7).rev() {
            let hi = !!(byte1 & (1 << bit)) << 1;
            let lo = !!(byte2 & (1 << bit));
            let color = hi | lo;
            canvas.set_draw_color(get_color(color));

            let rect = Rect::new(
                (x + (7 - bit) * SCALE) as i32,
                (y + ((tile_y / 2) as u32 * SCALE)) as i32,
                SCALE + 4, // change this if you want line speration
                SCALE + 4, // change this if you want line speration
            );
            canvas.fill_rect(rect).unwrap();
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
