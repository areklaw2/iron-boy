use std::{cell::RefCell, rc::Rc};

use crate::{GbSpeed, apu::Channel};

use super::{noise::NoiseChannel, square::SquareChannel, wave::WaveChannel};

pub struct DivApuContext<'a> {
    pub(super) ch1: &'a mut SquareChannel,
    pub(super) ch2: &'a mut SquareChannel,
    pub(super) ch3: &'a mut WaveChannel,
    pub(super) ch4: &'a mut NoiseChannel,
    pub(super) div: u8,
    pub(super) speed: GbSpeed,
}

// Also known as the FrameSequencer
pub struct DivApu {
    step: Rc<RefCell<u8>>,
    div_bit: bool,
}

impl DivApu {
    pub fn new(step: Rc<RefCell<u8>>) -> Self {
        Self { step, div_bit: false }
    }

    pub fn cycle(&mut self, mut ctx: DivApuContext) {
        let new_div_bit = match ctx.speed {
            GbSpeed::Normal => ctx.div & (1 << 4) != 0,
            GbSpeed::Double => ctx.div & (1 << 5) != 0,
        };

        if self.div_bit && !new_div_bit {
            let step = *self.step.borrow();
            if matches!(step, 0 | 2 | 4 | 6) {
                self.length_timer_cycle(&mut ctx)
            }

            if matches!(step, 2 | 6) {
                ctx.ch1.sweep_cycle();
            }

            if step == 7 {
                self.volume_envelope_cycle(&mut ctx)
            }

            *self.step.borrow_mut() = (step + 1) % 8;
        }
        self.div_bit = new_div_bit;
    }

    fn length_timer_cycle(&mut self, ctx: &mut DivApuContext) {
        ctx.ch1.length_timer_cycle();
        ctx.ch2.length_timer_cycle();
        ctx.ch3.length_timer_cycle();
        ctx.ch4.length_timer_cycle();
    }

    fn volume_envelope_cycle(&mut self, ctx: &mut DivApuContext) {
        ctx.ch1.volume_envelope_cycle();
        ctx.ch2.volume_envelope_cycle();
        ctx.ch4.volume_envelope_cycle();
    }

    pub fn reset(&mut self) {
        *self.step.borrow_mut() = 0;
    }
}
