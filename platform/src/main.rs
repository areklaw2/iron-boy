use ironboy_core::{
    audio_player::{AudioPlayer, CpalPlayer},
    gb::GameBoy,
    SCREEN_HEIGHT, SCREEN_WIDTH,
};
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
    ButtonUp(ironboy_core::JoypadButton),
    ButtonDown(ironboy_core::JoypadButton),
    SpeedUp,
    SpeedDown,
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Invalid input");
        return;
    }

    let mut cpu = build_game_boy(&args[1], true);
    let mut cpal_audio_stream = None;

    let player = CpalPlayer::get();
    match player {
        Some((player, stream)) => {
            cpu.enable_audio(Box::new(player) as Box<dyn AudioPlayer>);
            cpal_audio_stream = Some(stream);
        }
        None => {
            println!("Could not open audio device");
        }
    }

    let (sender1, receiver1) = mpsc::channel();
    let (sender2, receiver2) = mpsc::sync_channel(1);

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

    let cpu_thread = thread::spawn(move || run(cpu, sender2, receiver1));
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
                    keycode: Some(Keycode::LShift),
                    ..
                } => {
                    let _ = sender1.send(GameBoyEvent::SpeedDown);
                }
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
    }

    drop(cpal_audio_stream);
    drop(receiver2); // Stop CPU thread by disconnecting
    let _ = cpu_thread.join();
}

fn build_game_boy(filename: &str, dmg: bool) -> Box<GameBoy> {
    let game_boy = match dmg {
        true => GameBoy::new_dmg(filename),
        false => GameBoy::new_cgb(filename),
    };
    Box::new(game_boy)
}

fn run(mut cpu: Box<GameBoy>, sender: SyncSender<Vec<(u8, u8, u8)>>, receiver: Receiver<GameBoyEvent>) {
    let periodic = timer_periodic(16);
    let mut limit_speed = true;

    let wait_ticks = (4194304f64 / 10.0 * 16.0).round() as u32;
    let mut ticks = 0;

    'outer: loop {
        while ticks < wait_ticks {
            ticks += cpu.cycle();
            if cpu.get_ppu_update() {
                let data = cpu.get_ppu_data().to_vec();
                if let Err(TrySendError::Disconnected(..)) = sender.try_send(data) {
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
