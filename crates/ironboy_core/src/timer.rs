use std::{cell::RefCell, rc::Rc};

use getset::{CopyGetters, Setters};

use crate::{GbSpeed, T_CYCLES_PER_STEP, cpu::CPU_CLOCK_SPEED, system_bus::SystemMemoryAccess, t_cycles};

const DIV_INCREMENT_CLOCK_SPEED: u16 = 16384;
const DIV_INCREMENT_T_CYCLES: u16 = (CPU_CLOCK_SPEED / DIV_INCREMENT_CLOCK_SPEED as u32) as u16;

#[derive(Setters, CopyGetters)]
pub struct Timer {
    #[getset(get_copy = "pub")]
    div: u8,
    div_cycles: u16,
    tima: u8,
    tima_cycles: u16,
    tma: u8,
    enabled: bool,
    clock_select: u16,
    interrupt_flag: Rc<RefCell<u8>>,
}

impl SystemMemoryAccess for Timer {
    fn read_8(&self, address: u16) -> u8 {
        match address {
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => self.tac(),
            _ => panic!("Timer does not handle read from address {:#4X}", address),
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
    pub fn new(interrupt_flag: Rc<RefCell<u8>>) -> Self {
        Timer {
            div: 0,
            div_cycles: 0,
            tima: 0,
            tima_cycles: 0,
            tma: 0,
            enabled: false,
            clock_select: 256,
            interrupt_flag,
        }
    }

    pub fn cycle(&mut self, speed: GbSpeed) {
        self.div_cycles += t_cycles(speed) as u16;
        if self.div_cycles >= DIV_INCREMENT_T_CYCLES {
            self.div = self.div.wrapping_add(1);
            self.div_cycles -= DIV_INCREMENT_T_CYCLES
        }

        if self.enabled {
            self.tima_cycles += T_CYCLES_PER_STEP as u16;
            if self.tima_cycles >= self.clock_select {
                self.tima = self.tima.wrapping_add(1);
                if self.tima == 0 {
                    self.tima = self.tma;
                    *self.interrupt_flag.borrow_mut() |= 0b100;
                }
                self.tima_cycles -= self.clock_select;
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
