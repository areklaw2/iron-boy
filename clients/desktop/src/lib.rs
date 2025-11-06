use ironboy_core::{GameBoy, SAMPLES_PER_FRAME, SAMPLING_FREQUENCY, gb::GameBoyError};
use std::{fs::File, io::Read};

use sdl2::{
    Sdl,
    audio::{AudioDevice, AudioSpecDesired},
};
use thiserror::Error;

use crate::{audio::GbAudio, logger::initilize_logger};

pub mod audio;
mod logger;

#[derive(Error, Debug)]
pub enum DesktopError {
    #[error("Failed to initialize SDL: `{0}`")]
    SdlInitError(String),
    #[error("Failed to create audio subsystem: {0}")]
    AudioSubsystemError(String),
    #[error("Failed to open audio playback device: {0}")]
    AudioPlaybackError(String),
    #[error("Failed to read ROM file")]
    RomReadError(#[from] std::io::Error),
    #[error("Failed to start GameBoy")]
    GameBoyError(#[from] GameBoyError),
}

pub struct Desktop {
    pub game_boy: GameBoy,
    pub audio_device: AudioDevice<GbAudio>,
    pub sdl_context: Sdl,
}

impl Desktop {
    pub fn new(rom_path: String) -> Result<Desktop, DesktopError> {
        initilize_logger();
        let sdl_context = sdl2::init().map_err(DesktopError::SdlInitError)?;

        let desktop = Self {
            game_boy: GameBoy::new(&rom_path, read_rom(&rom_path)?)?,
            audio_device: create_audio_device(&sdl_context)?,
            sdl_context: sdl_context,
        };

        Ok(desktop)
    }
}

fn read_rom(rom_path: &str) -> Result<Vec<u8>, DesktopError> {
    let mut rom = File::open(rom_path)?;
    let mut buffer = Vec::new();
    rom.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn create_audio_device(sdl_context: &Sdl) -> Result<AudioDevice<GbAudio>, DesktopError> {
    let audio_spec_desired = AudioSpecDesired {
        freq: Some(SAMPLING_FREQUENCY as i32),
        samples: Some(SAMPLES_PER_FRAME as u16),
        channels: Some(2),
    };

    let audio = GbAudio::new();
    let audio_subsystem = sdl_context.audio().map_err(DesktopError::AudioSubsystemError)?;
    let device = audio_subsystem
        .open_playback(None, &audio_spec_desired, |_spec| audio)
        .map_err(DesktopError::AudioPlaybackError)?;
    Ok(device)
}
