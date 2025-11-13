use getset::{Getters, MutGetters};
use ironboy_core::{FPS, GameBoy, SAMPLES_PER_FRAME, gb::GameBoyError};
use std::{fs::File, io::Read};

use sdl2::{Sdl, audio::AudioDevice};
use thiserror::Error;

use crate::{
    audio::{AudioError, GbAudio, create_audio_device},
    logger::initilize_logger,
    window::{WindowError, WindowManager},
};

pub mod audio;
mod logger;
pub mod window;

const FRAME_DURATION_NANOS: f32 = 1_000_000_000.0 / FPS;
const FRAME_DURATION: std::time::Duration = std::time::Duration::from_nanos(FRAME_DURATION_NANOS as u64);

#[derive(Error, Debug)]
pub enum DesktopError {
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
pub struct Desktop {
    #[getset(get_mut = "pub")]
    game_boy: Option<GameBoy>,
    #[getset(get_mut = "pub")]
    audio_device: AudioDevice<GbAudio>,
    #[getset(get = "pub", get_mut = "pub")]
    window_manager: WindowManager,
    pub sdl_context: Sdl,
}

impl Desktop {
    pub fn new(rom_path: Option<String>) -> Result<Desktop, DesktopError> {
        initilize_logger();
        let sdl_context = sdl2::init().map_err(DesktopError::SdlInitError)?;

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
        };

        Ok(desktop)
    }

    pub fn load_rom(&mut self, rom_path: String) -> Result<(), DesktopError> {
        self.game_boy = Some(GameBoy::new(&rom_path, read_rom(&rom_path)?)?);
        Ok(())
    }

    pub fn run(&mut self, frame_clock: &mut std::time::Instant, fps_timer: &mut std::time::Instant, frame_count: &mut i32) {
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

            let time_elapsed = frame_clock.elapsed();
            if time_elapsed < FRAME_DURATION {
                std::thread::sleep(FRAME_DURATION - time_elapsed);
            }
            *frame_clock = std::time::Instant::now();

            // FPS counter
            // TODO: make this toggleable
            *frame_count += 1;
            let fps_elapsed = fps_timer.elapsed();
            if fps_elapsed.as_secs() >= 1 {
                let actual_fps = *frame_count as f64 / fps_elapsed.as_secs_f64();
                println!(
                    "FPS: {:.2} (Target: {:.2}) | Frame time: {:.2}ms",
                    actual_fps,
                    FPS,
                    time_elapsed.as_secs_f64() * 1000.0
                );
                *frame_count = 0;
                *fps_timer = std::time::Instant::now();
            }
        }
    }
}

fn read_rom(rom_path: &str) -> Result<Vec<u8>, DesktopError> {
    let mut rom = File::open(rom_path)?;
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer)?;
    Ok(buffer)
}
