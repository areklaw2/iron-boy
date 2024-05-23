use crate::bus::{Bus, Memory};

use self::{
    instructions::{get_instruction_by_opcode, instruction_name, Instruction},
    registers::{Registers, R16, R8},
};

mod execute;
pub mod instructions;
pub mod registers;

pub struct Cpu {
    bus: Bus,
    registers: Registers,
    current_opcode: u8,
    current_instruction: Instruction,
    halted: bool,
    ime: bool,
    stepping: bool,
}

impl Cpu {
    pub fn new(bus: Bus, registers: Registers) -> Self {
        Cpu {
            bus,
            registers,
            current_opcode: 0x00,
            current_instruction: Instruction::None,
            halted: false,
            ime: false,
            stepping: false,
        }
    }

    fn fetch_instruction(&mut self) {
        self.current_opcode = self.bus.mem_read(self.registers.pc);
        self.registers.pc += 1;
        self.current_instruction = get_instruction_by_opcode(self.current_opcode)
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

    fn reg_read_8(&mut self, register: R8) -> u8 {
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

    fn reg_read_16(&mut self, register: R16) -> u16 {
        match register {
            R16::BC => self.registers.bc(),
            R16::DE => self.registers.de(),
            R16::HL => self.registers.hl(),
            R16::SP => self.registers.sp,
        }
    }

    fn reg_write_8(&mut self, register: R8, data: u8) {
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

    fn reg_write_16(&mut self, register: R16, data: u16) {
        match register {
            R16::BC => self.registers.set_bc(data),
            R16::DE => self.registers.set_de(data),
            R16::HL => self.registers.set_hl(data),
            R16::SP => self.registers.sp = data,
        }
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

    pub fn cycle(&mut self) {
        if !self.halted {
            let pc = self.registers.pc;

            self.fetch_instruction();

            let flags = format!(
                "{}{}{}{}",
                if self.registers.f.bits() & (1 << 7) == 1 {
                    'Z'
                } else {
                    '-'
                },
                if self.registers.f.bits() & (1 << 6) == 1 {
                    'Z'
                } else {
                    '-'
                },
                if self.registers.f.bits() & (1 << 5) == 1 {
                    'Z'
                } else {
                    '-'
                },
                if self.registers.f.bits() & (1 << 4) == 1 {
                    'Z'
                } else {
                    '-'
                }
            );

            println!(
                "{:#06X}: {:<7} ({:#04X} {:#04X} {:#04X}) A: {:#04X} F: {flags} BC: {:#06X} DE: {:#06X} HL: {:#06X}\n",
                pc,
                instruction_name(&self.current_instruction),
                self.current_opcode,
                self.bus.mem_read(pc + 1),
                self.bus.mem_read(pc + 2),
                self.registers.a,
                self.registers.bc(),
                self.registers.de(),
                self.registers.hl()
            );

            self.execute_instructions();
        }
    }
}
