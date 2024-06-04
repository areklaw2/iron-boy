use crate::bus::Memory;

#[derive(Debug, Clone, Copy)]
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

pub struct Joypad {
    row0: u8,
    row1: u8,
    data: u8,
    pub interrupt: u8,
}

impl Memory for Joypad {
    fn mem_read(&mut self, _: u16) -> u8 {
        self.data
    }

    fn mem_write(&mut self, _: u16, data: u8) {
        self.data = (self.data & 0xCF) | (data & 0x30);
        self.update_buttons();
    }
}

impl Joypad {
    pub fn new() -> Self {
        Joypad {
            row0: 0x0F,
            row1: 0x0F,
            data: 0xFF,
            interrupt: 0,
        }
    }

    fn update_buttons(&mut self) {
        let old_values = self.data & 0xF;
        let mut new_values = 0xF;

        if self.data & 0x10 == 0x00 {
            new_values &= self.row0;
        }
        if self.data & 0x20 == 0x00 {
            new_values &= self.row1;
        }

        if old_values == 0xF && new_values != 0xF {
            self.interrupt = 0b10000;
        }

        self.data = (self.data & 0xF0) | new_values;
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
