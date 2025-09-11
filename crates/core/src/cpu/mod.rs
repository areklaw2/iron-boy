use getset::{CopyGetters, Getters, MutGetters, Setters};
use instructions::{arithmetic_logic, branch, load, miscellaneous, rotate_shift};
use tracing::debug;

use crate::{GbMode, MCycle, MCycleKind, cpu::interrupts::Interrupts, memory::MemoryInterface};

use self::{instructions::Instruction, registers::Registers};

pub mod instructions;
mod interrupts;
mod operands;
mod registers;

pub const CPU_CLOCK_SPEED: u32 = 4194304;

#[derive(Getters, MutGetters, CopyGetters, Setters)]
pub struct Cpu<I: MemoryInterface> {
    #[getset(get = "pub", get_mut = "pub")]
    pub bus: I,
    registers: Registers,
    #[getset(get = "pub", get_mut = "pub")]
    interrupts: Interrupts,
    current_opcode: u8,
    #[getset(get = "pub")]
    current_instruction: Instruction,
    halted: bool,
    halt_bug: bool,
    #[getset(get = "pub")]
    cycles: Vec<MCycle>,
    testing: bool,
    debugging: bool,
}

impl<I: MemoryInterface> Cpu<I> {
    pub fn new(bus: I, mode: GbMode) -> Self {
        Cpu {
            bus,
            registers: Registers::new(mode),
            interrupts: Interrupts::new(),
            current_opcode: 0x00,
            current_instruction: Instruction::Nop,
            halted: false,
            halt_bug: false,
            cycles: Vec::new(),
            testing: false,
            debugging: false,
        }
    }

    fn read_byte(&mut self, address: u16) -> u8 {
        self.m_cycle(MCycleKind::MemoryRead);
        self.bus.load_8(address)
    }

    fn read_word(&mut self, address: u16) -> u16 {
        let lo = self.read_byte(address) as u16;
        let hi = self.read_byte(address + 1) as u16;
        hi << 8 | lo
    }

    fn write_byte(&mut self, address: u16, value: u8) {
        self.m_cycle(MCycleKind::MemoryWrite);
        self.bus.store_8(address, value)
    }

    fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value & 0xFF) as u8);
        self.write_byte(address + 1, (value >> 8) as u8);
    }

    pub fn fetch_instruction(&mut self) {
        self.current_opcode = self.read_byte(self.registers.pc());
        self.current_instruction = Instruction::from(self.current_opcode);
        self.registers.set_pc(self.registers.pc().wrapping_add(1));
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.read_byte(self.registers.pc());
        self.registers.set_pc(self.registers.pc().wrapping_add(1));
        byte
    }

    fn fetch_word(&mut self) -> u16 {
        let word = self.read_word(self.registers.pc());
        self.registers.set_pc(self.registers.pc().wrapping_add(2));
        word
    }

    fn pop_stack(&mut self) -> u16 {
        let value = self.read_word(self.registers.sp());
        self.registers.set_sp(self.registers.sp().wrapping_add(2));
        value
    }

    fn push_stack(&mut self, value: u16) {
        self.registers.set_sp(self.registers.sp().wrapping_sub(2));
        self.write_word(self.registers.sp(), value);
        self.m_cycle(MCycleKind::Idle);
    }

    pub fn m_cycle(&mut self, kind: MCycleKind) {
        if self.testing {
            self.record_cycle(kind);
        }
        self.bus.cycle();
    }

    pub fn cycle(&mut self) {
        //TODO: hdma

        match self.halted {
            false => {
                self.execute_instruction();
                if self.debugging {
                    self.log_cycle(self.registers.pc());
                }
                self.execute_interrupt();
                self.fetch_instruction();
            }
            true => {
                if self.interrupts.pending_interrupt(&self.bus) {
                    self.halted = false;
                    if !self.interrupts.interrupt_master_enable() {
                        self.halt_bug = true;
                    }
                } else {
                    self.m_cycle(MCycleKind::Idle);
                }
                self.execute_interrupt();
            }
        }
    }

    fn execute_interrupt(&mut self) {
        if let Some(source_address) = self.interrupts.handle_interrupt(&mut self.bus) {
            self.m_cycle(MCycleKind::Idle);
            self.m_cycle(MCycleKind::Idle);

            let address = self.registers.pc();
            self.push_stack(address);
            self.registers.set_pc(source_address);
        }
    }

    fn handle_halt_bug(&mut self) {
        if !self.halted && self.halt_bug {
            self.registers.set_pc(self.registers.pc().wrapping_sub(1));
            self.halt_bug = false;
        }
    }

    pub fn execute_instruction(&mut self) {
        self.handle_halt_bug();
        self.interrupts.update_interrupt_master_enable();

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
            Instruction::LdHlSpPlusSignedImm8 => load::ld_hl_sp_plus_signed_imm8(self),
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
            Instruction::AddSpSignedImm8 => arithmetic_logic::add_sp_signed_imm8(self),
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
            Instruction::JrSignedImm8 => branch::jr_signed_imm8(self),
            Instruction::JrCondSignedImm8 => branch::jr_cond_signed_imm8(self),
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
            Instruction::Nop => {}
        }
    }

    pub fn registers(&mut self) -> &mut Registers {
        &mut self.registers
    }

    pub fn enable_testing_mode(&mut self) {
        self.testing = true;
        self.cycles.clear();
    }

    pub fn record_cycle(&mut self, kind: MCycleKind) {
        let pc = self.registers().pc();
        self.cycles.push((self.registers.pc(), self.bus.load_8(pc), kind));
    }

    fn log_cycle(&mut self, pc: u16) {
        let byte0 = self.bus.load_8(pc);
        let byte1 = self.bus.load_8(pc.wrapping_add(1));
        let byte2 = self.bus.load_8(pc.wrapping_add(2));
        let byte3 = self.bus.load_8(pc.wrapping_add(3));

        let log_line = format!(
            "A:{:02X} F:{:02X} B:{:02X} C:{:02X} D:{:02X} E:{:02X} H:{:02X} L:{:02X} SP:{:04X} PC:{:04X} PCMEM:{:02X},{:02X},{:02X},{:02X}",
            self.registers.a(),
            u8::from(&self.registers.f()),
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
