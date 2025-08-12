use ironboy_common::constants::{APU_CLOCK_SPEED, CPU_CLOCK_SPEED};

use super::{channel::Channel, noise::NoiseChannel, square::SquareChannel, wave::WaveChannel};

const CYCLES: u16 = (CPU_CLOCK_SPEED / APU_CLOCK_SPEED as u32) as u16;

pub struct FrameSequencer {
    clock: u16,
    pub step: u8,
}

impl FrameSequencer {
    pub fn new() -> Self {
        Self { clock: 0, step: 0 }
    }

    pub fn cycle(&mut self, cycles: u32, ch1: &mut SquareChannel, ch2: &mut SquareChannel, ch3: &mut WaveChannel, ch4: &mut NoiseChannel) {
        self.clock += cycles as u16;
        if self.clock >= CYCLES {
            match self.step {
                0 | 4 => self.length_timer_cycle(ch1, ch2, ch3, ch4),
                2 | 6 => {
                    ch1.sweep_cycle();
                    self.length_timer_cycle(ch1, ch2, ch3, ch4);
                }
                7 => self.volume_envelope_cycle(ch1, ch2, ch4),
                _ => {}
            }
            self.clock -= CYCLES;
            self.step = (self.step + 1) & 0x07;
        }
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
        self.clock = 0;
        self.step = 0;
    }
}
