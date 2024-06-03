use crate::{
    bus::{Bus, Memory},
    cpu::instructions::dissassemble_instruction,
};

use self::{
    instructions::{get_instruction_by_opcode, Instruction, R16Memory, R16Stack, R16, R8},
    registers::Registers,
};

mod execute;
pub mod instructions;
mod interrupts;
pub mod registers;
mod tests;

const IF_ADDRESS: u16 = 0xFF0F;
const IE_ADDRESS: u16 = 0xFFFF;

pub struct Cpu {
    pub bus: Bus,
    registers: Registers,
    current_opcode: u8,
    current_instruction: Instruction,
    halted: bool,
    interrupt_master_enable: bool,
    enabling_interrupts: bool,
    //stepping: bool,
    ticks: u32,
    debug_message: String,
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
            enabling_interrupts: false,
            //stepping: false,
            ticks: 0,
            debug_message: String::new(),
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

    fn reg_read_8(&mut self, register: &R8) -> u8 {
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

    fn reg_read_16(&mut self, register: &R16) -> u16 {
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

    fn stack_reg_read_16(&mut self, register: &R16Stack) -> u16 {
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

    fn pop_stack(&mut self) -> u16 {
        let data = self.bus.mem_read_16(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(2);
        data
    }

    fn push_stack(&mut self, data: u16) {
        self.registers.sp = self.registers.sp.wrapping_sub(2);
        self.bus.mem_write_16(self.registers.sp, data);
    }

    pub fn cycle(&mut self) -> u32 {
        let cpu_ticks = self.cpu_cycle();
        let ticks = self.bus.machine_cycle(cpu_ticks);
        self.ticks += ticks;
        return ticks;
    }

    fn cpu_cycle(&mut self) -> u32 {
        let interrupt_cycles = self.handle_interrupt() as u32;
        if interrupt_cycles != 0 {
            return interrupt_cycles;
        }

        if self.enabling_interrupts {
            self.interrupt_master_enable = true
        }

        if self.halted {
            // Nop while waiting for interrupt
            4
        } else {
            let pc = self.registers.pc;

            self.fetch_instruction();

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

            println!(
                "{} -> {:#06X}: {:<16} ({:#04X} {:#04X} {:#04X}) A: {:#04X} F: {flags} BC: {:#06X} DE: {:#06X} HL: {:#06X}\n",
                self.ticks,
                pc,
                dissassemble_instruction(&self.current_instruction, self.current_opcode, self.bus.mem_read(pc + 1)),
                self.current_opcode,
                self.bus.mem_read(pc + 1),
                self.bus.mem_read(pc + 2),
                self.registers.a,
                self.registers.bc(),
                self.registers.de(),
                self.registers.hl()
            );

            self.debug_update();
            self.print_debug_message();

            self.execute_instruction() as u32
        }
    }

    pub fn debug_update(&mut self) {
        if self.bus.mem_read(0xFF02) == 0x81 {
            let data = self.bus.mem_read(0xFF01);
            self.debug_message.push(data as char);
            self.bus.mem_write(0xFF02, 0)
        }
    }

    pub fn print_debug_message(&self) {
        if self.debug_message.len() != 0 {
            let message = self.debug_message.clone();
            println!("DEBUG: {message}")
        }
    }
}
