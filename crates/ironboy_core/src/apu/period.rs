use getset::{CopyGetters, Setters};

#[derive(Debug, CopyGetters, Setters)]
#[getset(get_copy = "pub", set = "pub")]
pub struct Period {
    low: u8,
    high: u8,
    timer: u16,
    reloaded: bool,
}

const WAVE_CHANNEL_PERIOD_DELAY: u16 = 3;

impl Period {
    pub fn new() -> Self {
        Period {
            low: 0,
            high: 0,
            timer: 0,
            reloaded: false,
        }
    }

    pub fn cycle(&mut self, mut increment: u8, mut handle_divider_reload: impl FnMut()) {
        self.reloaded = false;
        while increment > 0 {
            self.timer -= 1;
            if self.timer == 0 {
                self.timer = self.period_timer();
                handle_divider_reload();
                self.reloaded = true;
            }
            increment -= 1;
        }
        if self.timer != self.period_timer() {
            self.reloaded = false;
        }
    }

    fn period_timer(&self) -> u16 {
        2048 - self.value()
    }

    pub fn value(&self) -> u16 {
        let period_high = (self.high & 0b111) as u16;
        let period_low = self.low as u16;
        (period_high << 8) | period_low
    }

    pub fn trigger(&mut self) {
        self.timer = self.period_timer();
    }

    pub fn wave_channel_trigger_delay(&mut self) {
        self.timer += WAVE_CHANNEL_PERIOD_DELAY;
    }
}
