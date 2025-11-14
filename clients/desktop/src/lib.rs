use getset::{Getters, MutGetters};
use ironboy_core::{FPS, GameBoy, SAMPLES_PER_FRAME, gb::GameBoyError};
use std::{fs::File, io::Read};

use sdl2::{Sdl, audio::AudioDevice};
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
    #[error("Failed to initialize SDL contect: `{0}`")]
    SdlInitError(String),
    #[error("There was an audio error")]
    AudioError(#[from] AudioError),
    #[error("There was a window erorr")]
    WindowError(#[from] WindowError),
    #[error("Failed to read ROM file")]
    RomReadError(#[from] std::io::Error),
    #[error("There was a game boy error")]
    GameBoyError(#[from] GameBoyError),
}

#[derive(Getters, MutGetters)]
pub struct Application {
    #[getset(get_mut = "pub")]
    game_boy: Option<GameBoy>,
    #[getset(get_mut = "pub")]
    audio_device: AudioDevice<GbAudio>,
    #[getset(get = "pub", get_mut = "pub")]
    window_manager: WindowManager,
    pub sdl_context: Sdl,
    fps_counter: FrameTimer,
}

impl Application {
    pub fn new(rom_path: Option<String>) -> Result<Application, ApplicationError> {
        initilize_logger();
        let sdl_context = sdl2::init().map_err(ApplicationError::SdlInitError)?;

        let audio_device = create_audio_device(&sdl_context)?;
        let window_manager = WindowManager::new(&sdl_context)?;

        let game_boy = match rom_path {
            Some(rom_path) => Some(GameBoy::new(&rom_path, read_rom(&rom_path)?)?),
            None => None,
        };

        let desktop = Self {
            game_boy,
            audio_device,
            window_manager,
            sdl_context,
            fps_counter: FrameTimer::new(),
        };

        Ok(desktop)
    }

    pub fn load_rom(&mut self, rom_path: String) -> Result<(), ApplicationError> {
        self.game_boy = Some(GameBoy::new(&rom_path, read_rom(&rom_path)?)?);
        Ok(())
    }

    pub fn run_game_boy(&mut self) {
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

            self.window_manager.render_screen(game_boy.current_frame());
            self.fps_counter.slow_frame();
            self.fps_counter.count_frame()
        }
    }
}

fn read_rom(rom_path: &str) -> Result<Vec<u8>, ApplicationError> {
    let mut rom = File::open(rom_path)?;
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer)?;
    Ok(buffer)
}
