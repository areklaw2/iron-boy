use std::fs::OpenOptions;
use std::io::Write;

use crate::bus::{Bus, Memory};

use self::{
    disassembler::{Instruction, R16Memory, R16Stack, R16, R8},
    registers::Registers,
};

pub mod disassembler;
mod execute;
mod interrupts;
pub mod registers;

pub const CPU_CLOCK_SPEED: u32 = 4194304;

pub struct Cpu {
    pub bus: Bus,
    registers: Registers,
    current_opcode: u8,
    current_instruction: Instruction,
    halted: bool,
    interrupt_master_enable: bool,
    enable_interrupt: u8,
    disable_interrupt: u8,
    debugging: bool,
    ticks: u32,
}

impl Cpu {
    pub fn new(bus: Bus, registers: Registers) -> Self {
        Cpu {
            bus,
            registers,
            current_opcode: 0x00,
            current_instruction: Instruction::None,
            halted: false,
            interrupt_master_enable: false,
            enable_interrupt: 0,
            disable_interrupt: 0,
            debugging: false,
            ticks: 0,
        }
    }

    pub fn cycle(&mut self) -> u32 {
        let cpu_ticks = self.cpu_cycle();
        let ticks = self.bus.machine_cycle(cpu_ticks);
        self.ticks += ticks;
        return ticks;
    }

    fn cpu_cycle(&mut self) -> u32 {
        self.update_interrupt_master_enable();
        let interrupt_cycles = self.handle_interrupt() as u32;
        if interrupt_cycles != 0 {
            return interrupt_cycles;
        }

        if self.halted {
            4
        } else {
            let pc = self.registers.pc;
            self.fetch_instruction();
            if self.debugging {
                self.log_step(pc)
            }
            self.execute_instruction() as u32
        }
    }

    fn fetch_instruction(&mut self) {
        self.current_opcode = self.bus.mem_read(self.registers.pc);
        self.registers.pc += 1;
        self.current_instruction = Instruction::from(self.current_opcode)
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.bus.mem_read(self.registers.pc);
        self.registers.pc += 1;
        byte
    }

    fn fetch_word(&mut self) -> u16 {
        let word = self.bus.mem_read_16(self.registers.pc);
        self.registers.pc += 2;
        word
    }

    fn pop_stack(&mut self) -> u16 {
        let data = self.bus.mem_read_16(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(2);
        data
    }

    fn push_stack(&mut self, data: u16) {
        self.registers.sp = self.registers.sp.wrapping_sub(2);
        self.bus.mem_write_16(self.registers.sp, data);
    }

    fn reg_read_8(&self, register: &R8) -> u8 {
        match register {
            R8::A => self.registers.a,
            R8::B => self.registers.b,
            R8::C => self.registers.c,
            R8::D => self.registers.d,
            R8::E => self.registers.e,
            R8::H => self.registers.h,
            R8::L => self.registers.l,
            R8::HLMem => self.bus.mem_read(self.registers.hl()),
        }
    }

    fn reg_read_16(&self, register: &R16) -> u16 {
        match register {
            R16::BC => self.registers.bc(),
            R16::DE => self.registers.de(),
            R16::HL => self.registers.hl(),
            R16::SP => self.registers.sp,
        }
    }

    fn memory_reg_read_16(&mut self, register: &R16Memory) -> u16 {
        match register {
            R16Memory::BC => self.registers.bc(),
            R16Memory::DE => self.registers.de(),
            R16Memory::HLI => self.registers.increment_hl(),
            R16Memory::HLD => self.registers.decrement_hl(),
        }
    }

    fn stack_reg_read_16(&self, register: &R16Stack) -> u16 {
        match register {
            R16Stack::BC => self.registers.bc(),
            R16Stack::DE => self.registers.de(),
            R16Stack::HL => self.registers.hl(),
            R16Stack::AF => self.registers.af(),
        }
    }

    fn reg_write_8(&mut self, register: &R8, data: u8) {
        match register {
            R8::A => self.registers.a = data,
            R8::B => self.registers.b = data,
            R8::C => self.registers.c = data,
            R8::D => self.registers.d = data,
            R8::E => self.registers.e = data,
            R8::H => self.registers.h = data,
            R8::L => self.registers.l = data,
            R8::HLMem => self.bus.mem_write(self.registers.hl(), data),
        }
    }

    fn reg_write_16(&mut self, register: &R16, data: u16) {
        match register {
            R16::BC => self.registers.set_bc(data),
            R16::DE => self.registers.set_de(data),
            R16::HL => self.registers.set_hl(data),
            R16::SP => self.registers.sp = data,
        }
    }

    fn stack_reg_write_16(&mut self, register: &R16Stack, data: u16) {
        match register {
            R16Stack::BC => self.registers.set_bc(data),
            R16Stack::DE => self.registers.set_de(data),
            R16Stack::HL => self.registers.set_hl(data),
            R16Stack::AF => self.registers.set_af(data),
        }
    }

    fn log_step(&self, pc: u16) {
        let flags = format!(
            "{}{}{}{}",
            if self.registers.f.bits() & (0b1000_0000) == 0b1000_0000 {
                'Z'
            } else {
                '-'
            },
            if self.registers.f.bits() & (0b0100_0000) == 0b0100_0000 {
                'N'
            } else {
                '-'
            },
            if self.registers.f.bits() & (0b0010_0000) == 0b0010_0000 {
                'H'
            } else {
                '-'
            },
            if self.registers.f.bits() & (0b0001_0000) == 0b0001_0000 {
                'C'
            } else {
                '-'
            }
        );

        let log = format!(
            "{:#06X}: {:<16} ({:#04X} {:#04X} {:#04X}) A: {:#04X} F: {flags} BC: {:#06X} DE: {:#06X} HL: {:#06X} SP: {:#06X}\n",
            pc,
            &self.current_instruction.disassemble(self.current_opcode, self.bus.mem_read(pc + 1)),
            self.current_opcode,
            self.bus.mem_read(pc + 1),
            self.bus.mem_read(pc + 2),
            self.registers.a,
            self.registers.bc(),
            self.registers.de(),
            self.registers.hl(),
            self.registers.sp,
        );

        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open("iron_boy.csv")
            .expect("Could not open file");
        file.write(log.as_bytes()).expect("Could not write file");
    }
}
