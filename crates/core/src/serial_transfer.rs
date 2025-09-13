use std::{cell::RefCell, rc::Rc};

use crate::memory::SystemMemoryAccess;

pub struct SerialTransfer {
    data: u8,
    message: String,
    control: u8,
    interrupt_flag: Rc<RefCell<u8>>,
}

impl SerialTransfer {
    pub fn new(interrupt_flag: Rc<RefCell<u8>>) -> Self {
        SerialTransfer {
            data: 0,
            message: String::new(),
            control: 0,
            interrupt_flag,
        }
    }
}

impl SystemMemoryAccess for SerialTransfer {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0xFF01 => self.data,
            0xFF02 => self.control,
            _ => panic!("Serial Transfer does not handle read to address {:#4X}", address),
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0xFF01 => {
                self.data = value;
                self.message.push(value as char);
            }
            0xFF02 => {
                self.control = value;
                if self.control == 0x81 {
                    *self.interrupt_flag.borrow_mut() |= 0b1000;
                    //println!("{}", self.message);
                }
            }
            _ => panic!("Serial Transfer does not handle write to address {:#4X}", address),
        }
    }
}
