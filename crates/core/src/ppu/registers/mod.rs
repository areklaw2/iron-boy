pub mod lcd_control;
pub mod lcd_status;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PpuMode {
    HBlank = 0,
    VBlank = 1,
    OamScan = 2,
    DrawingPixels = 3,
}

impl From<u8> for PpuMode {
    fn from(value: u8) -> Self {
        match value {
            0 => PpuMode::HBlank,
            1 => PpuMode::VBlank,
            2 => PpuMode::OamScan,
            _ => PpuMode::DrawingPixels,
        }
    }
}
