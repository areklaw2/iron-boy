use std::{cell::RefCell, rc::Rc};

use getset::{CopyGetters, Getters, MutGetters, Setters};
use instructions::{arithmetic_logic, branch, load, miscellaneous, rotate_shift};
use tracing::debug;

use crate::{GbMode, cpu::instructions::Instruction, interrupts::InterruptKind};

use self::registers::Registers;

pub mod instructions;
mod operands;
mod registers;

pub const CPU_CLOCK_SPEED: u32 = 4194304;

#[derive(Getters, MutGetters, CopyGetters, Setters)]
pub struct Cpu<I: MemoryInterface> {
    #[getset(get = "pub", get_mut = "pub")]
    pub bus: I,
    #[getset(get = "pub", get_mut = "pub")]
    registers: Registers,
    interrupt_master_enable: bool,
    ei: u8,
    di: u8,
    halted: Rc<RefCell<bool>>,
    halt_bug: bool,
    current_opcode: u8,
    #[getset(get = "pub")]
    current_instruction: Instruction,
    debugging: bool,
    disassemble: bool,
}

impl<I: MemoryInterface> Cpu<I> {
    pub fn new(bus: I, mode: GbMode, halted: Rc<RefCell<bool>>) -> Self {
        Cpu {
            bus,
            registers: Registers::new(mode),
            interrupt_master_enable: false,
            ei: 0,
            di: 0,
            halted,
            halt_bug: false,
            current_opcode: 0x00,
            current_instruction: Instruction::Nop,
            //TODO: add flags for this to the
            debugging: false,
            disassemble: false,
        }
    }

    pub fn cycle(&mut self) {
        let halted = *self.halted.borrow();
        if !halted {
            self.update_interrupt_master_enable();
            self.execute_instruction();
            self.log_cycle(self.registers.pc());
        } else {
            self.bus.m_cycle();
        }

        self.execute_interrupt();

        if !halted {
            self.fetch_instruction();
        }
    }

    pub fn fetch_instruction(&mut self) {
        self.current_opcode = self.bus.load_8(self.registers.pc(), true);
        self.current_instruction = Instruction::from(self.current_opcode);
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
        if self.di == 1 {
            self.interrupt_master_enable = false;
        }
        self.di = self.di.saturating_sub(1);

        if self.ei == 1 {
            self.interrupt_master_enable = true;
        }
        self.ei = self.ei.saturating_sub(1);
    }

    pub fn execute_instruction(&mut self) {
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

    fn log_cycle(&mut self, pc: u16) {
        if !self.debugging {
            return;
        }

        let disassemble = match self.disassemble {
            true => {
                let next_byte = self.bus.load_8(pc.wrapping_add(1), false);
                let next_word = self.bus.load_16(pc.wrapping_add(1), false);
                format!(
                    "{:<20} ->: ",
                    self.current_instruction.disassemble(self.current_opcode, next_byte, next_word)
                )
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
