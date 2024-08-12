use crate::bus::Memory;

pub struct SerialTransfer {
    data: u8,
    message: String,
    control: u8,
    pub interrupt: u8,
}

impl SerialTransfer {
    pub fn new() -> Self {
        SerialTransfer {
            data: 0,
            message: String::new(),
            control: 0,
            interrupt: 0,
        }
    }
}

impl Memory for SerialTransfer {
    fn mem_read(&self, address: u16) -> u8 {
        match address {
            0xFF01 => self.data,
            0xFF02 => self.control,
            _ => panic!("Serial Transfer does not handle read to address {:4X}", address),
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        match address {
            0xFF01 => {
                self.data = data;
                self.message.push(data as char);
            }
            0xFF02 => {
                self.control = data;
                if self.control == 0x81 {
                    self.interrupt = 0b1000;
                    println!("{}", self.message);
                }
            }
            _ => panic!("Serial Transfer does not handle write to address {:4X}", address),
        }
    }
}
