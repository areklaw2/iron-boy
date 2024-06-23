use crate::FPS;

pub const CPU_CLOCK_SPEED: u32 = 4194304;

pub struct Clock {
    pub cycles_per_frame: f32,
    pub cycles_passed: f32,
}

impl Clock {
    pub fn new() -> Self {
        let cycles_per_frame = CPU_CLOCK_SPEED as f32 / FPS;

        Self {
            cycles_per_frame,
            cycles_passed: 0.0,
        }
    }

    pub fn tick(&mut self, ticks: u32) {
        self.cycles_passed += (ticks) as f32;
    }

    pub fn reset(&mut self) {
        self.cycles_passed = 0.0;
    }
}
