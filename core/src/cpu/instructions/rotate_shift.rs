use crate::cpu::{registers::CpuFlag, Cpu};

use super::R8;

pub fn rlca(cpu: &mut Cpu) -> u8 {
    let carry = cpu.registers.a & 0x80 == 0x80;
    let result = (cpu.registers.a << 1) | (if carry { 1 } else { 0 });

    cpu.registers.f.set(CpuFlag::Z, false);
    cpu.registers.f.set(CpuFlag::N, false);
    cpu.registers.f.set(CpuFlag::H, false);
    cpu.registers.f.set(CpuFlag::C, carry);

    cpu.registers.a = result;
    4
}

pub fn rrca(cpu: &mut Cpu) -> u8 {
    let carry = cpu.registers.a & 0x01 == 0x01;
    let result = (cpu.registers.a >> 1) | (if carry { 0x80 } else { 0 });

    cpu.registers.f.set(CpuFlag::Z, false);
    cpu.registers.f.set(CpuFlag::N, false);
    cpu.registers.f.set(CpuFlag::H, false);
    cpu.registers.f.set(CpuFlag::C, carry);

    cpu.registers.a = result;
    4
}

pub fn rla(cpu: &mut Cpu) -> u8 {
    let carry = cpu.registers.a & 0x80 == 0x80;
    let result = (cpu.registers.a << 1) | (if cpu.registers.f.contains(CpuFlag::C) { 1 } else { 0 });

    cpu.registers.f.set(CpuFlag::Z, false);
    cpu.registers.f.set(CpuFlag::N, false);
    cpu.registers.f.set(CpuFlag::H, false);
    cpu.registers.f.set(CpuFlag::C, carry);

    cpu.registers.a = result;
    4
}

pub fn rra(cpu: &mut Cpu) -> u8 {
    let carry = cpu.registers.a & 0x01 == 0x01;
    let result = (cpu.registers.a >> 1) | (if cpu.registers.f.contains(CpuFlag::C) { 0x80 } else { 0 });

    cpu.registers.f.set(CpuFlag::Z, false);
    cpu.registers.f.set(CpuFlag::N, false);
    cpu.registers.f.set(CpuFlag::H, false);
    cpu.registers.f.set(CpuFlag::C, carry);

    cpu.registers.a = result;
    4
}

pub fn rlc_r8(cpu: &mut Cpu, opcode: u8) -> u8 {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data = cpu.read_r8(&register);
    let carry = data & 0x80 == 0x80;
    let result = (data << 1) | (if carry { 1 } else { 0 });
    cpu.write_r8(&register, result);
    set_rotate_shift_flags(cpu, result, carry);
    if register == R8::HLMem {
        16
    } else {
        8
    }
}

pub fn rrc_r8(cpu: &mut Cpu, opcode: u8) -> u8 {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data = cpu.read_r8(&register);
    let carry = data & 0x01 == 0x01;
    let result = (data >> 1) | (if carry { 0x80 } else { 0 });
    cpu.write_r8(&register, result);
    set_rotate_shift_flags(cpu, result, carry);
    if register == R8::HLMem {
        16
    } else {
        8
    }
}

pub fn rl_r8(cpu: &mut Cpu, opcode: u8) -> u8 {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data = cpu.read_r8(&register);
    let carry = data & 0x80 == 0x80;
    let result = (data << 1) | (if cpu.registers.f.contains(CpuFlag::C) { 1 } else { 0 });
    cpu.write_r8(&register, result);
    set_rotate_shift_flags(cpu, result, carry);
    if register == R8::HLMem {
        16
    } else {
        8
    }
}

pub fn rr_r8(cpu: &mut Cpu, opcode: u8) -> u8 {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data = cpu.read_r8(&register);
    let carry = data & 0x01 == 0x01;
    let result = (data >> 1) | (if cpu.registers.f.contains(CpuFlag::C) { 0x80 } else { 0 });
    cpu.write_r8(&register, result);
    set_rotate_shift_flags(cpu, result, carry);
    if register == R8::HLMem {
        16
    } else {
        8
    }
}

pub fn sla_r8(cpu: &mut Cpu, opcode: u8) -> u8 {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data = cpu.read_r8(&register);
    let carry = data & 0x80 == 0x80;
    let result = data << 1;
    cpu.write_r8(&register, result);
    set_rotate_shift_flags(cpu, result, carry);
    if register == R8::HLMem {
        16
    } else {
        8
    }
}

pub fn sra_r8(cpu: &mut Cpu, opcode: u8) -> u8 {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data = cpu.read_r8(&register);
    let carry = data & 0x01 == 0x01;
    let result = (data >> 1) | (data & 0x80);
    cpu.write_r8(&register, result);
    set_rotate_shift_flags(cpu, result, carry);
    if register == R8::HLMem {
        16
    } else {
        8
    }
}

pub fn swap_r8(cpu: &mut Cpu, opcode: u8) -> u8 {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data = cpu.read_r8(&register);
    let result = (data >> 4) | (data << 4);
    cpu.write_r8(&register, result);

    cpu.registers.f.set(CpuFlag::Z, result == 0);
    cpu.registers.f.set(CpuFlag::N, false);
    cpu.registers.f.set(CpuFlag::H, false);
    cpu.registers.f.set(CpuFlag::C, false);
    if register == R8::HLMem {
        16
    } else {
        8
    }
}

pub fn srl_r8(cpu: &mut Cpu, opcode: u8) -> u8 {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data = cpu.read_r8(&register);
    let carry = data & 0x01 == 0x01;
    let result = data >> 1;
    cpu.write_r8(&register, result);
    set_rotate_shift_flags(cpu, result, carry);
    if register == R8::HLMem {
        16
    } else {
        8
    }
}

pub fn set_rotate_shift_flags(cpu: &mut Cpu, result: u8, carry: bool) {
    cpu.registers.f.set(CpuFlag::Z, result == 0);
    cpu.registers.f.set(CpuFlag::N, false);
    cpu.registers.f.set(CpuFlag::H, false);
    cpu.registers.f.set(CpuFlag::C, carry);
}
