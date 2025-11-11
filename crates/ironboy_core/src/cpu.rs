use std::{cell::RefCell, rc::Rc};

use getset::{CopyGetters, Getters, MutGetters, Setters};
use tracing::debug;

use crate::{GbMode, cpu::instructions::Instruction, interrupts::InterruptKind};

use self::registers::Registers;

mod dissassemble;
mod execute;
pub mod instructions;
mod operands;
mod registers;

pub const CPU_CLOCK_SPEED: u32 = 4194304;

#[derive(Getters, MutGetters, CopyGetters, Setters)]
pub struct Cpu<I: MemoryInterface> {
    #[getset(get = "pub", get_mut = "pub")]
    bus: I,
    #[getset(get = "pub", get_mut = "pub")]
    registers: Registers,
    interrupt_master_enable: bool,
    enable_interrupt_delay: u8,
    disable_interrupt_delay: u8,
    halted: Rc<RefCell<bool>>,
    halt_bug: bool,
    opcode: u8,
    #[getset(get = "pub")]
    instruction: Instruction,
    debugging: bool,
    disassemble: bool,
}

impl<I: MemoryInterface> Cpu<I> {
    pub fn new(bus: I, mode: GbMode, halted: Rc<RefCell<bool>>) -> Self {
        Cpu {
            bus,
            registers: Registers::new(mode),
            interrupt_master_enable: false,
            enable_interrupt_delay: 0,
            disable_interrupt_delay: 0,
            halted,
            halt_bug: false,
            opcode: 0x00,
            instruction: Instruction::Nop,
            //TODO: add flags for this
            debugging: false,
            disassemble: false,
        }
    }

    pub fn cycle(&mut self) {
        if !*self.halted.borrow() {
            self.update_interrupt_master_enable();
            self.execute_instruction();
            self.log_cycle(self.registers.pc());
        } else {
            self.bus.m_cycle();
        }

        self.execute_interrupt();

        if !*self.halted.borrow() {
            self.fetch_instruction();
        }
    }

    pub fn fetch_instruction(&mut self) {
        self.opcode = self.bus.load_8(self.registers.pc(), true);
        self.instruction = Instruction::from(self.opcode);
        if self.halt_bug {
            self.halt_bug = false;
        } else {
            self.registers.set_pc(self.registers.pc().wrapping_add(1));
        }
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.bus.load_8(self.registers.pc(), true);
        self.registers.set_pc(self.registers.pc().wrapping_add(1));
        byte
    }

    fn fetch_word(&mut self) -> u16 {
        let word = self.bus.load_16(self.registers.pc(), true);
        self.registers.set_pc(self.registers.pc().wrapping_add(2));
        word
    }

    fn pop_stack(&mut self) -> u16 {
        let value = self.bus.load_16(self.registers.sp(), true);
        self.registers.set_sp(self.registers.sp().wrapping_add(2));
        value
    }

    fn push_stack(&mut self, value: u16) {
        self.bus.m_cycle();
        self.registers.set_sp(self.registers.sp().wrapping_sub(2));
        self.bus.store_16(self.registers.sp(), value, true);
    }

    fn execute_interrupt(&mut self) {
        let reqeusted_interupt = self.bus.pending_interrupt();
        if reqeusted_interupt == 0 {
            return;
        }

        self.bus.m_cycle();
        if *self.halted.borrow() {
            *self.halted.borrow_mut() = false;
        }

        if !self.interrupt_master_enable {
            return;
        }

        self.interrupt_master_enable = false;
        self.bus.m_cycle();
        self.push_stack(self.registers.pc());

        let interrupt_bit = reqeusted_interupt.trailing_zeros() as u8;
        self.bus.clear_interrupt(interrupt_bit);
        let interrupt_kind = match interrupt_bit {
            0 => InterruptKind::VBlank,
            1 => InterruptKind::Lcd,
            2 => InterruptKind::Timer,
            3 => InterruptKind::Serial,
            4 => InterruptKind::Joypad,
            _ => panic!("Interrupt not valid"),
        };

        self.registers.set_pc(interrupt_kind.source_address());
    }

    pub fn update_interrupt_master_enable(&mut self) {
        if self.disable_interrupt_delay == 1 {
            self.interrupt_master_enable = false;
        }
        self.disable_interrupt_delay = self.disable_interrupt_delay.saturating_sub(1);

        if self.enable_interrupt_delay == 1 {
            self.interrupt_master_enable = true;
        }
        self.enable_interrupt_delay = self.enable_interrupt_delay.saturating_sub(1);
    }

    fn log_cycle(&mut self, pc: u16) {
        if !self.debugging {
            return;
        }

        let disassemble = match self.disassemble {
            true => {
                let next_byte = self.bus.load_8(pc.wrapping_add(1), false);
                let next_word = self.bus.load_16(pc.wrapping_add(1), false);
                format!("{:<20} ->: ", self.instruction.disassemble_instruction(self.opcode, next_byte, next_word))
            }
            false => "".to_string(),
        };

        let byte0 = self.bus.load_8(pc, false);
        let byte1 = self.bus.load_8(pc.wrapping_add(1), false);
        let byte2 = self.bus.load_8(pc.wrapping_add(2), false);
        let byte3 = self.bus.load_8(pc.wrapping_add(3), false);

        let log_line = format!(
            "{}A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}",
            disassemble,
            self.registers.a(),
            self.registers.f().into_bits(),
            self.registers.b(),
            self.registers.c(),
            self.registers.d(),
            self.registers.e(),
            self.registers.h(),
            self.registers.l(),
            self.registers.sp(),
            pc,
            byte0,
            byte1,
            byte2,
            byte3
        );

        debug!("{}", log_line.trim());
    }
}

pub trait MemoryInterface {
    fn load_8(&mut self, address: u16, with_cycles: bool) -> u8;

    fn load_16(&mut self, address: u16, with_cycles: bool) -> u16 {
        let lo = self.load_8(address, with_cycles) as u16;
        let hi = self.load_8(address + 1, with_cycles) as u16;
        hi << 8 | lo
    }

    fn store_8(&mut self, address: u16, value: u8, with_cycles: bool);

    fn store_16(&mut self, address: u16, value: u16, with_cycles: bool) {
        self.store_8(address, (value & 0xFF) as u8, with_cycles);
        self.store_8(address + 1, (value >> 8) as u8, with_cycles);
    }

    fn m_cycle(&mut self);

    fn total_m_cycles(&self) -> u64;

    fn pending_interrupt(&self) -> u8;

    fn clear_interrupt(&mut self, mask: u8);

    fn change_speed(&mut self);
}
