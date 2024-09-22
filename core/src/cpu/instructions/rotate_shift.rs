use crate::cpu::Cpu;

use super::R8;

pub fn rlca(cpu: &mut Cpu) -> u8 {
    let carry = cpu.registers.a & 0x80 == 0x80;
    let result = (cpu.registers.a << 1) | (if carry { 1 } else { 0 });

    cpu.registers.f.zero = false;
    cpu.registers.f.subtraction = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = carry;

    cpu.registers.a = result;
    4
}

pub fn rrca(cpu: &mut Cpu) -> u8 {
    let carry = cpu.registers.a & 0x01 == 0x01;
    let result = (cpu.registers.a >> 1) | (if carry { 0x80 } else { 0 });

    cpu.registers.f.zero = false;
    cpu.registers.f.subtraction = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = carry;

    cpu.registers.a = result;
    4
}

pub fn rla(cpu: &mut Cpu) -> u8 {
    let carry = cpu.registers.a & 0x80 == 0x80;
    let result = (cpu.registers.a << 1) | (if cpu.registers.f.carry { 1 } else { 0 });

    cpu.registers.f.zero = false;
    cpu.registers.f.subtraction = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = carry;

    cpu.registers.a = result;
    4
}

pub fn rra(cpu: &mut Cpu) -> u8 {
    let carry = cpu.registers.a & 0x01 == 0x01;
    let result = (cpu.registers.a >> 1) | (if cpu.registers.f.carry { 0x80 } else { 0 });

    cpu.registers.f.zero = false;
    cpu.registers.f.subtraction = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = carry;

    cpu.registers.a = result;
    4
}

pub fn rlc_r8(cpu: &mut Cpu, opcode: u8) -> u8 {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data = register.read(cpu);
    let carry = data & 0x80 == 0x80;
    let result = (data << 1) | (if carry { 1 } else { 0 });
    register.write(cpu, result);
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
    let data = register.read(cpu);
    let carry = data & 0x01 == 0x01;
    let result = (data >> 1) | (if carry { 0x80 } else { 0 });
    register.write(cpu, result);
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
    let data = register.read(cpu);
    let carry = data & 0x80 == 0x80;
    let result = (data << 1) | (if cpu.registers.f.carry { 1 } else { 0 });
    register.write(cpu, result);
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
    let data = register.read(cpu);
    let carry = data & 0x01 == 0x01;
    let result = (data >> 1) | (if cpu.registers.f.carry { 0x80 } else { 0 });
    register.write(cpu, result);
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
    let data = register.read(cpu);
    let carry = data & 0x80 == 0x80;
    let result = data << 1;
    register.write(cpu, result);
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
    let data = register.read(cpu);
    let carry = data & 0x01 == 0x01;
    let result = (data >> 1) | (data & 0x80);
    register.write(cpu, result);
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
    let data = register.read(cpu);
    let result = (data >> 4) | (data << 4);
    register.write(cpu, result);

    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtraction = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = false;
    if register == R8::HLMem {
        16
    } else {
        8
    }
}

pub fn srl_r8(cpu: &mut Cpu, opcode: u8) -> u8 {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data = register.read(cpu);
    let carry = data & 0x01 == 0x01;
    let result = data >> 1;
    register.write(cpu, result);
    set_rotate_shift_flags(cpu, result, carry);
    if register == R8::HLMem {
        16
    } else {
        8
    }
}

pub fn set_rotate_shift_flags(cpu: &mut Cpu, result: u8, carry: bool) {
    cpu.registers.f.zero = result == 0;
    cpu.registers.f.subtraction = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = carry;
}
