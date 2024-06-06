use crate::bus::Memory;

use super::{
    instructions::{Condition, Instruction, R16Memory, R16Stack, R16, R8},
    registers::CpuFlag,
    Cpu,
};

impl Cpu {
    pub fn execute_instruction(&mut self) -> u8 {
        match self.current_instruction {
            Instruction::LdR16Imm16 => self.ld_r16_imm16(),
            Instruction::LdR16MemA => self.ld_r16mem_a(),
            Instruction::LdAR16Mem => self.ld_a_r16mem(),
            Instruction::LdImm16Sp => self.ld_imm16_sp(),
            Instruction::LdR8Imm8 => self.ld_r8_imm8(),
            Instruction::LdR8R8 => self.ld_r8_r8(),
            Instruction::LdhCMemA => self.ld_cmem_a(),
            Instruction::LdhImm8MemA => self.ld_imm8mem_a(),
            Instruction::LdImm16MemA => self.ld_imm16mem_a(),
            Instruction::LdhACMem => self.ld_a_cmem(),
            Instruction::LdhAImm8Mem => self.ld_a_imm8mem(),
            Instruction::LdAImm16Mem => self.ld_a_imm16mem(),
            Instruction::LdHlSpPlusImm8 => self.ld_hl_sp_plus_imm8(),
            Instruction::LdSpHl => self.ld_sp_hl(),
            Instruction::PopR16Stk => self.pop_r16_stk(),
            Instruction::PushR16Stk => self.push_r16_stk(),
            Instruction::IncR16 => self.inc_r16(),
            Instruction::IncR8 => self.inc_r8(),
            Instruction::DecR16 => self.dec_r16(),
            Instruction::DecR8 => self.dec_r8(),
            Instruction::Daa => self.daa(),
            Instruction::Cpl => self.cpl(),
            Instruction::Scf => self.scf(),
            Instruction::Ccf => self.ccf(),
            Instruction::AddHlR16 => self.add_hl_r16(),
            Instruction::AddSpImm8 => self.add_sp_imm8(),
            Instruction::AddAR8 => self.add_a_r8(),
            Instruction::AdcAR8 => self.adc_a_r8(),
            Instruction::SubAR8 => self.sub_a_r8(),
            Instruction::SbcAR8 => self.sbc_a_r8(),
            Instruction::AndAR8 => self.and_a_r8(),
            Instruction::XorAR8 => self.xor_a_r8(),
            Instruction::OrAR8 => self.or_a_r8(),
            Instruction::CpAR8 => self.cp_a_r8(),
            Instruction::AddAImm8 => self.add_a_imm8(),
            Instruction::AdcAImm8 => self.adc_a_imm8(),
            Instruction::SubAImm8 => self.sub_a_imm8(),
            Instruction::SbcAImm8 => self.sbc_a_imm8(),
            Instruction::AndAImm8 => self.and_a_imm8(),
            Instruction::XorAImm8 => self.xor_a_imm8(),
            Instruction::OrAImm8 => self.or_a_imm8(),
            Instruction::CpAImm8 => self.cp_a_imm8(),
            Instruction::Rlca => self.rlca(),
            Instruction::Rrca => self.rrca(),
            Instruction::Rla => self.rla(),
            Instruction::Rra => self.rra(),
            Instruction::JrImm8 => self.jr_imm8(),
            Instruction::JrCondImm8 => self.jr_cond_imm8(),
            Instruction::JpCondImm16 => self.jp_cond_imm16(),
            Instruction::JpImm16 => self.jp_imm16(),
            Instruction::JpHl => self.jp_hl(),
            Instruction::RetCond => self.ret_cond(),
            Instruction::Ret => self.ret(),
            Instruction::Reti => self.reti(),
            Instruction::CallCondImm16 => self.call_cond_imm16(),
            Instruction::CallImm16 => self.call_imm16(),
            Instruction::RstTgt3 => self.rst_tgt3(),
            Instruction::Nop => 4,
            Instruction::Stop => self.stop(),
            Instruction::Halt => self.halt(),
            Instruction::Prefix => self.prefix() + 4,
            Instruction::Di => self.di(),
            Instruction::Ei => self.ei(),
            Instruction::None => panic!("Instruction not implemented"),
        }
    }

