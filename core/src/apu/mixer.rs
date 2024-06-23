use super::channel::ChannelBase;

pub struct Mixer {
    pub panning: [bool; 8],
}

impl Mixer {
    pub fn new() -> Self {
        Self { panning: [false; 8] }
    }

    pub fn read(&self) -> u8 {
        let mut data = 0;
        for i in 0..self.panning.len() {
            data |= self.panning[i] as u8
        }
        data
    }

    pub fn write(&mut self, data: u8) {
        self.panning[0] = data & 0x01 == 0x01;
        self.panning[1] = data & 0x02 == 0x02;
        self.panning[2] = data & 0x04 == 0x04;
        self.panning[3] = data & 0x08 == 0x08;
        self.panning[4] = data & 0x10 == 0x10;
        self.panning[5] = data & 0x20 == 0x20;
        self.panning[6] = data & 0x40 == 0x40;
        self.panning[7] = data & 0x80 == 0x80;
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
