use crate::{cpu::Cpu, memory::MemoryInterface};

use super::R8;

pub fn bit_b3_r8<I: MemoryInterface>(cpu: &mut Cpu<I>, opcode: u8) -> u8 {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let bit_index = (opcode & 0b0011_1000) >> 3;

    let result = value & (1 << (bit_index)) == 0;
    cpu.registers.f.zero = result;
    cpu.registers.f.subtraction = false;
    cpu.registers.f.half_carry = true;
    if register == R8::HLMem { 12 } else { 8 }
}

pub fn res_b3_r8<I: MemoryInterface>(cpu: &mut Cpu<I>, opcode: u8) -> u8 {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let bit_index = (opcode & 0b0011_1000) >> 3;
    register.write(cpu, value & !(1 << bit_index));
    if register == R8::HLMem { 16 } else { 8 }
}

pub fn set_b3_r8<I: MemoryInterface>(cpu: &mut Cpu<I>, opcode: u8) -> u8 {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let bit_index = (opcode & 0b0011_1000) >> 3;
    register.write(cpu, value | (1 << bit_index));
    if register == R8::HLMem { 16 } else { 8 }
}
