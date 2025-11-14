use getset::CopyGetters;
use ironboy_core::FPS;

const FRAME_DURATION_NANOS: f32 = 1_000_000_000.0 / FPS;
const FRAME_DURATION: std::time::Duration = std::time::Duration::from_nanos(FRAME_DURATION_NANOS as u64);

#[derive(CopyGetters)]
pub struct FrameTimer {
    frame_count: u16,
    frame_clock: std::time::Instant,
    fps_clock: std::time::Instant,
    #[getset(get_copy = "pub")]
    fps: f64, // add fps toggle as an overlay
}

impl FrameTimer {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            frame_clock: std::time::Instant::now(),
            fps_clock: std::time::Instant::now(),
            fps: 0.0,
        }
    }

    pub fn slow_frame(&mut self) {
        // slows frame rate down to the expected fps
        let time_elapsed = self.frame_clock.elapsed();
        if time_elapsed < FRAME_DURATION {
            std::thread::sleep(FRAME_DURATION - time_elapsed);
        }
        self.frame_clock = std::time::Instant::now();
    }

    pub fn count_frame(&mut self) {
        self.frame_count += 1;
        let time_elapsed = self.fps_clock.elapsed();
        if time_elapsed.as_secs() >= 1 {
            self.fps = self.frame_count as f64 / time_elapsed.as_secs_f64();
            self.frame_count = 0;
            self.fps_clock = std::time::Instant::now();
        }
    }
}
