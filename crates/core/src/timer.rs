use getset::Setters;

use crate::{GbSpeed, T_CYCLES_PER_STEP, memory::SystemMemoryAccess, t_cycles};

#[derive(Setters)]
pub struct Timer {
    div: u8,
    div_counter: u32,
    tima: u8,
    tima_counter: u32,
    tma: u8,
    enabled: bool,
    clock_select: u32,
    pub interrupt: u8,
    #[getset(set = "pub")]
    speed: GbSpeed,
}

impl SystemMemoryAccess for Timer {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac(),
            _ => panic!("Timer does not handle read to address {:#4X}", address),
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.div = 0,
            0xFF05 => self.tima = value,
            0xFF06 => self.tma = value,
            0xFF07 => self.set_tac(value),
            _ => panic!("Timer does not handle write to address {:4X}", address),
        }
    }
}

impl Timer {
    pub fn new(speed: GbSpeed) -> Self {
        Timer {
            div: 0,
            div_counter: 0,
            tima: 0,
            tima_counter: 0,
            tma: 0,
            enabled: false,
            clock_select: 256,
            interrupt: 0,
            speed,
        }
    }

    pub fn cycle(&mut self) {
        self.div_counter += t_cycles(self.speed) as u32;
        while self.div_counter >= 256 {
            self.div = self.div.wrapping_add(1);
            self.div_counter -= 256
        }

        if self.enabled {
            self.tima_counter += T_CYCLES_PER_STEP as u32;
            while self.tima_counter >= self.clock_select {
                self.tima = self.tima.wrapping_add(1);
                if self.tima == 0 {
                    self.tima = self.tma;
                    self.interrupt = 0b100;
                }
                self.tima_counter -= self.clock_select;
            }
        }
    }

    fn tac(&self) -> u8 {
        0xF8 | (self.enabled as u8) << 2
            | (match self.clock_select {
                16 => 0b01,
                64 => 0b10,
                256 => 0b11,
                _ => 0,
            })
    }

    fn set_tac(&mut self, value: u8) {
        self.enabled = (value & 0b100) != 0;
        self.clock_select = match value & 0b011 {
            0b01 => 16, // T-cyles
            0b10 => 64,
            0b11 => 256,
            _ => 1024,
        };
    }
}
