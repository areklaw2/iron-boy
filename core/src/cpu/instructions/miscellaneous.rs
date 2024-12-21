use crate::{cpu::Cpu, memory::MemoryInterface};

use super::{bit_operations, rotate_shift};

pub fn daa<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let mut a = cpu.registers.a;
    let mut correction = if cpu.registers.f.carry { 0x60 } else { 0x00 };

    if cpu.registers.f.half_carry {
        correction |= 0x06;
    }

    if !cpu.registers.f.subtraction {
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

    cpu.registers.f.zero = a == 0;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = correction >= 0x60;
    cpu.registers.a = a;
    4
}

pub fn cpl<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    cpu.registers.a = !cpu.registers.a;
    cpu.registers.f.subtraction = true;
    cpu.registers.f.half_carry = true;
    4
}

pub fn scf<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    cpu.registers.f.carry = true;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.subtraction = false;
    4
}

pub fn ccf<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let carry = !cpu.registers.f.carry;
    cpu.registers.f.carry = carry;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.subtraction = false;
    4
}

pub fn stop<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    cpu.bus.change_speed();
    4
}

pub fn halt<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    cpu.halted = true;
    4
}

pub fn di<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    cpu.interrupts.set_di(); //cpu cycles
    4
}

pub fn ei<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    cpu.interrupts.set_ei(); //cpu cycles
    4
}

pub fn prefix<I: MemoryInterface>(cpu: &mut Cpu<I>) -> u8 {
    let opcode = cpu.fetch_byte();
    let operation = (opcode & 0b1100_0000) >> 6;
    match operation {
        0b01 => bit_operations::bit_b3_r8(cpu, opcode),
        0b10 => bit_operations::res_b3_r8(cpu, opcode),
        0b11 => bit_operations::set_b3_r8(cpu, opcode),
        0b00 => {
            let operation = (opcode & 0b0011_1000) >> 3;
            match operation {
                0b000 => rotate_shift::rlc_r8(cpu, opcode),
                0b001 => rotate_shift::rrc_r8(cpu, opcode),
                0b010 => rotate_shift::rl_r8(cpu, opcode),
                0b011 => rotate_shift::rr_r8(cpu, opcode),
                0b100 => rotate_shift::sla_r8(cpu, opcode),
                0b101 => rotate_shift::sra_r8(cpu, opcode),
                0b110 => rotate_shift::swap_r8(cpu, opcode),
                0b111 => rotate_shift::srl_r8(cpu, opcode),
                _ => panic!("No operation exists"),
            }
        }
        _ => panic!("No operation exists"),
    }
}
