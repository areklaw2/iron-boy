use super::channel::ChannelBase;

// May make since to no thave from traits
pub struct Mixer {
    pub panning: [bool; 8],
}

impl Mixer {
    pub fn new() -> Self {
        Self { panning: [false; 8] }
    }

    pub fn mix(&self, channels: [&ChannelBase; 4]) -> (u8, u8) {
        let (mut output_left, mut output_right) = (0, 0);
        for (i, channel) in channels.iter().enumerate() {
            if self.panning[i + 4] {
                output_left += channel.get_output();
            }
            if self.panning[i] {
                output_right += channel.get_output();
            }
        }
        (output_left / 4, output_right / 4)
    }

    pub fn reset(&mut self) {
        self.panning = [false; 8];
    }
}

impl From<u8> for Mixer {
    fn from(value: u8) -> Self {
        let mut mixer = Mixer { panning: [false; 8] };
        for i in 0..mixer.panning.len() {
            mixer.panning[i] = value & (1 << i) == (1 << i);
        }
        mixer
    }
}

impl From<&Mixer> for u8 {
    fn from(mixer: &Mixer) -> Self {
        let mut data = 0;
        for i in 0..mixer.panning.len() {
            data |= mixer.panning[i] as u8
        }
        data
    }
}
