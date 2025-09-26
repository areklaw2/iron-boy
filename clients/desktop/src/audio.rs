use core::{GameBoy, SAMPLING_FREQUENCY, SAMPLING_RATE};
use sdl2::{
    Sdl,
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
};

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub struct GbAudio<'a> {
    pub audio_buffer: &'a mut Arc<Mutex<VecDeque<f32>>>,
    volume: &'a u8,
}

impl<'a> GbAudio<'a> {
    pub fn new(audio_buffer: &'a mut Arc<Mutex<VecDeque<f32>>>, volume: &'a u8) -> Self {
        Self { audio_buffer, volume }
    }
}

impl AudioCallback for GbAudio<'_> {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        let mut buffer = self.audio_buffer.lock().unwrap();
        for sample in out.iter_mut() {
            if !buffer.is_empty() {
                *sample = buffer.pop_front().unwrap() as f32 * (*self.volume as f32 / 256.0);
            } else {
                *sample = 0.0;
            }
        }
    }
}

pub fn create_audio_device<'a>(game_boy: &'a mut GameBoy, sdl_context: &'a Sdl) -> AudioDevice<GbAudio<'a>> {
    let audio_spec_desired = AudioSpecDesired {
        freq: Some(SAMPLING_FREQUENCY as i32),
        samples: Some(SAMPLING_RATE),
        channels: Some(2),
    };

    let volume = &game_boy.volume;
    let audio = GbAudio::new(&mut game_boy.cpu.bus.apu.audio_buffer, volume);

    let audio_subsystem = sdl_context.audio().unwrap();
    audio_subsystem.open_playback(None, &audio_spec_desired, |_spec| audio).unwrap()
}
