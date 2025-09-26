use core::{GameBoy, SAMPLING_FREQUENCY};
use sdl2::{
    Sdl,
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
};

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub struct GbAudio {
    pub audio_buffer: Arc<Mutex<VecDeque<f32>>>,
    volume: u8,
}

impl GbAudio {
    pub fn new(audio_buffer: Arc<Mutex<VecDeque<f32>>>, volume: u8) -> Self {
        Self { audio_buffer, volume }
    }
}

impl AudioCallback for GbAudio {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        let mut buffer = self.audio_buffer.lock().unwrap();
        for sample in out.iter_mut() {
            if !buffer.is_empty() {
                *sample = buffer.pop_front().unwrap() * (self.volume as f32 / 1000.0);
            } else {
                *sample = 0.0;
            }
        }
    }
}

pub fn create_audio_device(game_boy: &mut GameBoy, sdl_context: &Sdl) -> AudioDevice<GbAudio> {
    let audio_spec_desired = AudioSpecDesired {
        freq: Some(SAMPLING_FREQUENCY as i32),
        samples: Some(512),
        channels: Some(2),
    };

    let audio = GbAudio::new(game_boy.audio_buffer().clone(), *game_boy.volume());

    let audio_subsystem = sdl_context.audio().unwrap();
    audio_subsystem.open_playback(None, &audio_spec_desired, |_spec| audio).unwrap()
}
