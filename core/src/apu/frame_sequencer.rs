use super::{noise::NoiseChannel, square::SquareChannel, wave::WaveChannel, APU_CLOCK_SPEED};
use crate::cpu::CPU_CLOCK_SPEED;

const CYCLES_DIV: u16 = (CPU_CLOCK_SPEED / APU_CLOCK_SPEED as u32) as u16;

pub struct FrameSequencer {
    clock: u16,
    pub step: u8,
}

impl FrameSequencer {
    pub fn new() -> Self {
        Self { clock: 0, step: 0 }
    }

    pub fn cycle(&mut self, ticks: u32, ch1: &mut SquareChannel, ch2: &mut SquareChannel, ch3: &mut WaveChannel, ch4: &mut NoiseChannel) {
        self.clock += ticks as u16;
        if self.clock >= CYCLES_DIV {
            match self.step {
                0 | 4 => self.length_timer_cycle(ch1, ch2, ch3, ch4),
                2 | 6 => {
                    ch1.sweep_cycle();
                    self.length_timer_cycle(ch1, ch2, ch3, ch4);
                }
                7 => self.envelopes_cycles(ch1, ch2, ch4),
                _ => {}
            }
            self.clock -= CYCLES_DIV;
            self.step = (self.step + 1) & 0x07;
        }
    }

    fn length_timer_cycle(&mut self, ch1: &mut SquareChannel, ch2: &mut SquareChannel, ch3: &mut WaveChannel, ch4: &mut NoiseChannel) {
        ch1.length_counter.cycle(&mut ch1.base.enabled);
        ch2.length_counter.cycle(&mut ch2.base.enabled);
        ch3.length_counter.cycle(&mut ch3.base.enabled);
        ch4.length_counter.cycle(&mut ch4.base.enabled);
    }

    fn envelopes_cycles(&mut self, ch1: &mut SquareChannel, ch2: &mut SquareChannel, ch4: &mut NoiseChannel) {
        ch1.volume_envelope.cycle(&ch1.base.enabled);
        ch2.volume_envelope.cycle(&ch2.base.enabled);
        ch4.volume_envelope.cycle(&ch4.base.enabled);
    }

    pub fn reset(&mut self) {
        self.clock = 0;
        self.step = 0;
    }
}
