use std::collections::VecDeque;

use sdl2::audio::AudioCallback;

pub struct Audio {
    pub audio_buffer: VecDeque<u8>,
    left_master: u8,
    right_master: u8,
}

impl Audio {
    pub fn new(audio_buffer: VecDeque<u8>, left_master: u8, right_master: u8) -> Self {
        Self {
            audio_buffer,
            left_master,
            right_master,
        }
    }
}

impl AudioCallback for Audio {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {}
}
