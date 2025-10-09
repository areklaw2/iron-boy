use core::{SAMPLES_PER_FRAME, SAMPLING_FREQUENCY};
use sdl2::{
    Sdl,
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
};

use std::collections::VecDeque;

pub struct GbAudio {
    left_buffer: VecDeque<f32>,
    right_buffer: VecDeque<f32>,
}

impl GbAudio {
    pub fn new() -> Self {
        Self {
            left_buffer: VecDeque::new(),
            right_buffer: VecDeque::new(),
        }
    }

    pub fn queue_samples(&mut self, left: &[f32], right: &[f32]) {
        self.left_buffer.extend(left.iter());
        self.right_buffer.extend(right.iter());
    }

    pub fn sample_count(&self) -> usize {
        self.left_buffer.len()
    }
}

impl AudioCallback for GbAudio {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for (i, sample) in out.iter_mut().enumerate() {
            if i % 2 == 0 {
                *sample = self.left_buffer.pop_front().unwrap_or(0.0);
            } else {
                *sample = self.right_buffer.pop_front().unwrap_or(0.0);
            }
        }
    }
}

pub fn create_audio_device(sdl_context: &Sdl) -> AudioDevice<GbAudio> {
    let audio_spec_desired = AudioSpecDesired {
        freq: Some(SAMPLING_FREQUENCY as i32),
        samples: Some(SAMPLES_PER_FRAME as u16),
        channels: Some(2),
    };

    let audio = GbAudio::new();
    let audio_subsystem = sdl_context.audio().unwrap();
    audio_subsystem.open_playback(None, &audio_spec_desired, |_spec| audio).unwrap()
}
