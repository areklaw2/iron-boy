use getset::{CopyGetters, Getters, MutGetters, Setters};
use instructions::{arithmetic_logic, branch, load, miscellaneous, rotate_shift};
use tracing::debug;

use crate::{
    GbSpeed,
    interrupts::{IE_ADDRESS, IF_ADDRESS},
    memory::MemoryInterface,
    t_cycles,
};

use self::{instructions::Instruction, registers::Registers};

mod instructions;
mod operands;
pub mod registers;

pub const CPU_CLOCK_SPEED: u32 = 4194304;

#[derive(Getters, MutGetters, CopyGetters, Setters)]
pub struct Cpu<I: MemoryInterface> {
    #[getset(get = "pub", get_mut = "pub")]
    pub bus: I,
    registers: Registers,
    #[getset(get_copy = "pub", set = "pub")]
    interrupt_master_enable: bool,
    ei: u8,
    di: u8,
    current_opcode: u8,
    current_instruction: Instruction,
    current_instruction_cycles: u8,
    halted: bool,
    debugging: bool,
    total_cycles: u32,
}

impl<I: MemoryInterface> MemoryInterface for Cpu<I> {
    fn load_8(&self, address: u16) -> u8 {
        self.bus.load_8(address)
    }

    fn store_8(&mut self, address: u16, value: u8) {
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
            interrupt_master_enable: false,
            ei: 0,
            di: 0,
            current_opcode: 0x00,
            current_instruction: Instruction::None,
            current_instruction_cycles: 0,
            halted: false,
            debugging: true,
            total_cycles: 0,
        }
    }

    pub fn cycle(&mut self) -> u32 {
        self.cpu_cycle()
    }

    pub fn machine_cycle(&mut self) {
        self.current_instruction_cycles += t_cycles(self.speed());
        self.bus.cycle();
    }

    fn cpu_cycle(&mut self) -> u32 {
        let interrupt_cycles = self.execute_interrupt() as u32;
        if interrupt_cycles != 0 {
            return interrupt_cycles;
        }

        if self.halted {
            return 4;
        }

        let pc = self.registers.pc();
        self.fetch_instruction();
        if self.debugging {
            self.log_cycle(pc)
        }
        self.execute_instruction() as u32
    }

    fn execute_interrupt(&mut self) -> u8 {
        self.update_interrupt_master_enable();
        if !self.interrupt_master_enable && !self.halted {
            return 0;
        }

        let mut interrupt_flag = self.load_8(IF_ADDRESS);
        let interrupt_enable = self.load_8(IE_ADDRESS);
        let requested_interrupt = interrupt_flag & interrupt_enable;
        if requested_interrupt == 0 {
            return 0;
        }

        self.halted = false;
        if !self.interrupt_master_enable {
            return 0;
        }
        self.interrupt_master_enable = false;
        let interrupt = requested_interrupt.trailing_zeros();
        if interrupt >= 5 {
            panic!("Invalid interrupt triggered");
        }

        interrupt_flag &= !(1 << interrupt);
        self.store_8(IF_ADDRESS, interrupt_flag);

        let address = self.registers.pc();
        self.push_stack(address);
        self.registers.set_pc(0x0040 | ((interrupt as u16) << 3));
        16
    }

    fn update_interrupt_master_enable(&mut self) {
        if self.di == 1 {
            self.interrupt_master_enable = false;
        }
        self.di = self.di.saturating_sub(1);

        if self.ei == 1 {
            self.interrupt_master_enable = true;
        }
        self.ei = self.ei.saturating_sub(1);
    }

    pub fn set_ei(&mut self) {
        self.ei = 2
    }

    pub fn set_di(&mut self) {
        self.di = 2
    }

    pub fn fetch_instruction(&mut self) {
        self.current_opcode = self.load_8(self.registers.pc());
        self.registers.set_pc(self.registers.pc().wrapping_add(1));
        self.current_instruction = Instruction::from(self.current_opcode)
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
        let value = self.load_16(self.registers.sp());
        self.registers.set_sp(self.registers.sp().wrapping_add(2));
        value
    }

    fn push_stack(&mut self, value: u16) {
        self.registers.set_sp(self.registers.sp().wrapping_sub(2));
        self.store_16(self.registers.sp(), value);
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
