use getset::CopyGetters;

use crate::{GbSpeed, memory::SystemMemoryAccess};

#[derive(CopyGetters)]
pub struct SpeedSwitch {
    #[getset(get_copy = "pub")]
    speed: GbSpeed,
    switch_armed: bool,
}

impl SystemMemoryAccess for SpeedSwitch {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0xFF4D => ((self.speed as u8) << 7) | 0x7E | (self.switch_armed as u8),
            _ => panic!("Speed Switch does not handle read to address {:#4X}", address),
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0xFF4D => self.switch_armed = value & 0x1 != 0,
            _ => panic!("Speed Switch does not handle write to address {:#4X}", address),
        }
    }
}

impl SpeedSwitch {
    pub fn new() -> Self {
        SpeedSwitch {
            speed: GbSpeed::Normal,
            switch_armed: false,
        }
    }

    pub fn change_speed(&mut self) {
        if self.switch_armed {
            self.speed = match self.speed {
                GbSpeed::Normal => GbSpeed::Double,
                GbSpeed::Double => GbSpeed::Normal,
            };
            self.switch_armed = false;
        }
    }
}
