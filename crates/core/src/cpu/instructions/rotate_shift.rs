use crate::{cpu::Cpu, memory::MemoryInterface};

use super::R8;

pub fn rlca<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let carry = cpu.registers.a() & 0x80 == 0x80;
    let result = (cpu.registers.a() << 1) | (if carry { 1 } else { 0 });

    cpu.registers.f().set_zero(false);
    cpu.registers.f().set_subtraction(false);
    cpu.registers.f().set_half_carry(false);
    cpu.registers.f().set_carry(carry);

    cpu.registers.set_a(result);
}

pub fn rrca<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let carry = cpu.registers.a() & 0x01 == 0x01;
    let result = (cpu.registers.a() >> 1) | (if carry { 0x80 } else { 0 });

    cpu.registers.f().set_zero(false);
    cpu.registers.f().set_subtraction(false);
    cpu.registers.f().set_half_carry(false);
    cpu.registers.f().set_carry(carry);

    cpu.registers.set_a(result);
}

pub fn rla<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let carry = cpu.registers.a() & 0x80 == 0x80;
    let result = (cpu.registers.a() << 1) | (if cpu.registers.f().carry() { 1 } else { 0 });

    cpu.registers.f().set_zero(false);
    cpu.registers.f().set_subtraction(false);
    cpu.registers.f().set_half_carry(false);
    cpu.registers.f().set_carry(carry);

    cpu.registers.set_a(result);
}

pub fn rra<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let carry = cpu.registers.a() & 0x01 == 0x01;
    let result = (cpu.registers.a() >> 1) | (if cpu.registers.f().carry() { 0x80 } else { 0 });

    cpu.registers.f().set_zero(false);
    cpu.registers.f().set_subtraction(false);
    cpu.registers.f().set_half_carry(false);
    cpu.registers.f().set_carry(carry);

    cpu.registers.set_a(result);
}

pub fn rlc_r8<I: MemoryInterface>(cpu: &mut Cpu<I>, opcode: u8) {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let carry = value & 0x80 == 0x80;
    let result = (value << 1) | (if carry { 1 } else { 0 });
    register.write(cpu, result);
    set_rotate_shift_flags(cpu, result, carry);
}

pub fn rrc_r8<I: MemoryInterface>(cpu: &mut Cpu<I>, opcode: u8) {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let carry = value & 0x01 == 0x01;
    let result = (value >> 1) | (if carry { 0x80 } else { 0 });
    register.write(cpu, result);
    set_rotate_shift_flags(cpu, result, carry);
}

pub fn rl_r8<I: MemoryInterface>(cpu: &mut Cpu<I>, opcode: u8) {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let carry = value & 0x80 == 0x80;
    let result = (value << 1) | (if cpu.registers.f().carry() { 1 } else { 0 });
    register.write(cpu, result);
    set_rotate_shift_flags(cpu, result, carry);
}

pub fn rr_r8<I: MemoryInterface>(cpu: &mut Cpu<I>, opcode: u8) {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let carry = value & 0x01 == 0x01;
    let result = (value >> 1) | (if cpu.registers.f().carry() { 0x80 } else { 0 });
    register.write(cpu, result);
    set_rotate_shift_flags(cpu, result, carry);
}

pub fn sla_r8<I: MemoryInterface>(cpu: &mut Cpu<I>, opcode: u8) {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let carry = value & 0x80 == 0x80;
    let result = value << 1;
    register.write(cpu, result);
    set_rotate_shift_flags(cpu, result, carry);
}

pub fn sra_r8<I: MemoryInterface>(cpu: &mut Cpu<I>, opcode: u8) {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let carry = value & 0x01 == 0x01;
    let result = (value >> 1) | (value & 0x80);
    register.write(cpu, result);
    set_rotate_shift_flags(cpu, result, carry);
}

pub fn swap_r8<I: MemoryInterface>(cpu: &mut Cpu<I>, opcode: u8) {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let result = (value >> 4) | (value << 4);
    register.write(cpu, result);

    cpu.registers.f().set_zero(result == 0);
    cpu.registers.f().set_subtraction(false);
    cpu.registers.f().set_half_carry(false);
    cpu.registers.f().set_carry(false);
}

pub fn srl_r8<I: MemoryInterface>(cpu: &mut Cpu<I>, opcode: u8) {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let carry = value & 0x01 == 0x01;
    let result = value >> 1;
    register.write(cpu, result);
    set_rotate_shift_flags(cpu, result, carry);
}

pub fn set_rotate_shift_flags<I: MemoryInterface>(cpu: &mut Cpu<I>, result: u8, carry: bool) {
    cpu.registers.f().set_zero(result == 0);
    cpu.registers.f().set_subtraction(false);
    cpu.registers.f().set_half_carry(false);
    cpu.registers.f().set_carry(carry);
}
