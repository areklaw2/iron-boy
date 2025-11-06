use sdl2::audio::AudioCallback;

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
