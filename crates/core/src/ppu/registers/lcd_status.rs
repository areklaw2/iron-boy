use super::PpuMode;

pub struct LcdStatus {
    lyc_interrupt: bool,
    mode2_interrupt: bool,
    mode1_interrupt: bool,
    mode0_interrupt: bool,
    lyc_equals_ly: bool,
    pub mode: PpuMode,
}

impl LcdStatus {
    pub fn new() -> Self {
        LcdStatus {
            lyc_interrupt: false,
            mode2_interrupt: false,
            mode1_interrupt: false,
            mode0_interrupt: false,
            lyc_equals_ly: false,
            mode: PpuMode::OamScan,
        }
    }

    pub fn lyc_interrupt(&self) -> bool {
        self.lyc_interrupt
    }

    pub fn lyc_equals_ly(&self) -> bool {
        self.lyc_equals_ly
    }

    pub fn set_lyc_equals_ly(&mut self, status: bool) {
        self.lyc_equals_ly = status
    }

    pub fn mode(&self) -> PpuMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: PpuMode) -> bool {
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
