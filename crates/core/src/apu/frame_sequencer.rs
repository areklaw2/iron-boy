use crate::GbSpeed;

use super::{channel::Channel, noise::NoiseChannel, square::SquareChannel, wave::WaveChannel};

pub struct FrameSequencer {
    step: u8,
    div_bit: bool,
}

impl FrameSequencer {
    pub fn new() -> Self {
        Self { step: 0, div_bit: false }
    }

    pub fn cycle(
        &mut self,
        ch1: &mut SquareChannel,
        ch2: &mut SquareChannel,
        ch3: &mut WaveChannel,
        ch4: &mut NoiseChannel,
        div: u8,
        speed: GbSpeed,
    ) {
        let div_bit = match speed {
            GbSpeed::Normal => div & (1 << 4) != 0,
            GbSpeed::Double => div & (1 << 5) != 0,
        };

        if self.div_bit && !div_bit {
            self.step = (self.step + 1) & 0x07;
            match self.step {
                0 | 4 => self.length_timer_cycle(ch1, ch2, ch3, ch4),
                2 | 6 => {
                    ch1.sweep_cycle();
                    self.length_timer_cycle(ch1, ch2, ch3, ch4);
                }
                7 => self.volume_envelope_cycle(ch1, ch2, ch4),
                _ => {}
            }
        }
        self.div_bit = div_bit;
    }

    fn length_timer_cycle(&mut self, ch1: &mut SquareChannel, ch2: &mut SquareChannel, ch3: &mut WaveChannel, ch4: &mut NoiseChannel) {
        ch1.length_timer_cycle();
        ch2.length_timer_cycle();
        ch3.length_timer_cycle();
        ch4.length_timer_cycle();
    }

    fn volume_envelope_cycle(&mut self, ch1: &mut SquareChannel, ch2: &mut SquareChannel, ch4: &mut NoiseChannel) {
        ch1.volume_envelope_cycle();
        ch2.volume_envelope_cycle();
        ch4.volume_envelope_cycle();
    }

    pub fn reset(&mut self) {
        self.step = 0;
    }
}
