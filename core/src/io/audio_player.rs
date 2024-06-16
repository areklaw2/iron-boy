pub trait AudioPlayer: Send {
    fn play(&mut self, left_channel: &[f32], right_channel: &[f32]);
    fn samples_rate(&self) -> u32;
    fn underflowed(&self) -> bool;
}

struct NullAudioPlayer {}

impl AudioPlayer for NullAudioPlayer {
    fn play(&mut self, _buf_left: &[f32], _buf_right: &[f32]) {
        // Do nothing
    }

    fn samples_rate(&self) -> u32 {
        44100
    }

    fn underflowed(&self) -> bool {
        false
    }
}
