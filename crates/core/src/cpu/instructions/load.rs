use crate::cpu::{
    Cpu, MemoryInterface,
    operands::{R8, R16, R16Memory, R16Stack},
};

pub fn ld_r16_imm16<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let destination = (cpu.current_opcode & 0b0011_0000) >> 4;
    let value = cpu.fetch_word();
    R16::from(destination).store(cpu, value);
}

pub fn ld_r16mem_a<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let destination = (cpu.current_opcode & 0b0011_0000) >> 4;
    let address = R16Memory::from(destination).load(cpu);
    cpu.bus.store_8(address, cpu.registers.a(), true);
}

pub fn ld_a_r16mem<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let source = (cpu.current_opcode & 0b0011_0000) >> 4;
    let address = R16Memory::from(source).load(cpu);
    let byte = cpu.bus.load_8(address, true);
    cpu.registers.set_a(byte);
}

pub fn ld_imm16_sp<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let address = cpu.fetch_word();
    cpu.bus.store_16(address, cpu.registers.sp(), true);
}

pub fn ld_r8_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let destination = (cpu.current_opcode & 0b0011_1000) >> 3;
    let value = cpu.fetch_byte();
    let register = R8::from(destination);
    register.write(cpu, value);
}

pub fn ld_r8_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let destination = (cpu.current_opcode & 0b0011_1000) >> 3;
    let source = cpu.current_opcode & 0b0000_0111;
    let register1 = R8::from(destination);
    let register2 = R8::from(source);

    let value = register2.load(cpu);
    register1.write(cpu, value);
}

pub fn ld_cmem_a<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let address = 0xFF00 | cpu.registers.c() as u16;
    cpu.bus.store_8(address, cpu.registers.a(), true);
}

pub fn ld_imm8mem_a<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let address = 0xFF00 | cpu.fetch_byte() as u16;
    cpu.bus.store_8(address, cpu.registers.a(), true);
}

pub fn ld_imm16mem_a<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let address = cpu.fetch_word();
    cpu.bus.store_8(address, cpu.registers.a(), true);
}

pub fn ld_a_cmem<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let address = 0xFF00 | cpu.registers.c() as u16;
    let byte = cpu.bus.load_8(address, true);
    cpu.registers.set_a(byte);
}

pub fn ld_a_imm8mem<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let address = 0xFF00 | cpu.fetch_byte() as u16;
    let byte = cpu.bus.load_8(address, true);
    cpu.registers.set_a(byte);
}

pub fn ld_a_imm16mem<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let address = cpu.fetch_word();
    let byte = cpu.bus.load_8(address, true);
    cpu.registers.set_a(byte);
}

pub fn ld_hl_sp_plus_signed_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let value1 = cpu.registers.sp();
    let value2 = cpu.fetch_byte() as i8 as i16 as u16;
    cpu.bus.m_cycle();
    let result = value1.wrapping_add(value2);
    cpu.registers.set_hl(result);

    cpu.registers.f_mut().set_zero(false);
    cpu.registers.f_mut().set_subtraction(false);
    cpu.registers.f_mut().set_half_carry((value1 & 0x000F) + (value2 & 0x000F) > 0x000F);
    cpu.registers.f_mut().set_carry((value1 & 0x00FF) + (value2 & 0x00FF) > 0x00FF);
}

pub fn ld_sp_hl<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    cpu.registers.set_sp(cpu.registers.hl());
    cpu.bus.m_cycle();
}

pub fn pop_r16_stk<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let value = cpu.pop_stack();
    let register = (cpu.current_opcode & 0b0011_0000) >> 4;
    R16Stack::from(register).store(cpu, value);
}

pub fn push_r16_stk<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let register = (cpu.current_opcode & 0b0011_0000) >> 4;
    let value = R16Stack::from(register).load(cpu);
    cpu.push_stack(value);
}