    fn ld_r16_imm16(&mut self) -> u8 {
        let destination = (self.current_opcode & 0b0011_0000) >> 4;
        let data = self.fetch_word();
        self.reg_write_16(&R16::get_register(destination), data);
        12
    }

    fn ld_r16mem_a(&mut self) -> u8 {
        let destination = (self.current_opcode & 0b0011_0000) >> 4;
        let address = self.memory_reg_read_16(&R16Memory::get_register(destination));
        self.bus.mem_write(address, self.registers.a);
        8
    }

    fn ld_a_r16mem(&mut self) -> u8 {
        let source = (self.current_opcode & 0b0011_0000) >> 4;
        let address = self.memory_reg_read_16(&R16Memory::get_register(source));
        self.registers.a = self.bus.mem_read(address);
        8
    }

    fn ld_imm16_sp(&mut self) -> u8 {
        let address = self.fetch_word();
        self.bus.mem_write_16(address, self.registers.sp);
        20
    }

    fn ld_r8_imm8(&mut self) -> u8 {
        let destination = (self.current_opcode & 0b0011_1000) >> 3;
        let data = self.fetch_byte();
        let register = R8::get_register(destination);
        self.reg_write_8(&register, data);
        if register == R8::HLMem {
            12
        } else {
            8
        }
    }

    fn ld_r8_r8(&mut self) -> u8 {
        let destination = (self.current_opcode & 0b0011_1000) >> 3;
        let source = self.current_opcode & 0b0000_0111;
        let register1 = R8::get_register(destination);
        let register2 = R8::get_register(source);

        let data = self.reg_read_8(&register2);
        self.reg_write_8(&register1, data);
        if register1 == R8::HLMem || register2 == R8::HLMem {
            8
        } else {
            4
        }
    }

    fn ld_cmem_a(&mut self) -> u8 {
        let address = 0xFF00 | self.registers.c as u16;
        self.bus.mem_write(address, self.registers.a);
        8
    }

    fn ld_imm8mem_a(&mut self) -> u8 {
        let address = 0xFF00 | self.fetch_byte() as u16;
        self.bus.mem_write(address, self.registers.a);
        12
    }

    fn ld_imm16mem_a(&mut self) -> u8 {
        let address = self.fetch_word();
        self.bus.mem_write(address, self.registers.a);
        16
    }

    fn ld_a_cmem(&mut self) -> u8 {
        let address = 0xFF00 | self.registers.c as u16;
        self.registers.a = self.bus.mem_read(address);
        8
    }

    fn ld_a_imm8mem(&mut self) -> u8 {
        let address = 0xFF00 | self.fetch_byte() as u16;
        self.registers.a = self.bus.mem_read(address);
        12
    }

    fn ld_a_imm16mem(&mut self) -> u8 {
        let address = self.fetch_word();
        self.registers.a = self.bus.mem_read(address);
        16
    }

