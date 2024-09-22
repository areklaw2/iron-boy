use crate::{bus::MemoryAccess, cpu::Cpu};

use super::{R16Memory, R16Stack, R16, R8};

pub fn ld_r16_imm16(cpu: &mut Cpu) -> u8 {
    let destination = (cpu.current_opcode & 0b0011_0000) >> 4;
    let data = cpu.fetch_word();
    R16::from(destination).write(cpu, data);
    12
}

pub fn ld_r16mem_a(cpu: &mut Cpu) -> u8 {
    let destination = (cpu.current_opcode & 0b0011_0000) >> 4;
    let address = R16Memory::from(destination).read(cpu);
    cpu.write_8(address, cpu.registers.a);
    8
}

pub fn ld_a_r16mem(cpu: &mut Cpu) -> u8 {
    let source = (cpu.current_opcode & 0b0011_0000) >> 4;
    let address = R16Memory::from(source).read(cpu);
    cpu.registers.a = cpu.read_8(address);
    8
}

pub fn ld_imm16_sp(cpu: &mut Cpu) -> u8 {
    let address = cpu.fetch_word();
    cpu.write_16(address, cpu.registers.sp);
    20
}

pub fn ld_r8_imm8(cpu: &mut Cpu) -> u8 {
    let destination = (cpu.current_opcode & 0b0011_1000) >> 3;
    let data = cpu.fetch_byte();
    let register = R8::from(destination);
    register.write(cpu, data);
    if register == R8::HLMem {
        12
    } else {
        8
    }
}

pub fn ld_r8_r8(cpu: &mut Cpu) -> u8 {
    let destination = (cpu.current_opcode & 0b0011_1000) >> 3;
    let source = cpu.current_opcode & 0b0000_0111;
    let register1 = R8::from(destination);
    let register2 = R8::from(source);

    let data = register2.read(cpu);
    register1.write(cpu, data);
    if register1 == R8::HLMem || register2 == R8::HLMem {
        8
    } else {
        4
    }
}

pub fn ld_cmem_a(cpu: &mut Cpu) -> u8 {
    let address = 0xFF00 | cpu.registers.c as u16;
    cpu.write_8(address, cpu.registers.a);
    8
}

pub fn ld_imm8mem_a(cpu: &mut Cpu) -> u8 {
    let address = 0xFF00 | cpu.fetch_byte() as u16;
    cpu.write_8(address, cpu.registers.a);
    12
}

pub fn ld_imm16mem_a(cpu: &mut Cpu) -> u8 {
    let address = cpu.fetch_word();
    cpu.write_8(address, cpu.registers.a);
    16
}

pub fn ld_a_cmem(cpu: &mut Cpu) -> u8 {
    let address = 0xFF00 | cpu.registers.c as u16;
    cpu.registers.a = cpu.read_8(address);
    8
}

pub fn ld_a_imm8mem(cpu: &mut Cpu) -> u8 {
    let address = 0xFF00 | cpu.fetch_byte() as u16;
    cpu.registers.a = cpu.read_8(address);
    12
}

pub fn ld_a_imm16mem(cpu: &mut Cpu) -> u8 {
    let address = cpu.fetch_word();
    cpu.registers.a = cpu.read_8(address);
    16
}

pub fn ld_hl_sp_plus_imm8(cpu: &mut Cpu) -> u8 {
    let data1 = cpu.registers.sp;
    let data2 = cpu.fetch_byte() as i8 as i16 as u16;
    let result = data1.wrapping_add(data2);
    cpu.registers.set_hl(result);

    cpu.registers.f.z = false;
    cpu.registers.f.n = false;
    cpu.registers.f.h = (data1 & 0x000F) + (data2 & 0x000F) > 0x000F;
    cpu.registers.f.c = (data1 & 0x00FF) + (data2 & 0x00FF) > 0x00FF;
    12
}

pub fn ld_sp_hl(cpu: &mut Cpu) -> u8 {
    cpu.registers.sp = cpu.registers.hl();
    8
}

pub fn pop_r16_stk(cpu: &mut Cpu) -> u8 {
    let data = cpu.pop_stack();
    let register = (cpu.current_opcode & 0b0011_0000) >> 4;
    R16Stack::from(register).write(cpu, data);
    12
}

pub fn push_r16_stk(cpu: &mut Cpu) -> u8 {
    let register = (cpu.current_opcode & 0b0011_0000) >> 4;
    let data = R16Stack::from(register).read(cpu);
    cpu.push_stack(data);
    16
}
