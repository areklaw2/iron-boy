use std::{cell::RefCell, rc::Rc};

use crate::{
    JoypadButton,
    cartridge::Cartridge,
    cpu::{Cpu, registers::Registers},
    memory::system_bus::SystemBus,
    utils::{event::EventType, scheduler::Scheduler},
};

pub struct GameBoy {
    pub cpu: Cpu<SystemBus>,
    scheduler: Rc<RefCell<Scheduler>>,
    game_title: String,
    pub volume: u8,
}

impl GameBoy {
    pub fn new(rom_name: &str, buffer: Vec<u8>) -> GameBoy {
        let cartridge = Cartridge::load(rom_name.into(), buffer).unwrap();
        let game_title = cartridge.title().to_string();
        let mode = cartridge.mode();
        let scheduler = Rc::new(RefCell::new(Scheduler::new()));
        GameBoy {
            cpu: Cpu::new(SystemBus::new(cartridge, scheduler.clone()), Registers::new(mode)),
            scheduler,
            game_title,
            volume: 50,
        }
    }

    pub fn run_frame(&mut self) -> bool {
        loop {
            while self.scheduler.borrow().timestamp() <= self.scheduler.borrow().timestamp_of_next_event() {
                let cycles = self.cpu.cycle() as usize;
                self.scheduler.borrow_mut().update(cycles);
            }

            if self.handle_events() {
                return true;
            }
        }
    }

    fn handle_events(&mut self) -> bool {
        let mut scheduler = self.scheduler.borrow_mut();
        while let Some((event, timestamp)) = scheduler.pop() {
            match event {
                EventType::FrameComplete => {
                    return true;
                }
                EventType::Timer(timer_event) => {
                    if let Some((event_type, delta_time)) = self.cpu.bus.timer.handle_event(timer_event, timestamp) {
                        scheduler.schedule_at_timestamp(event_type, timestamp + delta_time);
                    }
                }
                EventType::Ppu(ppu_event) => {
                    let events = self.cpu.bus.ppu.handle_event(ppu_event);
                    for (event_type, delta_time) in events {
                        scheduler.schedule_at_timestamp(event_type, timestamp + delta_time);
                    }
                }
                EventType::Apu(apu_event) => {
                    if let Some((event_type, delta_time)) = self.cpu.bus.apu.handle_event(apu_event) {
                        scheduler.schedule_at_timestamp(event_type, timestamp + delta_time);
                    }
                }
            };
        }
        false
    }

    pub fn current_frame(&self) -> &Vec<(u8, u8, u8)> {
        self.cpu.bus.ppu.read_buffer()
    }

    pub fn game_title(&self) -> String {
        self.game_title.clone()
    }

    pub fn increase_volume(&mut self) {
        if self.volume > 95 {
            return;
        }
        self.volume += 5;
    }

    pub fn decrease_volume(&mut self) {
        if self.volume < 5 {
            return;
        }
        self.volume -= 5;
    }

    pub fn button_up(&mut self, button: JoypadButton) {
        self.cpu.bus.joy_pad.button_up(button)
    }

    pub fn button_down(&mut self, button: JoypadButton) {
        self.cpu.bus.joy_pad.button_down(button)
    }
}
