use std::{cell::RefCell, rc::Rc};

use crate::{
    cpu::CPU_CLOCK_SPEED,
    memory::SystemMemoryAccess,
    utils::{
        event::{EventType, TimerEvent},
        scheduler::Scheduler,
    },
};

const TIMER_CLOCK_SPEED: u16 = 16384;
const DIV_CYCLES: u32 = CPU_CLOCK_SPEED / TIMER_CLOCK_SPEED as u32;
const INCREMENTS_TO_OVERFLOW: u32 = 256;

pub struct Timer {
    div: u8,
    div_start: usize,
    tima: u8,
    tima_start: usize,
    tma: u8,
    enabled: bool,
    clock_select: u32,
    scheduler: Rc<RefCell<Scheduler>>,
    pub interrupt: u8,
}

impl SystemMemoryAccess for Timer {
    fn read_8(&mut self, address: u16) -> u8 {
        match address {
            0xFF04 => self.div(),
            0xFF05 => self.tima(),
            0xFF06 => self.tma,
            0xFF07 => self.tac(),
            _ => panic!("Timer does not handle read to address {:4X}", address),
        }
    }

    fn write_8(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => self.set_div(),
            0xFF05 => self.set_tima(value),
            0xFF06 => self.tma = value,
            0xFF07 => self.set_tac(value),
            _ => panic!("Timer does not handle write to address {:4X}", address),
        }
    }
}

impl Timer {
    pub fn new(scheduler: Rc<RefCell<Scheduler>>) -> Self {
        Timer {
            div: 0,
            div_start: 0,
            tima: 0,
            tima_start: 0,
            tma: 0,
            enabled: false,
            clock_select: 256,
            scheduler,
            interrupt: 0,
        }
    }

    pub fn handle_event(&mut self, timer_event: TimerEvent, timestamp: usize) -> Option<(EventType, usize)> {
        match timer_event {
            TimerEvent::DivOverflow => {
                self.div = 0;
                self.div_start = timestamp;
                let cycles = DIV_CYCLES * INCREMENTS_TO_OVERFLOW;
                Some((EventType::Timer(TimerEvent::DivOverflow), cycles as usize))
            }
            TimerEvent::TimaOverflow => {
                self.interrupt = 0b100;
                self.tima = self.tma;
                self.tima_start = timestamp;
                let cycles = self.clock_select * (INCREMENTS_TO_OVERFLOW - self.tima as u32);
                match self.enabled {
                    true => Some((EventType::Timer(TimerEvent::TimaOverflow), cycles as usize)),
                    false => None,
                }
            }
        }
    }

    fn reschedule_event(&mut self, timer_event: TimerEvent) {
        self.scheduler.borrow_mut().cancel_events(EventType::Timer(timer_event));
        match timer_event {
            TimerEvent::DivOverflow => {
                let cycles = DIV_CYCLES * (INCREMENTS_TO_OVERFLOW - self.div() as u32);
                self.scheduler
                    .borrow_mut()
                    .schedule(EventType::Timer(TimerEvent::DivOverflow), cycles as usize);
            }
            TimerEvent::TimaOverflow => {
                if self.enabled {
                    let cycles = self.clock_select * (INCREMENTS_TO_OVERFLOW - self.tima() as u32);
                    self.scheduler
                        .borrow_mut()
                        .schedule(EventType::Timer(TimerEvent::TimaOverflow), cycles as usize);
                }
            }
        }
    }

    fn div(&mut self) -> u8 {
        let current_timestamp = self.scheduler.borrow().timestamp();
        self.div = ((current_timestamp - self.div_start) / DIV_CYCLES as usize) as u8;
        self.div
    }

    fn set_div(&mut self) {
        self.div = 0;
        self.div_start = self.scheduler.borrow().timestamp();
        self.reschedule_event(TimerEvent::DivOverflow);
    }

    fn tima(&mut self) -> u8 {
        if !self.enabled {
            return self.tima;
        }

        let current_timestamp = self.scheduler.borrow().timestamp();
        let time_increments = (current_timestamp - self.tima_start) / self.clock_select as usize;
        self.tima = (self.tima as usize + time_increments) as u8;
        self.tima
    }

    fn set_tima(&mut self, value: u8) {
        if self.tima == value {
            return;
        }
        self.tima = value;
        self.tima_start = self.scheduler.borrow().timestamp();
        self.reschedule_event(TimerEvent::TimaOverflow);
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
        if self.tac() == value {
            return;
        }

        let previous_enabled = self.enabled;
        let current_timestamp = self.scheduler.borrow().timestamp();
        if previous_enabled {
            let time_increments = (current_timestamp - self.tima_start) / self.clock_select as usize;
            self.tima = (self.tima as usize + time_increments) as u8;
        }

        self.enabled = (value & 0b100) != 0;
        self.clock_select = match value & 0b011 {
            0b01 => 16, // T-cyles
            0b10 => 64,
            0b11 => 256,
            _ => 1024,
        };

        self.tima_start = current_timestamp;
        self.reschedule_event(TimerEvent::TimaOverflow);
    }
}
