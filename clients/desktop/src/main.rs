use core::{FPS, GameBoy, JoypadButton, VIEWPORT_HEIGHT, VIEWPORT_WIDTH, read_rom};
use std::env;
use std::sync::Arc;
use std::time::Instant;

use pixels::{Pixels, SurfaceTexture};
use winit::application::ApplicationHandler;
use winit::dpi::LogicalSize;
use winit::event::{DeviceEvent, DeviceId, StartCause, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::KeyCode;
use winit::window::{Window, WindowId};
use winit_input_helper::WinitInputHelper;

const SCALE: u32 = 6;
const WINDOW_WIDTH: u32 = (VIEWPORT_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (VIEWPORT_HEIGHT as u32) * SCALE;

const FRAME_DURATION_NANOS: f32 = 1_000_000_000.0 / FPS;
const FRAME_DURATION: std::time::Duration = std::time::Duration::from_nanos(FRAME_DURATION_NANOS as u64);

struct App {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
    input: WinitInputHelper,
    game_boy: GameBoy,
    last_frame_time: Instant,
}

impl App {
    fn draw(&mut self) {
        let frame = self.pixels.as_mut().unwrap().frame_mut();
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            // Convert window pixel coordinates to Game Boy pixel coordinates
            let window_x = i % WINDOW_WIDTH as usize;
            let window_y = i / WINDOW_WIDTH as usize;
            let gb_x = window_x / SCALE as usize;
            let gb_y = window_y / SCALE as usize;
            let gb_index = gb_y * VIEWPORT_WIDTH + gb_x;

            let (r, g, b) = self.game_boy.current_frame()[gb_index];
            pixel.copy_from_slice(&[r, g, b, 0xFF]);
        }
    }
}

impl ApplicationHandler for App {
    fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        // Just process the event, don't draw here
        self.input.process_window_event(&event);

        // Only draw when specifically requested
        if let WindowEvent::RedrawRequested = event {
            self.draw();
            if let Err(err) = self.pixels.as_mut().unwrap().render() {
                eprintln!("{}", err);
                return;
            }
        }
    }

    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        // pass in events
        self.input.process_device_event(&event);
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        self.input.end_step();

        if self.input.key_released(KeyCode::Escape) || self.input.close_requested() || self.input.destroyed() {
            event_loop.exit();
            return;
        }

        // Handle key presses
        if self.input.key_pressed(KeyCode::KeyX) {
            self.game_boy.button_down(JoypadButton::A);
        }
        if self.input.key_pressed(KeyCode::KeyZ) {
            self.game_boy.button_down(JoypadButton::B);
        }
        if self.input.key_pressed(KeyCode::Enter) {
            self.game_boy.button_down(JoypadButton::Select);
        }
        if self.input.key_pressed(KeyCode::Space) {
            self.game_boy.button_down(JoypadButton::Start);
        }
        if self.input.key_pressed(KeyCode::ArrowUp) {
            self.game_boy.button_down(JoypadButton::Up);
        }
        if self.input.key_pressed(KeyCode::ArrowLeft) {
            self.game_boy.button_down(JoypadButton::Left);
        }
        if self.input.key_pressed(KeyCode::ArrowDown) {
            self.game_boy.button_down(JoypadButton::Down);
        }
        if self.input.key_pressed(KeyCode::ArrowRight) {
            self.game_boy.button_down(JoypadButton::Right);
        }

        // Handle key releases
        if self.input.key_released(KeyCode::KeyX) {
            self.game_boy.button_up(JoypadButton::A);
        }
        if self.input.key_released(KeyCode::KeyZ) {
            self.game_boy.button_up(JoypadButton::B);
        }
        if self.input.key_released(KeyCode::Enter) {
            self.game_boy.button_up(JoypadButton::Select);
        }
        if self.input.key_released(KeyCode::Space) {
            self.game_boy.button_up(JoypadButton::Start);
        }
        if self.input.key_released(KeyCode::ArrowUp) {
            self.game_boy.button_up(JoypadButton::Up);
        }
        if self.input.key_released(KeyCode::ArrowLeft) {
            self.game_boy.button_up(JoypadButton::Left);
        }
        if self.input.key_released(KeyCode::ArrowDown) {
            self.game_boy.button_up(JoypadButton::Down);
        }
        if self.input.key_released(KeyCode::ArrowRight) {
            self.game_boy.button_up(JoypadButton::Right);
        }

        if let Some(size) = self.input.window_resized() {
            if let Err(err) = self.pixels.as_mut().unwrap().resize_surface(size.width, size.height) {
                println!("pixels.resize_surface: {}", err);
                return;
            }
        }

        let time_elapsed = self.last_frame_time.elapsed();
        if time_elapsed >= FRAME_DURATION {
            // Enough time has passed, update the game and request redraw
            self.game_boy.run_until_frame();
            self.last_frame_time = Instant::now();
            self.window.as_ref().unwrap().request_redraw();
        }
    }

    fn new_events(&mut self, _: &ActiveEventLoop, _: StartCause) {
        self.input.step();
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let size = LogicalSize::new(WINDOW_WIDTH as f64, WINDOW_HEIGHT as f64);
            let window = Arc::new(
                event_loop
                    .create_window(
                        Window::default_attributes()
                            .with_title("Hello Pixels")
                            .with_inner_size(size)
                            .with_min_inner_size(size),
                    )
                    .unwrap(),
            );

            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window.clone());
            self.pixels = Some(Pixels::new(WINDOW_WIDTH, WINDOW_HEIGHT, surface_texture).unwrap());
            self.window = Some(window);
        }
    }
}

fn main() {
    let rom_path = env::args().nth(1).expect("Please provide a file path as an argument");
    let rom_name = rom_path.split("/").last().expect("Please provide a file path as an argument");
    let buffer = read_rom(&rom_path);
    let game_boy = GameBoy::new(rom_name, buffer);

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop
        .run_app(&mut App {
            input: WinitInputHelper::new(),
            window: None,
            pixels: None,
            game_boy,
            last_frame_time: Instant::now(),
        })
        .unwrap();
}
