use bitfields::bitfield;

#[bitfield(u8, order = msb)]
#[derive(Clone, Copy)]
pub struct Mixer {
    ch4_left: bool,
    ch3_left: bool,
    ch2_left: bool,
    ch1_left: bool,
    ch4_right: bool,
    ch3_right: bool,
    ch2_right: bool,
    ch1_right: bool,
}

impl Mixer {
    pub fn mix(&self, channels_outputs: [u8; 4]) -> (u8, u8) {
        let mut output_left = 0;
        let mut output_right = 0;
        let panning = self.into_bits();

        for (i, output) in channels_outputs.iter().enumerate() {
            if panning & (1 << (7 - i)) != 0 {
                output_left += output;
            }
            if panning & (1 << (7 - i)) != 0 {
                output_right += output;
            }
        }
        (output_left / 4, output_right / 4)
    }
}
