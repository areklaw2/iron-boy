use crate::Cpu;

use ironboy_common::MemoryInterface;

use super::{R8, R16};

pub fn add_a_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let value1 = cpu.registers.a;
    let operand = cpu.current_opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value2 = register.load(cpu);
    let result = value1.wrapping_add(value2);
    cpu.registers.a = result;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtraction = false;
    cpu.registers.f.half_carry = (value1 & 0x0F) + (value2 & 0x0F) > 0x0F;
    cpu.registers.f.carry = value1 as u16 + value2 as u16 > 0xFF;
    if register == R8::HLMem { 8 } else { 4 }
}

pub fn add_a_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let value1 = cpu.registers.a;
    let value2 = cpu.fetch_byte();
    let result = value1.wrapping_add(value2);
    cpu.registers.a = result;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtraction = false;
    cpu.registers.f.half_carry = (value1 as u8 & 0x0F) + (value2 as u8 & 0x0F) > 0x0F;
    cpu.registers.f.carry = value1 as u16 + value2 as u16 > 0xFF;
    8
}

pub fn add_hl_r16<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let value1 = cpu.registers.hl();
    let operand = (cpu.current_opcode & 0b0011_0000) >> 4;
    let value2 = R16::from(operand).load(cpu);
    let result = value1.wrapping_add(value2);

    cpu.registers.set_hl(result);
    cpu.registers.f.subtraction = false;
    cpu.registers.f.half_carry = (value1 & 0x0FFF) + (value2 & 0x0FFF) > 0x0FFF;
    cpu.registers.f.carry = value1 as u32 + value2 as u32 > 0xFFFF;
    8
}

pub fn add_sp_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let value1 = cpu.registers.sp;
    let value2 = cpu.fetch_byte() as i8 as i16 as u16;
    let result = value1.wrapping_add(value2);
    cpu.registers.sp = result;

    cpu.registers.f.zero = false;
    cpu.registers.f.subtraction = false;
    cpu.registers.f.half_carry = (value1 & 0x000F) + (value2 & 0x000F) > 0x000F;
    cpu.registers.f.carry = (value1 & 0x00FF) + (value2 & 0x00FF) > 0x00FF;
    16
}

pub fn adc_a_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let value1 = cpu.registers.a;
    let operand = cpu.current_opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value2 = register.load(cpu);
    let carry = if cpu.registers.f.carry { 1 } else { 0 };
    let result = value1.wrapping_add(value2).wrapping_add(carry);
    cpu.registers.a = result;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtraction = false;
    cpu.registers.f.half_carry = (value1 & 0x0F) + (value2 & 0x0F) + carry > 0x0F;
    cpu.registers.f.carry = value1 as u16 + value2 as u16 + carry as u16 > 0xFF;
    if register == R8::HLMem { 8 } else { 4 }
}

pub fn adc_a_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let value1 = cpu.registers.a;
    let value2 = cpu.fetch_byte();
    let carry = if cpu.registers.f.carry { 1 } else { 0 };
    let result = value1.wrapping_add(value2).wrapping_add(carry);
    cpu.registers.a = result;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtraction = false;
    cpu.registers.f.half_carry = (value1 & 0x0F) + (value2 & 0x0F) + carry > 0x0F;
    cpu.registers.f.carry = value1 as u16 + value2 as u16 + carry as u16 > 0xFF;
    8
}

pub fn sub_a_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let value1 = cpu.registers.a;
    let operand = cpu.current_opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value2 = register.load(cpu);
    let result = value1.wrapping_sub(value2);
    cpu.registers.a = result;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtraction = true;
    cpu.registers.f.half_carry = (value1 & 0x0F) < (value2 & 0x0F);
    cpu.registers.f.carry = (value1 as u16) < (value2 as u16);
    if register == R8::HLMem { 8 } else { 4 }
}

pub fn sub_a_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let value1 = cpu.registers.a;
    let value2 = cpu.fetch_byte();
    let result = value1.wrapping_sub(value2);
    cpu.registers.a = result;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtraction = true;
    cpu.registers.f.half_carry = (value1 & 0x0F) < (value2 & 0x0F);
    cpu.registers.f.carry = (value1 as u16) < (value2 as u16);
    8
}

