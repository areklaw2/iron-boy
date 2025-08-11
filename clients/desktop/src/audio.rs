use ironboy_core::{SAMPLING_FREQUENCY, SAMPLING_RATE, gb::GameBoy};
use sdl2::{
    Sdl,
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
};

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

pub struct GbAudioCallback<'a> {
    pub audio_buffer: &'a mut Arc<Mutex<VecDeque<u8>>>,
    left_master: &'a u8,
    right_master: &'a u8,
    volume: &'a u8,
}

impl<'a> GbAudioCallback<'a> {
    pub fn new(audio_buffer: &'a mut Arc<Mutex<VecDeque<u8>>>, left_master: &'a u8, right_master: &'a u8, volume: &'a u8) -> Self {
        Self {
            audio_buffer,
            left_master,
            right_master,
            volume,
        }
    }
}

impl AudioCallback for GbAudioCallback<'_> {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for (i, sample) in out.iter_mut().enumerate() {
            if !self.audio_buffer.lock().unwrap().is_empty() {
                let master_volume = if i % 2 == 0 { self.left_master } else { self.right_master };

                *sample = self.audio_buffer.lock().unwrap().pop_front().unwrap() as f32 * (*self.volume as f32 / 10000.0) * *master_volume as f32;
            }
        }
    }
}

pub fn create_audio_device<'a>(game_boy: &'a mut GameBoy, sdl_context: &'a Sdl) -> AudioDevice<GbAudioCallback<'a>> {
    let audio_spec_desired = AudioSpecDesired {
        freq: Some(SAMPLING_FREQUENCY as i32),
        samples: Some(SAMPLING_RATE),
        channels: Some(2),
    };

    let left_volume = &game_boy.cpu.bus.apu.left_volume;
    let right_volume = &game_boy.cpu.bus.apu.right_volume;
    let volume = &game_boy.volume;
    let audio = GbAudioCallback::new(&mut game_boy.cpu.bus.apu.audio_buffer, left_volume, right_volume, volume);

    let audio_subsystem = sdl_context.audio().unwrap();
    audio_subsystem.open_playback(None, &audio_spec_desired, |_spec| audio).unwrap()
}
