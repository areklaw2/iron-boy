use crate::memory::SystemMemoryAccess;

pub struct Timer {
    divider: u8,
    internal_divider: u32,
    counter: u8,
    internal_counter: u32,
    modulo: u8,
    enabled: bool,
    clock_select: u32,
    pub interrupt: u8,
}

impl SystemMemoryAccess for Timer {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0xFF04 => self.divider,
            0xFF05 => self.counter,
            0xFF06 => self.modulo,
            0xFF07 => {
                0b1111_1000
                    | (if self.enabled { 0b100 } else { 0 })
                    | (match self.clock_select {
                        16 => 0b01,
                        64 => 0b10,
                        256 => 0b11,
                        _ => 0,
                    })
            }
            _ => panic!("Timer does not handle read to address {:4X}", address),
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.divider = 0,
            0xFF05 => self.counter = value,
            0xFF06 => self.modulo = value,
            0xFF07 => {
                self.enabled = (value & 0b100) != 0;
                self.clock_select = match value & 0b011 {
                    0b01 => 16, // M-cyles
                    0b10 => 64,
                    0b11 => 256,
                    _ => 1024,
                };
            }
            _ => panic!("Timer does not handle write to address {:4X}", address),
        }
    }
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            divider: 0,
            internal_divider: 0,
            counter: 0,
            internal_counter: 0,
            modulo: 0,
            enabled: false,
            clock_select: 256,
            interrupt: 0,
        }
    }

    pub fn cycle(&mut self, cycles: u32) {
        self.internal_divider += cycles;
        while self.internal_divider >= 256 {
            self.divider = self.divider.wrapping_add(1);
            self.internal_divider -= 256
        }

        if self.enabled {
            self.internal_counter += cycles;
            while self.internal_counter >= self.clock_select {
                self.counter = self.counter.wrapping_add(1);
                if self.counter == 0 {
                    self.counter = self.modulo;
                    self.interrupt = 0b100;
                }
                self.internal_counter -= self.clock_select;
            }
        }
    }
}
