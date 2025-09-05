use std::{cell::RefCell, rc::Rc};

use crate::{interrupts::Interrupts, memory::SystemMemoryAccess};

pub enum JoypadButton {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Select,
    Start,
}

pub struct JoyPad {
    row0: u8,
    row1: u8,
    value: u8,
    interrupt_flags: Rc<RefCell<Interrupts>>,
}

impl SystemMemoryAccess for JoyPad {
    fn read_8(&mut self, _: u16) -> u8 {
        self.value
    }

    fn write_8(&mut self, _: u16, value: u8) {
        self.value = (self.value & 0xCF) | (value & 0x30);
        self.update_buttons();
    }
}

impl JoyPad {
    pub fn new(interrupt_flags: Rc<RefCell<Interrupts>>) -> Self {
        JoyPad {
            row0: 0x0F,
            row1: 0x0F,
            value: 0xFF,
            interrupt_flags,
        }
    }

    fn update_buttons(&mut self) {
        let values = self.value & 0xF;
        let mut updated_values = 0xF;

        if self.value & 0x10 == 0x00 {
            updated_values &= self.row0;
        }
        if self.value & 0x20 == 0x00 {
            updated_values &= self.row1;
        }

        if values == 0xF && updated_values != 0xF {
            self.interrupt_flags.borrow_mut().set_joypad(true);
        }

        self.value = (self.value & 0xF0) | updated_values;
    }

    pub fn button_down(&mut self, button: JoypadButton) {
        match button {
            JoypadButton::Right => self.row0 &= !(1 << 0),
            JoypadButton::Left => self.row0 &= !(1 << 1),
            JoypadButton::Up => self.row0 &= !(1 << 2),
            JoypadButton::Down => self.row0 &= !(1 << 3),
            JoypadButton::A => self.row1 &= !(1 << 0),
            JoypadButton::B => self.row1 &= !(1 << 1),
            JoypadButton::Select => self.row1 &= !(1 << 2),
            JoypadButton::Start => self.row1 &= !(1 << 3),
        }
        self.update_buttons();
    }

    pub fn button_up(&mut self, button: JoypadButton) {
        match button {
            JoypadButton::Right => self.row0 |= 1 << 0,
            JoypadButton::Left => self.row0 |= 1 << 1,
            JoypadButton::Up => self.row0 |= 1 << 2,
            JoypadButton::Down => self.row0 |= 1 << 3,
            JoypadButton::A => self.row1 |= 1 << 0,
            JoypadButton::B => self.row1 |= 1 << 1,
            JoypadButton::Select => self.row1 |= 1 << 2,
            JoypadButton::Start => self.row1 |= 1 << 3,
        }
        self.update_buttons();
    }
}
