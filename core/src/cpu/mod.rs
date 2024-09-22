use std::fs::OpenOptions;
use std::io::Write;

use instructions::{arithmetic_logic, branch, load, miscellaneous, rotate_shift};
use interrupts::{Interrupts, IE_ADDRESS, IF_ADDRESS};

use crate::bus::{Bus, MemoryAccess};

use self::{instructions::Instruction, registers::Registers};

mod instructions;
mod interrupts;
mod operands;
pub mod registers;

pub const CPU_CLOCK_SPEED: u32 = 4194304;

pub struct Cpu {
    pub bus: Bus,
    registers: Registers,
    interrupts: Interrupts,
    current_opcode: u8,
    current_instruction: Instruction,
    halted: bool,
    debugging: bool,
    ticks: u32,
}

impl MemoryAccess for Cpu {
    fn read_8(&self, address: u16) -> u8 {
        self.bus.read_8(address)
    }

    fn write_8(&mut self, address: u16, data: u8) {
        self.bus.write_8(address, data)
    }
}

impl Cpu {
    pub fn new(bus: Bus, registers: Registers) -> Self {
        Cpu {
            bus,
            registers,
            interrupts: Interrupts::new(),
            current_opcode: 0x00,
            current_instruction: Instruction::None,
            halted: false,
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
        let interrupt_cycles = self.execute_interrupt() as u32;
        if interrupt_cycles != 0 {
            return interrupt_cycles;
        }

        if self.halted {
            return 4;
        }

        let pc = self.registers.pc;
        self.fetch_instruction();
        if self.debugging {
            self.log_cycle(pc)
        }
        self.execute_instruction() as u32
    }

    pub fn execute_interrupt(&mut self) -> u8 {
        self.interrupts.update_ime();
        if !self.interrupts.ime() && !self.halted {
            return 0;
        }

        let mut interrupt_flag = self.read_8(IF_ADDRESS);
        let interrupt_enable = self.read_8(IE_ADDRESS);
        let requested_interrupt = interrupt_flag & interrupt_enable;
        if requested_interrupt == 0 {
            return 0;
        }

        self.halted = false;
        if !self.interrupts.ime() {
            return 0;
        }
        self.interrupts.set_ime(false);
        let interrupt = requested_interrupt.trailing_zeros();
        if interrupt >= 5 {
            panic!("Invalid interrupt triggered");
        }

        interrupt_flag &= !(1 << interrupt);
        self.write_8(IF_ADDRESS, interrupt_flag);

        let address = self.registers.pc;
        self.push_stack(address);
        self.registers.pc = 0x0040 | ((interrupt as u16) << 3);
        16
    }

    fn fetch_instruction(&mut self) {
        self.current_opcode = self.read_8(self.registers.pc);
        self.registers.pc += 1;
        self.current_instruction = Instruction::from(self.current_opcode)
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.read_8(self.registers.pc);
        self.registers.pc += 1;
        byte
    }

    fn fetch_word(&mut self) -> u16 {
        let word = self.read_16(self.registers.pc);
        self.registers.pc += 2;
        word
    }

    fn pop_stack(&mut self) -> u16 {
        let data = self.read_16(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(2);
        data
    }

    fn push_stack(&mut self, data: u16) {
        self.registers.sp = self.registers.sp.wrapping_sub(2);
        self.write_16(self.registers.sp, data);
    }

    pub fn execute_instruction(&mut self) -> u8 {
        match self.current_instruction {
            Instruction::LdR16Imm16 => load::ld_r16_imm16(self),
            Instruction::LdR16MemA => load::ld_r16mem_a(self),
            Instruction::LdAR16Mem => load::ld_a_r16mem(self),
            Instruction::LdImm16Sp => load::ld_imm16_sp(self),
            Instruction::LdR8Imm8 => load::ld_r8_imm8(self),
            Instruction::LdR8R8 => load::ld_r8_r8(self),
            Instruction::LdhCMemA => load::ld_cmem_a(self),
            Instruction::LdhImm8MemA => load::ld_imm8mem_a(self),
            Instruction::LdImm16MemA => load::ld_imm16mem_a(self),
            Instruction::LdhACMem => load::ld_a_cmem(self),
            Instruction::LdhAImm8Mem => load::ld_a_imm8mem(self),
            Instruction::LdAImm16Mem => load::ld_a_imm16mem(self),
            Instruction::LdHlSpPlusImm8 => load::ld_hl_sp_plus_imm8(self),
            Instruction::LdSpHl => load::ld_sp_hl(self),
            Instruction::PopR16Stk => load::pop_r16_stk(self),
            Instruction::PushR16Stk => load::push_r16_stk(self),
            Instruction::IncR16 => arithmetic_logic::inc_r16(self),
            Instruction::IncR8 => arithmetic_logic::inc_r8(self),
            Instruction::DecR16 => arithmetic_logic::dec_r16(self),
            Instruction::DecR8 => arithmetic_logic::dec_r8(self),
            Instruction::Daa => miscellaneous::daa(self),
            Instruction::Cpl => miscellaneous::cpl(self),
            Instruction::Scf => miscellaneous::scf(self),
            Instruction::Ccf => miscellaneous::ccf(self),
            Instruction::AddHlR16 => arithmetic_logic::add_hl_r16(self),
            Instruction::AddSpImm8 => arithmetic_logic::add_sp_imm8(self),
            Instruction::AddAR8 => arithmetic_logic::add_a_r8(self),
            Instruction::AdcAR8 => arithmetic_logic::adc_a_r8(self),
            Instruction::SubAR8 => arithmetic_logic::sub_a_r8(self),
            Instruction::SbcAR8 => arithmetic_logic::sbc_a_r8(self),
            Instruction::AndAR8 => arithmetic_logic::and_a_r8(self),
            Instruction::XorAR8 => arithmetic_logic::xor_a_r8(self),
            Instruction::OrAR8 => arithmetic_logic::or_a_r8(self),
            Instruction::CpAR8 => arithmetic_logic::cp_a_r8(self),
            Instruction::AddAImm8 => arithmetic_logic::add_a_imm8(self),
            Instruction::AdcAImm8 => arithmetic_logic::adc_a_imm8(self),
            Instruction::SubAImm8 => arithmetic_logic::sub_a_imm8(self),
            Instruction::SbcAImm8 => arithmetic_logic::sbc_a_imm8(self),
            Instruction::AndAImm8 => arithmetic_logic::and_a_imm8(self),
            Instruction::XorAImm8 => arithmetic_logic::xor_a_imm8(self),
            Instruction::OrAImm8 => arithmetic_logic::or_a_imm8(self),
            Instruction::CpAImm8 => arithmetic_logic::cp_a_imm8(self),
            Instruction::Rlca => rotate_shift::rlca(self),
            Instruction::Rrca => rotate_shift::rrca(self),
            Instruction::Rla => rotate_shift::rla(self),
            Instruction::Rra => rotate_shift::rra(self),
            Instruction::JrImm8 => branch::jr_imm8(self),
            Instruction::JrCondImm8 => branch::jr_cond_imm8(self),
            Instruction::JpCondImm16 => branch::jp_cond_imm16(self),
            Instruction::JpImm16 => branch::jp_imm16(self),
            Instruction::JpHl => branch::jp_hl(self),
            Instruction::RetCond => branch::ret_cond(self),
            Instruction::Ret => branch::ret(self),
            Instruction::Reti => branch::reti(self),
            Instruction::CallCondImm16 => branch::call_cond_imm16(self),
            Instruction::CallImm16 => branch::call_imm16(self),
            Instruction::RstTgt3 => branch::rst_tgt3(self),
            Instruction::Stop => miscellaneous::stop(self),
            Instruction::Halt => miscellaneous::halt(self),
            Instruction::Prefix => miscellaneous::prefix(self),
            Instruction::Di => miscellaneous::di(self),
            Instruction::Ei => miscellaneous::ei(self),
            Instruction::Nop => 4,
            Instruction::None => panic!("Instruction not implemented"),
        }
    }

    fn log_cycle(&self, pc: u16) {
        let flags = format!(
            "{}{}{}{}",
            if u8::from(&self.registers.f) & (0b1000_0000) == 0b1000_0000 {
                'Z'
            } else {
                '-'
            },
            if u8::from(&self.registers.f) & (0b0100_0000) == 0b0100_0000 {
                'N'
            } else {
                '-'
            },
            if u8::from(&self.registers.f) & (0b0010_0000) == 0b0010_0000 {
                'H'
            } else {
                '-'
            },
            if u8::from(&self.registers.f) & (0b0001_0000) == 0b0001_0000 {
                'C'
            } else {
                '-'
            }
        );

        let log = format!(
            "{:#06X}: {:<16} ({:#04X} {:#04X} {:#04X}) A: {:#04X} F: {flags} BC: {:#06X} DE: {:#06X} HL: {:#06X} SP: {:#06X}\n",
            pc,
            &self.current_instruction.disassemble(self.current_opcode, self.read_8(pc + 1)),
            self.current_opcode,
            self.read_8(pc + 1),
            self.read_8(pc + 2),
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
