use getset::{CopyGetters, Getters, MutGetters, Setters};
use instructions::{arithmetic_logic, branch, load, miscellaneous, rotate_shift};
use tracing::debug;

use crate::{GbSpeed, cpu::interrupts::Interrupts, memory::MemoryInterface, t_cycles};

use self::{instructions::Instruction, registers::Registers};

mod instructions;
mod interrupts;
mod operands;
pub mod registers;

pub const CPU_CLOCK_SPEED: u32 = 4194304;

#[derive(Getters, MutGetters, CopyGetters, Setters)]
pub struct Cpu<I: MemoryInterface> {
    #[getset(get = "pub", get_mut = "pub")]
    pub bus: I,
    registers: Registers,
    #[getset(get = "pub", get_mut = "pub")]
    interupts: Interrupts,
    current_opcode: u8,
    current_instruction: Instruction,
    current_instruction_cycles: u8,
    halted: bool,
    halt_bug: bool,
    debugging: bool,
    total_cycles: u32,
}

impl<I: MemoryInterface> MemoryInterface for Cpu<I> {
    fn load_8(&mut self, address: u16) -> u8 {
        self.machine_cycle();
        self.bus.load_8(address)
    }

    fn store_8(&mut self, address: u16, value: u8) {
        self.machine_cycle();
        self.bus.store_8(address, value)
    }

    fn cycle(&mut self) {
        self.bus.cycle();
    }

    fn change_speed(&mut self) {
        self.bus.change_speed();
    }

    fn speed(&self) -> GbSpeed {
        self.bus.speed()
    }
}

impl<I: MemoryInterface> Cpu<I> {
    pub fn new(bus: I, registers: Registers) -> Self {
        Cpu {
            bus,
            registers,
            interupts: Interrupts::new(),
            current_opcode: 0x00,
            current_instruction: Instruction::Nop,
            current_instruction_cycles: 0,
            halted: false,
            halt_bug: false,
            debugging: true,
            total_cycles: 0,
        }
    }

    pub fn fetch_next_instruction(&mut self) {
        self.current_opcode = self.load_8(self.registers.pc());
        self.current_instruction = Instruction::from(self.current_opcode);
        self.registers.set_pc(self.registers.pc().wrapping_add(1));
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.load_8(self.registers.pc());
        self.registers.set_pc(self.registers.pc().wrapping_add(1));
        byte
    }

    fn fetch_word(&mut self) -> u16 {
        let word = self.load_16(self.registers.pc());
        self.registers.set_pc(self.registers.pc().wrapping_add(2));
        word
    }

    fn pop_stack(&mut self) -> u16 {
        // TODO: use this

        let value = self.load_16(self.registers.sp());
        self.registers.set_sp(self.registers.sp().wrapping_add(2));
        value
    }

    fn push_stack(&mut self, value: u16) {
        // TODO: not sure if this is cycle accurate
        self.machine_cycle();
        self.registers.set_sp(self.registers.sp().wrapping_sub(2));
        self.store_16(self.registers.sp(), value);
    }

    pub fn machine_cycle(&mut self) {
        self.current_instruction_cycles += t_cycles(self.speed());
        self.bus.cycle();
    }

    pub fn cycle(&mut self) {
        //TODO: hdma

        self.execute_instruction();

        let pc = self.registers.pc();
        if self.debugging {
            self.log_cycle(pc)
        }

        self.execute_interrupt();
        self.fetch_next_instruction();
    }

    fn execute_interrupt(&mut self) {
        if let Some(source_address) = self.interupts.handle_interrupt(&mut self.bus) {
            self.machine_cycle();
            self.machine_cycle();

            let address = self.registers.pc();
            self.push_stack(address);
            self.registers.set_pc(source_address);
        }
    }

    fn count_cycles(&mut self) {
        self.total_cycles += self.current_instruction_cycles as u32;
        self.current_instruction_cycles = 0;
    }

    fn handle_halt_bug(&mut self) {
        if !self.halted && self.halt_bug {
            self.registers.set_pc(self.registers.pc().wrapping_add(1));
            self.halt_bug = false;
        }
    }

    pub fn execute_instruction(&mut self) -> u8 {
        self.count_cycles();
        self.handle_halt_bug();
        self.interupts.update_interrupt_master_enable();

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
        }
    }

    pub fn registers(&mut self) -> &mut Registers {
        &mut self.registers
    }

    fn log_cycle(&mut self, pc: u16) {
        let flags = format!(
            "{}{}{}{}",
            if self.registers.f().zero() { 'Z' } else { '-' },
            if self.registers.f().subtraction() { 'N' } else { '-' },
            if self.registers.f().half_carry() { 'H' } else { '-' },
            if self.registers.f().carry() { 'C' } else { '-' }
        );

        let next_byte = self.load_8(pc + 1);
        let byte_after_next = self.load_8(pc + 2);
        let next_word = self.load_16(pc + 1);

        debug!(
            "{:<6}: {:#06X}: {:<20} ({:#04X} {:#04X} {:#04X}) A: {:#04X} F: {flags} BC: {:#06X} DE: {:#06X} HL: {:#06X} SP: {:#06X}",
            self.total_cycles,
            pc,
            &self.current_instruction.disassemble(self.current_opcode, next_byte, next_word),
            self.current_opcode,
            next_byte,
            byte_after_next,
            self.registers.a(),
            self.registers.bc(),
            self.registers.de(),
            self.registers.hl(),
            self.registers.sp(),
        );
    }
}
