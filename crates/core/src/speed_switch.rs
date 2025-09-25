use std::{cell::RefCell, rc::Rc};

use getset::Getters;

use crate::{GbSpeed, system_bus::SystemMemoryAccess};

#[derive(Debug, Getters)]
pub struct SpeedSwitch {
    #[getset(get = "pub")]
    speed: Rc<RefCell<GbSpeed>>,
    switch_armed: bool,
}

impl SystemMemoryAccess for SpeedSwitch {
    fn read_8(&self, _address: u16) -> u8 {
        ((*self.speed.borrow() as u8) << 7) | 0x7E | (self.switch_armed as u8)
    }

    fn write_8(&mut self, _address: u16, value: u8) {
        self.switch_armed = value & 0x1 != 0;
    }
}

impl SpeedSwitch {
    pub fn new(speed: Rc<RefCell<GbSpeed>>) -> Self {
        SpeedSwitch { speed, switch_armed: false }
    }

    pub fn change_speed(&mut self) {
        if self.switch_armed {
            *self.speed.borrow_mut() = match *self.speed.borrow() {
                GbSpeed::Normal => GbSpeed::Double,
                GbSpeed::Double => GbSpeed::Normal,
            };
            self.switch_armed = false;
        }
    }
}
