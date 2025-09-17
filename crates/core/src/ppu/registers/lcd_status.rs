use getset::{CopyGetters, Setters};

use super::PpuMode;

#[derive(CopyGetters, Setters)]
pub struct LcdStatus {
    #[getset(get_copy = "pub")]
    lyc_interrupt: bool,
    mode2_interrupt: bool,
    mode1_interrupt: bool,
    mode0_interrupt: bool,
    #[getset(get_copy = "pub", set = "pub")]
    lyc_equals_ly: bool,
    #[getset(get_copy = "pub")]
    mode: PpuMode,
}

impl LcdStatus {
    pub fn new() -> Self {
        LcdStatus {
            lyc_interrupt: false,
            mode2_interrupt: false,
            mode1_interrupt: false,
            mode0_interrupt: false,
            lyc_equals_ly: false,
            mode: PpuMode::VBlank,
        }
    }

    pub fn set_mode(&mut self, mode: PpuMode) -> bool {
        if self.mode == mode {
            return false;
        }

        self.mode = mode;
        match self.mode {
            PpuMode::HBlank => self.mode0_interrupt,
            PpuMode::VBlank => self.mode1_interrupt,
            PpuMode::OamScan => self.mode1_interrupt,
            PpuMode::DrawingPixels => false,
        }
    }
}

impl From<&LcdStatus> for u8 {
    fn from(lcd_status: &LcdStatus) -> Self {
        (lcd_status.lyc_interrupt as u8) << 6
            | (lcd_status.mode2_interrupt as u8) << 5
            | (lcd_status.mode1_interrupt as u8) << 4
            | (lcd_status.mode0_interrupt as u8) << 3
            | (lcd_status.lyc_equals_ly as u8) << 2
            | (lcd_status.mode as u8)
    }
}

impl From<u8> for LcdStatus {
    fn from(value: u8) -> Self {
        LcdStatus {
            lyc_interrupt: (value & 0x40) != 0,
            mode2_interrupt: (value & 0x20) != 0,
            mode1_interrupt: (value & 0x10) != 0,
            mode0_interrupt: (value & 0x08) != 0,
            lyc_equals_ly: (value & 0x04) != 0,
            mode: (value & 0x03).into(),
        }
    }
}
