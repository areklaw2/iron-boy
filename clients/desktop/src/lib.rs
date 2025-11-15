use ironboy_core::{GameBoy, JoypadButton, SAMPLES_PER_FRAME, gb::GameBoyError};
use std::{fs::File, io::Read};

use sdl2::{
    EventPump,
    audio::AudioDevice,
    event::{Event, WindowEvent},
    keyboard::Keycode,
};
use thiserror::Error;

use crate::{
    audio::{AudioError, GbAudio, create_audio_device},
    frame::FrameTimer,
    logger::initilize_logger,
    window::{WindowError, WindowManager},
};

mod audio;
mod frame;
mod logger;
mod window;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Failed to initialize SDL context: {0}")]
    SdlInitError(String),
    #[error("There was an audio error: {0}")]
    AudioError(#[from] AudioError),
    #[error("There was a window erorr: {0}")]
    WindowError(#[from] WindowError),
    #[error("Failed to initialize event pump: {0}")]
    EventPumpError(String),
    #[error("Failed to read ROM file: {0}")]
    RomReadError(#[from] std::io::Error),
    #[error("There was a game boy error: {0}")]
    GameBoyError(#[from] GameBoyError),
}

pub struct Application {
    game_boy: Option<GameBoy>,
    audio_device: AudioDevice<GbAudio>,
    window_manager: WindowManager,
    event_pump: EventPump,
    frame_timer: FrameTimer,
}

impl Application {
    pub fn new(rom_path: Option<String>) -> Result<Application, ApplicationError> {
        initilize_logger();
        let sdl_context = sdl2::init().map_err(ApplicationError::SdlInitError)?;

        let audio_device = create_audio_device(&sdl_context)?;
        let window_manager = WindowManager::new(&sdl_context)?;
        let event_pump = sdl_context.event_pump().map_err(ApplicationError::EventPumpError)?;

        let game_boy = match rom_path {
            Some(rom_path) => Some(GameBoy::new(&rom_path, read_rom(&rom_path)?)?),
            None => None,
        };

        let desktop = Self {
            game_boy,
            audio_device,
            window_manager,
            event_pump,
            frame_timer: FrameTimer::new(),
        };

        Ok(desktop)
    }

    pub fn run(&mut self) -> Result<(), ApplicationError> {
        self.window_manager.render_splash()?;
        let main_window_id = self.window_manager.main_canvas().window().id();

        'game: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'game,
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        window_id,
                        ..
                    } => {
                        if window_id == main_window_id {
                            break 'game;
                        }
                    }
                    Event::Window {
                        win_event: WindowEvent::Close,
                        window_id,
                        ..
                    } => {
                        if window_id == main_window_id {
                            break 'game;
                        }
                    }
                    Event::DropFile { window_id, filename, .. } => {
                        if window_id == main_window_id {
                            self.game_boy = Some(GameBoy::new(&filename, read_rom(&filename)?)?);
                        }
                    }
                    Event::KeyDown { keycode, .. } => {
                        if let Some(ref mut game_boy) = self.game_boy {
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
                        if let Some(ref mut game_boy) = self.game_boy {
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

            self.run_game_boy()?;
        }

        Ok(())
    }

    fn run_game_boy(&mut self) -> Result<(), ApplicationError> {
        if let Some(ref mut game_boy) = self.game_boy {
            let audio_lock = self.audio_device.lock();
            let sample_count = audio_lock.sample_count();
            drop(audio_lock);

            if sample_count < SAMPLES_PER_FRAME {
                let (left_samples, right_samples) = game_boy.run_until_audio_buffer_full();
                let mut audio_lock = self.audio_device.lock();
                audio_lock.queue_samples(left_samples, right_samples);
                drop(audio_lock)
            }

            //TODO: make this toggleable
            let fps = self.frame_timer.fps();
            self.window_manager.render_screen(game_boy.current_frame(), Some(fps))?;
            self.frame_timer.slow_frame();
            self.frame_timer.count_frame();
        }

        Ok(())
    }
}

fn read_rom(rom_path: &str) -> Result<Vec<u8>, ApplicationError> {
    let mut rom = File::open(rom_path)?;
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer)?;
    Ok(buffer)
}
