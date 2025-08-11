use crate::{cpu::Cpu, memory::MemoryInterface};

use super::{R16Memory, R16Stack, R16, R8};

pub fn ld_r16_imm16<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let destination = (cpu.current_opcode & 0b0011_0000) >> 4;
    let value = cpu.fetch_word();
    R16::from(destination).store(cpu, value);
    12
}

pub fn ld_r16mem_a<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let destination = (cpu.current_opcode & 0b0011_0000) >> 4;
    let address = R16Memory::from(destination).load(cpu);
    cpu.store_8(address, cpu.registers.a);
    8
}

pub fn ld_a_r16mem<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let source = (cpu.current_opcode & 0b0011_0000) >> 4;
    let address = R16Memory::from(source).load(cpu);
    cpu.registers.a = cpu.load_8(address);
    8
}

pub fn ld_imm16_sp<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let address = cpu.fetch_word();
    cpu.store_16(address, cpu.registers.sp);
    20
}

pub fn ld_r8_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let destination = (cpu.current_opcode & 0b0011_1000) >> 3;
    let value = cpu.fetch_byte();
    let register = R8::from(destination);
    register.write(cpu, value);
    if register == R8::HLMem {
        12
    } else {
        8
    }
}

pub fn ld_r8_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let destination = (cpu.current_opcode & 0b0011_1000) >> 3;
    let source = cpu.current_opcode & 0b0000_0111;
    let register1 = R8::from(destination);
    let register2 = R8::from(source);

    let value = register2.load(cpu);
    register1.write(cpu, value);
    if register1 == R8::HLMem || register2 == R8::HLMem {
        8
    } else {
        4
    }
}

pub fn ld_cmem_a<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let address = 0xFF00 | cpu.registers.c as u16;
    cpu.store_8(address, cpu.registers.a);
    8
}

pub fn ld_imm8mem_a<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let address = 0xFF00 | cpu.fetch_byte() as u16;
    cpu.store_8(address, cpu.registers.a);
    12
}

pub fn ld_imm16mem_a<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let address = cpu.fetch_word();
    cpu.store_8(address, cpu.registers.a);
    16
}

pub fn ld_a_cmem<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let address = 0xFF00 | cpu.registers.c as u16;
    cpu.registers.a = cpu.load_8(address);
    8
}

pub fn ld_a_imm8mem<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let address = 0xFF00 | cpu.fetch_byte() as u16;
    cpu.registers.a = cpu.load_8(address);
    12
}

pub fn ld_a_imm16mem<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let address = cpu.fetch_word();
    cpu.registers.a = cpu.load_8(address);
    16
}

pub fn ld_hl_sp_plus_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let value1 = cpu.registers.sp;
    let value2 = cpu.fetch_byte() as i8 as i16 as u16;
    let result = value1.wrapping_add(value2);
    cpu.registers.set_hl(result);

    cpu.registers.f.zero = false;
    cpu.registers.f.subtraction = false;
    cpu.registers.f.half_carry = (value1 & 0x000F) + (value2 & 0x000F) > 0x000F;
    cpu.registers.f.carry = (value1 & 0x00FF) + (value2 & 0x00FF) > 0x00FF;
    12
}

pub fn ld_sp_hl<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    cpu.registers.sp = cpu.registers.hl();
    8
}

pub fn pop_r16_stk<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let value = cpu.pop_stack();
    let register = (cpu.current_opcode & 0b0011_0000) >> 4;
    R16Stack::from(register).store(cpu, value);
    12
}

pub fn push_r16_stk<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let register = (cpu.current_opcode & 0b0011_0000) >> 4;
    let value = R16Stack::from(register).load(cpu);
    cpu.push_stack(value);
    16
}
