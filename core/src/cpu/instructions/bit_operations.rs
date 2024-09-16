use crate::cpu::{registers::CpuFlag, Cpu};

use super::R8;

pub fn bit_b3_r8(cpu: &mut Cpu, opcode: u8) -> u8 {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data = register.read_r8(cpu);
    let bit_index = (opcode & 0b0011_1000) >> 3;

    let result = data & (1 << (bit_index)) == 0;
    cpu.registers.f.set(CpuFlag::Z, result);
    cpu.registers.f.set(CpuFlag::N, false);
    cpu.registers.f.set(CpuFlag::H, true);
    if register == R8::HLMem {
        12
    } else {
        8
    }
}

pub fn res_b3_r8(cpu: &mut Cpu, opcode: u8) -> u8 {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data = register.read_r8(cpu);
    let bit_index = (opcode & 0b0011_1000) >> 3;
    register.write_r8(cpu, data & !(1 << bit_index));
    if register == R8::HLMem {
        16
    } else {
        8
    }
}

pub fn set_b3_r8(cpu: &mut Cpu, opcode: u8) -> u8 {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data = register.read_r8(cpu);
    let bit_index = (opcode & 0b0011_1000) >> 3;
    register.write_r8(cpu, data | (1 << bit_index));
    if register == R8::HLMem {
        16
    } else {
        8
    }
}