pub fn sbc_a_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let value1 = cpu.registers.a;
    let operand = cpu.current_opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value2 = register.load(cpu);
    let carry = if cpu.registers.f.carry { 1 } else { 0 };
    let result = value1.wrapping_sub(value2).wrapping_sub(carry);
    cpu.registers.a = result;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtraction = true;
    cpu.registers.f.half_carry = (value1 & 0x0F) < (value2 & 0x0F) + carry;
    cpu.registers.f.carry = (value1 as u16) < (value2 as u16) + carry as u16;
    if register == R8::HLMem { 8 } else { 4 }
}

pub fn sbc_a_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let value1 = cpu.registers.a;
    let value2 = cpu.fetch_byte();
    let carry = if cpu.registers.f.carry { 1 } else { 0 };
    let result = value1.wrapping_sub(value2).wrapping_sub(carry);
    cpu.registers.a = result;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtraction = true;
    cpu.registers.f.half_carry = (value1 & 0x0F) < (value2 & 0x0F) + carry;
    cpu.registers.f.carry = (value1 as u16) < (value2 as u16) + carry as u16;
    8
}

pub fn and_a_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let operand = cpu.current_opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let result = cpu.registers.a & value;
    cpu.registers.a = result;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtraction = false;
    cpu.registers.f.half_carry = true;
    cpu.registers.f.carry = false;
    if register == R8::HLMem { 8 } else { 4 }
}

pub fn and_a_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let value = cpu.fetch_byte();
    let result = cpu.registers.a & value;
    cpu.registers.a = result;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtraction = false;
    cpu.registers.f.half_carry = true;
    cpu.registers.f.carry = false;
    8
}

pub fn xor_a_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let operand = cpu.current_opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let result = cpu.registers.a ^ value;
    cpu.registers.a = result;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtraction = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;
    if register == R8::HLMem { 8 } else { 4 }
}

pub fn xor_a_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let value = cpu.fetch_byte();
    let result = cpu.registers.a ^ value;
    cpu.registers.a = result;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtraction = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;
    8
}

pub fn or_a_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let operand = cpu.current_opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let result = cpu.registers.a | value;
    cpu.registers.a = result;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtraction = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;
    if register == R8::HLMem { 8 } else { 4 }
}

pub fn or_a_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let value = cpu.fetch_byte();
    let result = cpu.registers.a | value;
    cpu.registers.a = result;

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtraction = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;
    8
}

pub fn cp_a_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let value1 = cpu.registers.a;
    let operand = cpu.current_opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value2 = register.load(cpu);
    let result = value1.wrapping_sub(value2);

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtraction = true;
    cpu.registers.f.half_carry = (value1 & 0x0F) < (value2 & 0x0F);
    cpu.registers.f.carry = (value1 as u16) < (value2 as u16);
    if register == R8::HLMem { 8 } else { 4 }
}

pub fn cp_a_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let value1 = cpu.registers.a;
    let value2 = cpu.fetch_byte();
    let result = value1.wrapping_sub(value2);

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtraction = true;
    cpu.registers.f.half_carry = (value1 & 0x0F) < (value2 & 0x0F);
    cpu.registers.f.carry = (value1 as u16) < (value2 as u16);
    8
}

pub fn inc_r16<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let operand = (cpu.current_opcode & 0b0011_0000) >> 4;
    let register = R16::from(operand);
    let value = register.load(cpu).wrapping_add(1);
    register.store(cpu, value);
    8
}

pub fn inc_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let operand = (cpu.current_opcode & 0b0011_1000) >> 3;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let result = value.wrapping_add(1);
    register.write(cpu, result);

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtraction = false;
    cpu.registers.f.half_carry = (value & 0x0F) + 1 > 0x0F;
    if register == R8::HLMem { 12 } else { 4 }
}

pub fn dec_r16<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let operand = (cpu.current_opcode & 0b0011_0000) >> 4;
    let register = R16::from(operand);
    let value = register.load(cpu).wrapping_sub(1);
    register.store(cpu, value);
    8
}

pub fn dec_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let operand = (cpu.current_opcode & 0b0011_1000) >> 3;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let result = value.wrapping_sub(1);
    register.write(cpu, result);

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtraction = true;
    cpu.registers.f.half_carry = (value & 0x0F) == 0;
    if register == R8::HLMem { 12 } else { 4 }
}
