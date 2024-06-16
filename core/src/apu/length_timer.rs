pub struct LengthTimer {
    enabled: bool,
    value: u16,
    max_value: u16,
}

impl LengthTimer {
    pub fn new(max_value: u16) -> LengthTimer {
        LengthTimer {
            enabled: false,
            value: 0,
            max_value,
        }
    }

    pub fn active(&self) -> bool {
        self.value > 0
    }

    pub fn enabled(&mut self) -> bool {
        self.enabled
    }

    pub fn step(&mut self) {
        if self.enabled && self.value > 0 {
            self.value -= 1
        }
    }

    pub fn set(&mut self, value: u8) {
        self.value = self.max_value - value as u16;
    }

    pub fn enable(&mut self, enable: bool, frame_step: u8) {
        let prev_enabled = self.enabled;
        self.enabled = enable;
        if !prev_enabled && extra_step(frame_step) {
            self.step()
        }
    }

    pub fn trigger(&mut self, frame_step: u8) {
        if self.value == 0 {
            self.value = self.max_value;
            if extra_step(frame_step) {
                self.step()
            }
        }
    }
}

fn extra_step(frame_step: u8) -> bool {
    frame_step % 2 == 1
}