    fn ld_hl_sp_plus_imm8(&mut self) -> u8 {
        let data1 = self.registers.sp;
        let data2 = self.fetch_byte() as i8 as i16 as u16;
        let result = data1.wrapping_add(data2);
        self.registers.set_hl(result);

        self.registers.set_flag(CpuFlag::Z, false);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, (data1 & 0x000F) + (data2 & 0x000F) > 0x000F);
        self.registers.set_flag(CpuFlag::C, (data1 & 0x00FF) + (data2 & 0x00FF) > 0x00FF);
        12
    }

    fn ld_sp_hl(&mut self) -> u8 {
        self.registers.sp = self.registers.hl();
        8
    }

    fn pop_r16_stk(&mut self) -> u8 {
        let data = self.pop_stack();
        let register = (self.current_opcode & 0b0011_0000) >> 4;
        self.stack_reg_write_16(&R16Stack::get_register(register), data);
        12
    }

    fn push_r16_stk(&mut self) -> u8 {
        let register = (self.current_opcode & 0b0011_0000) >> 4;
        let data = self.stack_reg_read_16(&R16Stack::get_register(register));
        self.push_stack(data);
        16
    }

    fn inc_r16(&mut self) -> u8 {
        let operand = (self.current_opcode & 0b0011_0000) >> 4;
        let register = R16::get_register(operand);
        let data = self.reg_read_16(&register).wrapping_add(1);
        self.reg_write_16(&register, data);
        8
    }

    fn inc_r8(&mut self) -> u8 {
        let operand = (self.current_opcode & 0b0011_1000) >> 3;
        let register = R8::get_register(operand);
        let data = self.reg_read_8(&register);
        let result = data.wrapping_add(1);
        self.reg_write_8(&register, result);

        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, (data & 0x0F) + 1 > 0x0F);
        if register == R8::HLMem {
            12
        } else {
            4
        }
    }

    fn dec_r16(&mut self) -> u8 {
        let operand = (self.current_opcode & 0b0011_0000) >> 4;
        let register = R16::get_register(operand);
        let data = self.reg_read_16(&register).wrapping_sub(1);
        self.reg_write_16(&register, data);
        8
    }

    fn dec_r8(&mut self) -> u8 {
        let operand = (self.current_opcode & 0b0011_1000) >> 3;
        let register = R8::get_register(operand);
        let data = self.reg_read_8(&register);
        let result = data.wrapping_sub(1);
        self.reg_write_8(&register, result);

        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, true);
        self.registers.set_flag(CpuFlag::H, (data & 0x0F) == 0);
        if register == R8::HLMem {
            12
        } else {
            4
        }
    }

    fn daa(&mut self) -> u8 {
        let mut a = self.registers.a;
        let mut correction = if self.registers.f.contains(CpuFlag::C) { 0x60 } else { 0x00 };

        if self.registers.f.contains(CpuFlag::H) {
            correction |= 0x06;
        }

        if !self.registers.f.contains(CpuFlag::N) {
            if a & 0x0F > 0x09 {
                correction |= 0x06;
            };
            if a > 0x99 {
                correction |= 0x60;
            }
            a = a.wrapping_add(correction);
        } else {
            a = a.wrapping_sub(correction);
        }

        self.registers.set_flag(CpuFlag::Z, a == 0);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, correction >= 0x60);
        self.registers.a = a;
        4
    }

    fn cpl(&mut self) -> u8 {
        self.registers.a = !self.registers.a;
        self.registers.set_flag(CpuFlag::N, true);
        self.registers.set_flag(CpuFlag::H, true);
        4
    }

    fn scf(&mut self) -> u8 {
        self.registers.set_flag(CpuFlag::C, true);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::N, false);
        4
    }

    fn ccf(&mut self) -> u8 {
        let carry = !self.registers.f.contains(CpuFlag::C);
        self.registers.set_flag(CpuFlag::C, carry);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::N, false);
        4
    }

    fn add_hl_r16(&mut self) -> u8 {
        let data1 = self.registers.hl();
        let operand = (self.current_opcode & 0b0011_0000) >> 4;
        let data2 = self.reg_read_16(&R16::get_register(operand));
        let result = data1.wrapping_add(data2);

        self.registers.set_hl(result);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, (data1 & 0x07FF) + (data2 & 0x07FF) > 0x07FF);
        self.registers.set_flag(CpuFlag::C, data1 as u32 + data2 as u32 > 0xFFFF);
        8
    }

    fn add_sp_imm8(&mut self) -> u8 {
        let data1 = self.registers.sp;
        let data2 = self.fetch_byte() as i8 as i16 as u16;
        let result = data1.wrapping_add(data2);
        self.registers.sp = result;

        self.registers.set_flag(CpuFlag::Z, false);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, (data1 & 0x000F) + (data2 & 0x000F) > 0x000F);
        self.registers.set_flag(CpuFlag::C, (data1 & 0x00FF) + (data2 & 0x00FF) > 0x00FF);
        16
    }

    fn add_a_r8(&mut self) -> u8 {
        let data1 = self.registers.a;
        let operand = self.current_opcode & 0b0000_0111;
        let register = R8::get_register(operand);
        let data2 = self.reg_read_8(&register);
        let result = data1.wrapping_add(data2);
        self.registers.a = result;

        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, (data1 & 0x0F) + (data2 & 0x0F) > 0x0F);
        self.registers.set_flag(CpuFlag::C, data1 as u16 + data2 as u16 > 0xFF);
        if register == R8::HLMem {
            8
        } else {
            4
        }
    }

    fn adc_a_r8(&mut self) -> u8 {
        let data1 = self.registers.a;
        let operand = self.current_opcode & 0b0000_0111;
        let register = R8::get_register(operand);
        let data2 = self.reg_read_8(&register);
        let carry = if self.registers.f.contains(CpuFlag::C) { 1 } else { 0 };
        let result = data1.wrapping_add(data2).wrapping_add(carry);
        self.registers.a = result;

        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, (data1 & 0x0F) + (data2 & 0x0F) + carry > 0x0F);
        self.registers.set_flag(CpuFlag::C, data1 as u16 + data2 as u16 + carry as u16 > 0xFF);
        if register == R8::HLMem {
            8
        } else {
            4
        }
    }

    fn sub_a_r8(&mut self) -> u8 {
        let data1 = self.registers.a;
        let operand = self.current_opcode & 0b0000_0111;
        let register = R8::get_register(operand);
        let data2 = self.reg_read_8(&register);
        let result = data1.wrapping_sub(data2);
        self.registers.a = result;

        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, true);
        self.registers.set_flag(CpuFlag::H, (data1 & 0x0F) < (data2 & 0x0F));
        self.registers.set_flag(CpuFlag::C, (data1 as u16) < (data2 as u16));
        if register == R8::HLMem {
            8
        } else {
            4
        }
    }

    fn sbc_a_r8(&mut self) -> u8 {
        let data1 = self.registers.a;
        let operand = self.current_opcode & 0b0000_0111;
        let register = R8::get_register(operand);
        let data2 = self.reg_read_8(&register);
        let carry = if self.registers.f.contains(CpuFlag::C) { 1 } else { 0 };
        let result = data1.wrapping_sub(data2).wrapping_sub(carry);
        self.registers.a = result;

        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, true);
        self.registers.set_flag(CpuFlag::H, (data1 & 0x0F) < (data2 & 0x0F) + carry);
        self.registers.set_flag(CpuFlag::C, (data1 as u16) < (data2 as u16) + carry as u16);
        if register == R8::HLMem {
            8
        } else {
            4
        }
    }

    fn and_a_r8(&mut self) -> u8 {
        let operand = self.current_opcode & 0b0000_0111;
        let register = R8::get_register(operand);
        let data = self.reg_read_8(&register);
        let result = self.registers.a & data;
        self.registers.a = result;

        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, true);
        self.registers.set_flag(CpuFlag::C, false);
        if register == R8::HLMem {
            8
        } else {
            4
        }
    }

    fn xor_a_r8(&mut self) -> u8 {
        let operand = self.current_opcode & 0b0000_0111;
        let register = R8::get_register(operand);
        let data = self.reg_read_8(&register);
        let result = self.registers.a ^ data;
        self.registers.a = result;

        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, false);
        if register == R8::HLMem {
            8
        } else {
            4
        }
    }

    fn or_a_r8(&mut self) -> u8 {
        let operand = self.current_opcode & 0b0000_0111;
        let register = R8::get_register(operand);
        let data = self.reg_read_8(&register);
        let result = self.registers.a | data;
        self.registers.a = result;

        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, false);
        if register == R8::HLMem {
            8
        } else {
            4
        }
    }

    fn cp_a_r8(&mut self) -> u8 {
        let data1 = self.registers.a;
        let operand = self.current_opcode & 0b0000_0111;
        let register = R8::get_register(operand);
        let data2 = self.reg_read_8(&register);
        let result = data1.wrapping_sub(data2);

        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, true);
        self.registers.set_flag(CpuFlag::H, (data1 & 0x0F) < (data2 & 0x0F));
        self.registers.set_flag(CpuFlag::C, (data1 as u16) < (data2 as u16));
        if register == R8::HLMem {
            8
        } else {
            4
        }
    }

    fn add_a_imm8(&mut self) -> u8 {
        let data1 = self.registers.a;
        let data2 = self.fetch_byte();
        let result = data1.wrapping_add(data2);
        self.registers.a = result;

        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, (data1 as u8 & 0x0F) + (data2 as u8 & 0x0F) > 0x0F);
        self.registers.set_flag(CpuFlag::C, data1 as u16 + data2 as u16 > 0xFF);
        8
    }

    fn adc_a_imm8(&mut self) -> u8 {
        let data1 = self.registers.a;
        let data2 = self.fetch_byte();
        let carry = if self.registers.f.contains(CpuFlag::C) { 1 } else { 0 };
        let result = data1.wrapping_add(data2).wrapping_add(carry);
        self.registers.a = result;

        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, (data1 & 0x0F) + (data2 & 0x0F) + carry > 0x0F);
        self.registers.set_flag(CpuFlag::C, data1 as u16 + data2 as u16 + carry as u16 > 0xFF);
        8
    }

    fn sub_a_imm8(&mut self) -> u8 {
        let data1 = self.registers.a;
        let data2 = self.fetch_byte();
        let result = data1.wrapping_sub(data2);
        self.registers.a = result;

        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, true);
        self.registers.set_flag(CpuFlag::H, (data1 & 0x0F) < (data2 & 0x0F));
        self.registers.set_flag(CpuFlag::C, (data1 as u16) < (data2 as u16));
        8
    }

    fn sbc_a_imm8(&mut self) -> u8 {
        let data1 = self.registers.a;
        let data2 = self.fetch_byte();
        let carry = if self.registers.f.contains(CpuFlag::C) { 1 } else { 0 };
        let result = data1.wrapping_sub(data2).wrapping_sub(carry);
        self.registers.a = result;

        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, true);
        self.registers.set_flag(CpuFlag::H, (data1 & 0x0F) < (data2 & 0x0F) + carry);
        self.registers.set_flag(CpuFlag::C, (data1 as u16) < (data2 as u16) + carry as u16);
        8
    }

    fn and_a_imm8(&mut self) -> u8 {
        let data = self.fetch_byte();
        let result = self.registers.a & data;
        self.registers.a = result;

        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, true);
        self.registers.set_flag(CpuFlag::C, false);
        8
    }

    fn xor_a_imm8(&mut self) -> u8 {
        let data = self.fetch_byte();
        let result = self.registers.a ^ data;
        self.registers.a = result;

        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, false);
        8
    }

    fn or_a_imm8(&mut self) -> u8 {
        let data = self.fetch_byte();
        let result = self.registers.a | data;
        self.registers.a = result;

        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, false);
        8
    }

    fn cp_a_imm8(&mut self) -> u8 {
        let data1 = self.registers.a;
        let data2 = self.fetch_byte();
        let result = data1.wrapping_sub(data2);

        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, true);
        self.registers.set_flag(CpuFlag::H, (data1 & 0x0F) < (data2 & 0x0F));
        self.registers.set_flag(CpuFlag::C, (data1 as u16) < (data2 as u16));
        8
    }

    fn rlca(&mut self) -> u8 {
        let carry = self.registers.a & 0x80 == 0x80;
        let result = (self.registers.a << 1) | (if carry { 1 } else { 0 });

        self.registers.set_flag(CpuFlag::Z, false);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, carry);

        self.registers.a = result;
        4
    }

    fn rrca(&mut self) -> u8 {
        let carry = self.registers.a & 0x01 == 0x01;
        let result = (self.registers.a >> 1) | (if carry { 0x80 } else { 0 });

        self.registers.set_flag(CpuFlag::Z, false);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, carry);

        self.registers.a = result;
        4
    }

    fn rla(&mut self) -> u8 {
        let carry = self.registers.a & 0x80 == 0x80;
        let result = (self.registers.a << 1) | (if self.registers.f.contains(CpuFlag::C) { 1 } else { 0 });

        self.registers.set_flag(CpuFlag::Z, false);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, carry);

        self.registers.a = result;
        4
    }

    fn rra(&mut self) -> u8 {
        let carry = self.registers.a & 0x01 == 0x01;
        let result = (self.registers.a >> 1) | (if self.registers.f.contains(CpuFlag::C) { 0x80 } else { 0 });

        self.registers.set_flag(CpuFlag::Z, false);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, carry);

        self.registers.a = result;
        4
    }

    fn jr_imm8(&mut self) -> u8 {
        let signed = self.fetch_byte() as i8;
        self.registers.pc = ((self.registers.pc as i16) + (signed as i16)) as u16;
        12
    }

    fn jr_cond_imm8(&mut self) -> u8 {
        let z = self.registers.f.contains(CpuFlag::Z);
        let c = self.registers.f.contains(CpuFlag::C);

        let cond = (self.current_opcode & 0b0001_1000) >> 3;
        let jump = match Condition::get_condtion(cond) {
            Condition::NC => c == false,
            Condition::C => c == true,
            Condition::NZ => z == false,
            Condition::Z => z == true,
        };

        if jump {
            let signed = self.fetch_byte() as i8;
            self.registers.pc = ((self.registers.pc as i16) + (signed as i16)) as u16;
            12
        } else {
            self.registers.pc += 1;
            8
        }
    }

    fn jp_cond_imm16(&mut self) -> u8 {
        let z = self.registers.f.contains(CpuFlag::Z);
        let c = self.registers.f.contains(CpuFlag::C);

        let cond = (self.current_opcode & 0b0001_1000) >> 3;
        let jump = match Condition::get_condtion(cond) {
            Condition::NC => c == false,
            Condition::C => c == true,
            Condition::NZ => z == false,
            Condition::Z => z == true,
        };

        if jump {
            self.registers.pc = self.fetch_word();
            16
        } else {
            self.registers.pc += 2;
            12
        }
    }

    fn jp_imm16(&mut self) -> u8 {
        self.registers.pc = self.fetch_word();
        16
    }

    fn jp_hl(&mut self) -> u8 {
        self.registers.pc = self.registers.hl();
        4
    }

    fn ret_cond(&mut self) -> u8 {
        let z = self.registers.f.contains(CpuFlag::Z);
        let c = self.registers.f.contains(CpuFlag::C);

        let cond = (self.current_opcode & 0b0001_1000) >> 3;
        let ret = match Condition::get_condtion(cond) {
            Condition::NC => c == false,
            Condition::C => c == true,
            Condition::NZ => z == false,
            Condition::Z => z == true,
        };

        if ret {
            self.registers.pc = self.pop_stack();
            20
        } else {
            8
        }
    }

    fn ret(&mut self) -> u8 {
        self.registers.pc = self.pop_stack();
        16
    }

    fn reti(&mut self) -> u8 {
        self.registers.pc = self.pop_stack();
        self.interrupt_master_enable = true;
        16
    }

    fn call_cond_imm16(&mut self) -> u8 {
        let z = self.registers.f.contains(CpuFlag::Z);
        let c = self.registers.f.contains(CpuFlag::C);

        let cond = (self.current_opcode & 0b0001_1000) >> 3;

        let call = match Condition::get_condtion(cond) {
            Condition::NC => c == false,
            Condition::C => c == true,
            Condition::NZ => z == false,
            Condition::Z => z == true,
        };

        if call {
            self.push_stack(self.registers.pc + 2);
            self.registers.pc = self.fetch_word();
            24
        } else {
            self.registers.pc += 2;
            12
        }
    }

    fn call_imm16(&mut self) -> u8 {
        self.push_stack(self.registers.pc + 2);
        self.registers.pc = self.fetch_word();
        24
    }

    fn rst_tgt3(&mut self) -> u8 {
        self.push_stack(self.registers.pc);
        let target = ((self.current_opcode & 0b0011_1000) >> 3) / 8;
        self.registers.pc = target as u16;
        16
    }

    fn stop(&mut self) -> u8 {
        //Stop not used in DMG"//
        4
    }

    fn halt(&mut self) -> u8 {
        self.halted = true;
        4
    }

    fn di(&mut self) -> u8 {
        self.interrupt_master_enable = false;
        self.enabling_interrupts = false;
        4
    }

    fn ei(&mut self) -> u8 {
        self.interrupt_master_enable = true;
        self.enabling_interrupts = true;
        4
    }

    fn prefix(&mut self) -> u8 {
        let opcode = self.fetch_byte();
        let operation = (opcode & 0b1100_0000) >> 6;
        match operation {
            0b01 => self.bit_b3_r8(opcode),
            0b10 => self.res_b3_r8(opcode),
            0b11 => self.set_b3_r8(opcode),
            0b00 => {
                let operation = (opcode & 0b0011_1000) >> 3;
                match operation {
                    0b000 => self.rlc_r8(opcode),
                    0b001 => self.rrc_r8(opcode),
                    0b010 => self.rl_r8(opcode),
                    0b011 => self.rr_r8(opcode),
                    0b100 => self.sla_r8(opcode),
                    0b101 => self.sra_r8(opcode),
                    0b110 => self.swap_r8(opcode),
                    0b111 => self.srl_r8(opcode),
                    _ => panic!("No operation exists"),
                }
            }
            _ => panic!("No operation exists"),
        }
    }

    fn bit_b3_r8(&mut self, opcode: u8) -> u8 {
        let operand = opcode & 0b0000_0111;
        let register = R8::get_register(operand);
        let data = self.reg_read_8(&register);
        let bit_index = (opcode & 0b0011_1000) >> 3;

        let result = data & (1 << (bit_index)) == 0;
        self.registers.set_flag(CpuFlag::Z, result);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, true);
        if register == R8::HLMem {
            12
        } else {
            8
        }
    }

    fn res_b3_r8(&mut self, opcode: u8) -> u8 {
        let operand = opcode & 0b0000_0111;
        let register = R8::get_register(operand);
        let data = self.reg_read_8(&register);
        let bit_index = (opcode & 0b0011_1000) >> 3;
        self.reg_write_8(&register, data & !(1 << bit_index));
        if register == R8::HLMem {
            16
        } else {
            8
        }
    }

    fn set_b3_r8(&mut self, opcode: u8) -> u8 {
        let operand = opcode & 0b0000_0111;
        let register = R8::get_register(operand);
        let data = self.reg_read_8(&register);
        let bit_index = (opcode & 0b0011_1000) >> 3;
        self.reg_write_8(&register, data | (1 << bit_index));
        if register == R8::HLMem {
            16
        } else {
            8
        }
    }

    fn rlc_r8(&mut self, opcode: u8) -> u8 {
        let operand = opcode & 0b0000_0111;
        let register = R8::get_register(operand);
        let data = self.reg_read_8(&register);
        let carry = data & 0x80 == 0x80;
        let result = (data << 1) | (if carry { 1 } else { 0 });
        self.reg_write_8(&register, result);
        self.set_rotate_shift_flags(result, carry);
        if register == R8::HLMem {
            16
        } else {
            8
        }
    }

    fn rrc_r8(&mut self, opcode: u8) -> u8 {
        let operand = opcode & 0b0000_0111;
        let register = R8::get_register(operand);
        let data = self.reg_read_8(&register);
        let carry = data & 0x01 == 0x01;
        let result = (data >> 1) | (if carry { 0x80 } else { 0 });
        self.reg_write_8(&register, result);
        self.set_rotate_shift_flags(result, carry);
        if register == R8::HLMem {
            16
        } else {
            8
        }
    }

    fn rl_r8(&mut self, opcode: u8) -> u8 {
        let operand = opcode & 0b0000_0111;
        let register = R8::get_register(operand);
        let data = self.reg_read_8(&register);
        let carry = data & 0x80 == 0x80;
        let result = (data << 1) | (if self.registers.f.contains(CpuFlag::C) { 1 } else { 0 });
        self.reg_write_8(&register, result);
        self.set_rotate_shift_flags(result, carry);
        if register == R8::HLMem {
            16
        } else {
            8
        }
    }

    fn rr_r8(&mut self, opcode: u8) -> u8 {
        let operand = opcode & 0b0000_0111;
        let register = R8::get_register(operand);
        let data = self.reg_read_8(&register);
        let carry = data & 0x01 == 0x01;
        let result = (data >> 1) | (if self.registers.f.contains(CpuFlag::C) { 0x80 } else { 0 });
        self.reg_write_8(&register, result);
        self.set_rotate_shift_flags(result, carry);
        if register == R8::HLMem {
            16
        } else {
            8
        }
    }

    fn sla_r8(&mut self, opcode: u8) -> u8 {
        let operand = opcode & 0b0000_0111;
        let register = R8::get_register(operand);
        let data = self.reg_read_8(&register);
        let carry = data & 0x80 == 0x80;
        let result = data << 1;
        self.reg_write_8(&register, result);
        self.set_rotate_shift_flags(result, carry);
        if register == R8::HLMem {
            16
        } else {
            8
        }
    }

    fn sra_r8(&mut self, opcode: u8) -> u8 {
        let operand = opcode & 0b0000_0111;
        let register = R8::get_register(operand);
        let data = self.reg_read_8(&register);
        let carry = data & 0x01 == 0x01;
        let result = (data >> 1) | (data & 0x80);
        self.reg_write_8(&register, result);
        self.set_rotate_shift_flags(result, carry);
        if register == R8::HLMem {
            16
        } else {
            8
        }
    }

    fn swap_r8(&mut self, opcode: u8) -> u8 {
        let operand = opcode & 0b0000_0111;
        let register = R8::get_register(operand);
        let data = self.reg_read_8(&register);
        let result = (data >> 4) | (data << 4);
        self.reg_write_8(&register, result);

        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, false);
        if register == R8::HLMem {
            16
        } else {
            8
        }
    }

    fn srl_r8(&mut self, opcode: u8) -> u8 {
        let operand = opcode & 0b0000_0111;
        let register = R8::get_register(operand);
        let data = self.reg_read_8(&register);
        let carry = data & 0x01 == 0x01;
        let result = data >> 1;
        self.reg_write_8(&register, result);
        self.set_rotate_shift_flags(result, carry);
        if register == R8::HLMem {
            16
        } else {
            8
        }
    }

    fn set_rotate_shift_flags(&mut self, result: u8, carry: bool) {
        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, carry);
    }
}
