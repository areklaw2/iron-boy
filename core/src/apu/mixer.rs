pub struct Mixer {
    panning: [bool; 8],
}

impl Mixer {
    pub fn new() -> Self {
        Self { panning: [false; 8] }
    }

    pub fn mix(&self, channels_outputs: [u8; 4]) -> (u8, u8) {
        let (mut output_left, mut output_right) = (0, 0);
        for (i, output) in channels_outputs.iter().enumerate() {
            if self.panning[i + 4] {
                output_left += output;
            }
            if self.panning[i] {
                output_right += output;
            }
        }
        (output_left / 4, output_right / 4)
    }

    pub fn read(&self) -> u8 {
        let mut data = 0;
        for i in 0..self.panning.len() {
            data |= self.panning[i] as u8
        }
        data
    }

    pub fn write(&mut self, value: u8) {
        for i in 0..self.panning.len() {
            self.panning[i] = value & (1 << i) == (1 << i);
        }
    }

    pub fn reset(&mut self) {
        self.panning = [false; 8];
    }
}
