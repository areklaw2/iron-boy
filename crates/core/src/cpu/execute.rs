use crate::cpu::{
    Cpu, MemoryInterface,
    operands::{Condition, R8, R16, R16Memory, R16Stack},
};

pub fn ld_r16_imm16<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let destination = (cpu.opcode & 0b0011_0000) >> 4;
    let value = cpu.fetch_word();
    R16::from(destination).store(cpu, value);
}

pub fn ld_r16mem_a<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let destination = (cpu.opcode & 0b0011_0000) >> 4;
    let address = R16Memory::from(destination).load(cpu);
    cpu.bus.store_8(address, cpu.registers.a(), true);
}

pub fn ld_a_r16mem<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let source = (cpu.opcode & 0b0011_0000) >> 4;
    let address = R16Memory::from(source).load(cpu);
    let byte = cpu.bus.load_8(address, true);
    cpu.registers.set_a(byte);
}

pub fn ld_imm16_sp<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let address = cpu.fetch_word();
    cpu.bus.store_16(address, cpu.registers.sp(), true);
}

pub fn ld_r8_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let destination = (cpu.opcode & 0b0011_1000) >> 3;
    let value = cpu.fetch_byte();
    let register = R8::from(destination);
    register.write(cpu, value);
}

pub fn ld_r8_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let destination = (cpu.opcode & 0b0011_1000) >> 3;
    let source = cpu.opcode & 0b0000_0111;
    let register1 = R8::from(destination);
    let register2 = R8::from(source);

    let value = register2.load(cpu);
    register1.write(cpu, value);
}

pub fn ldh_cmem_a<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let address = 0xFF00 | cpu.registers.c() as u16;
    cpu.bus.store_8(address, cpu.registers.a(), true);
}

pub fn ldh_imm8mem_a<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let address = 0xFF00 | cpu.fetch_byte() as u16;
    cpu.bus.store_8(address, cpu.registers.a(), true);
}

pub fn ld_imm16mem_a<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let address = cpu.fetch_word();
    cpu.bus.store_8(address, cpu.registers.a(), true);
}

pub fn ldh_a_cmem<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let address = 0xFF00 | cpu.registers.c() as u16;
    let byte = cpu.bus.load_8(address, true);
    cpu.registers.set_a(byte);
}

pub fn ldh_a_imm8mem<I: MemoryInterface>(cpu: &mut Cpu<I>) {
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
    let register = (cpu.opcode & 0b0011_0000) >> 4;
    R16Stack::from(register).store(cpu, value);
}

pub fn push_r16_stk<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let register = (cpu.opcode & 0b0011_0000) >> 4;
    let value = R16Stack::from(register).load(cpu);
    cpu.push_stack(value);
}

pub fn add_a_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let value1 = cpu.registers.a();
    let operand = cpu.opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value2 = register.load(cpu);
    let result = value1.wrapping_add(value2);
    cpu.registers.set_a(result);

    cpu.registers.f_mut().set_zero(result == 0);
    cpu.registers.f_mut().set_subtraction(false);
    cpu.registers.f_mut().set_half_carry((value1 & 0x0F) + (value2 & 0x0F) > 0x0F);
    cpu.registers.f_mut().set_carry(value1 as u16 + value2 as u16 > 0xFF);
}

pub fn add_a_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let value1 = cpu.registers.a();
    let value2 = cpu.fetch_byte();
    let result = value1.wrapping_add(value2);
    cpu.registers.set_a(result);

    cpu.registers.f_mut().set_zero(result == 0);
    cpu.registers.f_mut().set_subtraction(false);
    cpu.registers.f_mut().set_half_carry((value1 as u8 & 0x0F) + (value2 as u8 & 0x0F) > 0x0F);
    cpu.registers.f_mut().set_carry(value1 as u16 + value2 as u16 > 0xFF);
}

pub fn add_hl_r16<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let value1 = cpu.registers.hl();
    let operand = (cpu.opcode & 0b0011_0000) >> 4;
    let value2 = R16::from(operand).load(cpu);
    let result = value1.wrapping_add(value2);
    cpu.bus.m_cycle();

    cpu.registers.set_hl(result);
    cpu.registers.f_mut().set_subtraction(false);
    cpu.registers.f_mut().set_half_carry((value1 & 0x0FFF) + (value2 & 0x0FFF) > 0x0FFF);
    cpu.registers.f_mut().set_carry(value1 as u32 + value2 as u32 > 0xFFFF);
}

pub fn add_sp_signed_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let value1 = cpu.registers.sp();
    let value2 = cpu.fetch_byte() as i8 as i16 as u16;
    let result = value1.wrapping_add(value2);
    cpu.bus.m_cycle();
    cpu.registers.set_sp(result);
    cpu.bus.m_cycle();

    cpu.registers.f_mut().set_zero(false);
    cpu.registers.f_mut().set_subtraction(false);
    cpu.registers.f_mut().set_half_carry((value1 & 0x000F) + (value2 & 0x000F) > 0x000F);
    cpu.registers.f_mut().set_carry((value1 & 0x00FF) + (value2 & 0x00FF) > 0x00FF);
}

pub fn adc_a_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let value1 = cpu.registers.a();
    let operand = cpu.opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value2 = register.load(cpu);
    let carry = if cpu.registers.f().carry() { 1 } else { 0 };
    let result = value1.wrapping_add(value2).wrapping_add(carry);
    cpu.registers.set_a(result);

    cpu.registers.f_mut().set_zero(result == 0);
    cpu.registers.f_mut().set_subtraction(false);
    cpu.registers.f_mut().set_half_carry((value1 & 0x0F) + (value2 & 0x0F) + carry > 0x0F);
    cpu.registers.f_mut().set_carry(value1 as u16 + value2 as u16 + carry as u16 > 0xFF);
}

pub fn adc_a_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let value1 = cpu.registers.a();
    let value2 = cpu.fetch_byte();
    let carry = if cpu.registers.f().carry() { 1 } else { 0 };
    let result = value1.wrapping_add(value2).wrapping_add(carry);
    cpu.registers.set_a(result);

    cpu.registers.f_mut().set_zero(result == 0);
    cpu.registers.f_mut().set_subtraction(false);
    cpu.registers.f_mut().set_half_carry((value1 & 0x0F) + (value2 & 0x0F) + carry > 0x0F);
    cpu.registers.f_mut().set_carry(value1 as u16 + value2 as u16 + carry as u16 > 0xFF);
}

pub fn sub_a_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let value1 = cpu.registers.a();
    let operand = cpu.opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value2 = register.load(cpu);
    let result = value1.wrapping_sub(value2);
    cpu.registers.set_a(result);

    cpu.registers.f_mut().set_zero(result == 0);
    cpu.registers.f_mut().set_subtraction(true);
    cpu.registers.f_mut().set_half_carry((value1 & 0x0F) < (value2 & 0x0F));
    cpu.registers.f_mut().set_carry((value1 as u16) < (value2 as u16));
}

pub fn sub_a_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let value1 = cpu.registers.a();
    let value2 = cpu.fetch_byte();
    let result = value1.wrapping_sub(value2);
    cpu.registers.set_a(result);

    cpu.registers.f_mut().set_zero(result == 0);
    cpu.registers.f_mut().set_subtraction(true);
    cpu.registers.f_mut().set_half_carry((value1 & 0x0F) < (value2 & 0x0F));
    cpu.registers.f_mut().set_carry((value1 as u16) < (value2 as u16));
}

pub fn sbc_a_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let value1 = cpu.registers.a();
    let operand = cpu.opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value2 = register.load(cpu);
    let carry = if cpu.registers.f().carry() { 1 } else { 0 };
    let result = value1.wrapping_sub(value2).wrapping_sub(carry);
    cpu.registers.set_a(result);

    cpu.registers.f_mut().set_zero(result == 0);
    cpu.registers.f_mut().set_subtraction(true);
    cpu.registers.f_mut().set_half_carry((value1 & 0x0F) < (value2 & 0x0F) + carry);
    cpu.registers.f_mut().set_carry((value1 as u16) < (value2 as u16) + carry as u16);
}

pub fn sbc_a_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let value1 = cpu.registers.a();
    let value2 = cpu.fetch_byte();
    let carry = if cpu.registers.f().carry() { 1 } else { 0 };
    let result = value1.wrapping_sub(value2).wrapping_sub(carry);
    cpu.registers.set_a(result);

    cpu.registers.f_mut().set_zero(result == 0);
    cpu.registers.f_mut().set_subtraction(true);
    cpu.registers.f_mut().set_half_carry((value1 & 0x0F) < (value2 & 0x0F) + carry);
    cpu.registers.f_mut().set_carry((value1 as u16) < (value2 as u16) + carry as u16);
}

pub fn and_a_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let operand = cpu.opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let result = cpu.registers.a() & value;
    cpu.registers.set_a(result);

    cpu.registers.f_mut().set_zero(result == 0);
    cpu.registers.f_mut().set_subtraction(false);
    cpu.registers.f_mut().set_half_carry(true);
    cpu.registers.f_mut().set_carry(false);
}

pub fn and_a_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let value = cpu.fetch_byte();
    let result = cpu.registers.a() & value;
    cpu.registers.set_a(result);

    cpu.registers.f_mut().set_zero(result == 0);
    cpu.registers.f_mut().set_subtraction(false);
    cpu.registers.f_mut().set_half_carry(true);
    cpu.registers.f_mut().set_carry(false);
}

pub fn xor_a_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let operand = cpu.opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let result = cpu.registers.a() ^ value;
    cpu.registers.set_a(result);

    cpu.registers.f_mut().set_zero(result == 0);
    cpu.registers.f_mut().set_subtraction(false);
    cpu.registers.f_mut().set_half_carry(false);
    cpu.registers.f_mut().set_carry(false);
}

pub fn xor_a_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let value = cpu.fetch_byte();
    let result = cpu.registers.a() ^ value;
    cpu.registers.set_a(result);

    cpu.registers.f_mut().set_zero(result == 0);
    cpu.registers.f_mut().set_subtraction(false);
    cpu.registers.f_mut().set_half_carry(false);
    cpu.registers.f_mut().set_carry(false);
}

pub fn or_a_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let operand = cpu.opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let result = cpu.registers.a() | value;
    cpu.registers.set_a(result);

    cpu.registers.f_mut().set_zero(result == 0);
    cpu.registers.f_mut().set_subtraction(false);
    cpu.registers.f_mut().set_half_carry(false);
    cpu.registers.f_mut().set_carry(false);
}

pub fn or_a_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let value = cpu.fetch_byte();
    let result = cpu.registers.a() | value;
    cpu.registers.set_a(result);

    cpu.registers.f_mut().set_zero(result == 0);
    cpu.registers.f_mut().set_subtraction(false);
    cpu.registers.f_mut().set_half_carry(false);
    cpu.registers.f_mut().set_carry(false);
}

pub fn cp_a_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let value1 = cpu.registers.a();
    let operand = cpu.opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value2 = register.load(cpu);
    let result = value1.wrapping_sub(value2);

    cpu.registers.f_mut().set_zero(result == 0);
    cpu.registers.f_mut().set_subtraction(true);
    cpu.registers.f_mut().set_half_carry((value1 & 0x0F) < (value2 & 0x0F));
    cpu.registers.f_mut().set_carry((value1 as u16) < (value2 as u16));
}

pub fn cp_a_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let value1 = cpu.registers.a();
    let value2 = cpu.fetch_byte();
    let result = value1.wrapping_sub(value2);

    cpu.registers.f_mut().set_zero(result == 0);
    cpu.registers.f_mut().set_subtraction(true);
    cpu.registers.f_mut().set_half_carry((value1 & 0x0F) < (value2 & 0x0F));
    cpu.registers.f_mut().set_carry((value1 as u16) < (value2 as u16));
}

pub fn inc_r16<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let operand = (cpu.opcode & 0b0011_0000) >> 4;
    let register = R16::from(operand);
    let value = register.load(cpu).wrapping_add(1);
    register.store(cpu, value);
    cpu.bus.m_cycle();
}

pub fn inc_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let operand = (cpu.opcode & 0b0011_1000) >> 3;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let result = value.wrapping_add(1);
    register.write(cpu, result);

    cpu.registers.f_mut().set_zero(result == 0);
    cpu.registers.f_mut().set_subtraction(false);
    cpu.registers.f_mut().set_half_carry((value & 0x0F) + 1 > 0x0F);
}

pub fn dec_r16<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let operand = (cpu.opcode & 0b0011_0000) >> 4;
    let register = R16::from(operand);
    let value = register.load(cpu).wrapping_sub(1);
    register.store(cpu, value);
    cpu.bus.m_cycle();
}

pub fn dec_r8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let operand = (cpu.opcode & 0b0011_1000) >> 3;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let result = value.wrapping_sub(1);
    register.write(cpu, result);

    cpu.registers.f_mut().set_zero(result == 0);
    cpu.registers.f_mut().set_subtraction(true);
    cpu.registers.f_mut().set_half_carry((value & 0x0F) == 0);
}

pub fn rlca<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let carry = cpu.registers.a() & 0x80 == 0x80;
    let result = (cpu.registers.a() << 1) | (if carry { 1 } else { 0 });

    cpu.registers.f_mut().set_zero(false);
    cpu.registers.f_mut().set_subtraction(false);
    cpu.registers.f_mut().set_half_carry(false);
    cpu.registers.f_mut().set_carry(carry);

    cpu.registers.set_a(result);
}

pub fn rrca<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let carry = cpu.registers.a() & 0x01 == 0x01;
    let result = (cpu.registers.a() >> 1) | (if carry { 0x80 } else { 0 });

    cpu.registers.f_mut().set_zero(false);
    cpu.registers.f_mut().set_subtraction(false);
    cpu.registers.f_mut().set_half_carry(false);
    cpu.registers.f_mut().set_carry(carry);

    cpu.registers.set_a(result);
}

pub fn rla<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let carry = cpu.registers.a() & 0x80 == 0x80;
    let result = (cpu.registers.a() << 1) | (if cpu.registers.f().carry() { 1 } else { 0 });

    cpu.registers.f_mut().set_zero(false);
    cpu.registers.f_mut().set_subtraction(false);
    cpu.registers.f_mut().set_half_carry(false);
    cpu.registers.f_mut().set_carry(carry);

    cpu.registers.set_a(result);
}

pub fn rra<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let carry = cpu.registers.a() & 0x01 == 0x01;
    let result = (cpu.registers.a() >> 1) | (if cpu.registers.f().carry() { 0x80 } else { 0 });

    cpu.registers.f_mut().set_zero(false);
    cpu.registers.f_mut().set_subtraction(false);
    cpu.registers.f_mut().set_half_carry(false);
    cpu.registers.f_mut().set_carry(carry);

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

    cpu.registers.f_mut().set_zero(result == 0);
    cpu.registers.f_mut().set_subtraction(false);
    cpu.registers.f_mut().set_half_carry(false);
    cpu.registers.f_mut().set_carry(false);
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

fn set_rotate_shift_flags<I: MemoryInterface>(cpu: &mut Cpu<I>, result: u8, carry: bool) {
    cpu.registers.f_mut().set_zero(result == 0);
    cpu.registers.f_mut().set_subtraction(false);
    cpu.registers.f_mut().set_half_carry(false);
    cpu.registers.f_mut().set_carry(carry);
}

pub fn jr_signed_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let signed = cpu.fetch_byte() as i8;
    cpu.registers.set_pc(((cpu.registers.pc() as u32 as i32) + (signed as i32)) as u16);
    cpu.bus.m_cycle();
}

pub fn jr_cond_signed_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let z = cpu.registers.f().zero();
    let c = cpu.registers.f().carry();

    let cond = (cpu.opcode & 0b0001_1000) >> 3;
    let jump = match Condition::from(cond) {
        Condition::NC => c == false,
        Condition::C => c == true,
        Condition::NZ => z == false,
        Condition::Z => z == true,
    };

    if jump {
        let signed_byte = cpu.fetch_byte() as i8;
        cpu.registers.set_pc(((cpu.registers.pc() as i16) + (signed_byte as i16)) as u16);
    } else {
        cpu.registers.set_pc(cpu.registers.pc() + 1);
    }
    cpu.bus.m_cycle();
}

pub fn jp_cond_imm16<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let z = cpu.registers.f().zero();
    let c = cpu.registers.f().carry();

    let cond = (cpu.opcode & 0b0001_1000) >> 3;
    let jump = match Condition::from(cond) {
        Condition::NC => c == false,
        Condition::C => c == true,
        Condition::NZ => z == false,
        Condition::Z => z == true,
    };

    let word = cpu.fetch_word();
    if jump {
        cpu.registers.set_pc(word);
        cpu.bus.m_cycle();
    }
}

pub fn jp_imm16<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let word = cpu.fetch_word();
    cpu.registers.set_pc(word);
    cpu.bus.m_cycle();
}

pub fn jp_hl<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let hl = cpu.registers.hl();
    cpu.registers.set_pc(hl);
}

pub fn ret_cond<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let z = cpu.registers.f().zero();
    let c = cpu.registers.f().carry();

    let cond = (cpu.opcode & 0b0001_1000) >> 3;
    let ret = match Condition::from(cond) {
        Condition::NC => c == false,
        Condition::C => c == true,
        Condition::NZ => z == false,
        Condition::Z => z == true,
    };

    if ret {
        let pop_stack = cpu.pop_stack();
        cpu.registers.set_pc(pop_stack);
        cpu.bus.m_cycle();
    }
    cpu.bus.m_cycle();
}

pub fn ret<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let pop_stack = cpu.pop_stack();
    cpu.registers.set_pc(pop_stack);
    cpu.bus.m_cycle();
}

pub fn reti<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let pop_stack = cpu.pop_stack();
    cpu.registers.set_pc(pop_stack);
    cpu.bus.m_cycle();
    cpu.interrupt_master_enable = true;
}

pub fn call_cond_imm16<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let z = cpu.registers.f().zero();
    let c = cpu.registers.f().carry();

    let cond = (cpu.opcode & 0b0001_1000) >> 3;
    let call = match Condition::from(cond) {
        Condition::NC => c == false,
        Condition::C => c == true,
        Condition::NZ => z == false,
        Condition::Z => z == true,
    };

    let word = cpu.fetch_word();
    if call {
        cpu.push_stack(cpu.registers.pc());
        cpu.registers.set_pc(word);
    }
}

pub fn call_imm16<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    cpu.push_stack(cpu.registers.pc() + 2);
    let word = cpu.fetch_word();
    cpu.registers.set_pc(word);
}

pub fn rst_tgt3<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    cpu.push_stack(cpu.registers.pc());
    let target = cpu.opcode & 0b0011_1000;
    cpu.registers.set_pc(target as u16);
}

pub fn bit_b3_r8<I: MemoryInterface>(cpu: &mut Cpu<I>, opcode: u8) {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let bit_index = (opcode & 0b0011_1000) >> 3;

    let result = value & (1 << (bit_index)) == 0;
    cpu.registers.f_mut().set_zero(result);
    cpu.registers.f_mut().set_subtraction(false);
    cpu.registers.f_mut().set_half_carry(true);
}

pub fn res_b3_r8<I: MemoryInterface>(cpu: &mut Cpu<I>, opcode: u8) {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let bit_index = (opcode & 0b0011_1000) >> 3;
    register.write(cpu, value & !(1 << bit_index));
}

pub fn set_b3_r8<I: MemoryInterface>(cpu: &mut Cpu<I>, opcode: u8) {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand);
    let value = register.load(cpu);
    let bit_index = (opcode & 0b0011_1000) >> 3;
    register.write(cpu, value | (1 << bit_index));
}

pub fn daa<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let mut a = cpu.registers.a();
    let mut correction = if cpu.registers.f().carry() { 0x60 } else { 0x00 };

    if cpu.registers.f().half_carry() {
        correction |= 0x06;
    }

    if !cpu.registers.f().subtraction() {
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

    cpu.registers.f_mut().set_zero(a == 0);
    cpu.registers.f_mut().set_half_carry(false);
    cpu.registers.f_mut().set_carry(correction >= 0x60);
    cpu.registers.set_a(a);
}

pub fn cpl<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let not_a = !cpu.registers.a();
    cpu.registers.set_a(not_a);
    cpu.registers.f_mut().set_subtraction(true);
    cpu.registers.f_mut().set_half_carry(true);
}

pub fn scf<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    cpu.registers.f_mut().set_carry(true);
    cpu.registers.f_mut().set_half_carry(false);
    cpu.registers.f_mut().set_subtraction(false);
}

pub fn ccf<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let carry = !cpu.registers.f().carry();
    cpu.registers.f_mut().set_carry(carry);
    cpu.registers.f_mut().set_half_carry(false);
    cpu.registers.f_mut().set_subtraction(false);
}

pub fn stop<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    cpu.bus.change_speed();
}

pub fn halt<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    *cpu.halted.borrow_mut() = true;
    cpu.halt_bug = cpu.bus.pending_interrupt() != 0 && !cpu.interrupt_master_enable
}

pub fn di<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    cpu.disable_interrupt_delay = 2;
}

pub fn ei<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    cpu.enable_interrupt_delay = 2;
}

pub fn prefix<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let opcode = cpu.fetch_byte();
    let operation = (opcode & 0b1100_0000) >> 6;
    match operation {
        0b01 => bit_b3_r8(cpu, opcode),
        0b10 => res_b3_r8(cpu, opcode),
        0b11 => set_b3_r8(cpu, opcode),
        0b00 => {
            let operation = (opcode & 0b0011_1000) >> 3;
            match operation {
                0b000 => rlc_r8(cpu, opcode),
                0b001 => rrc_r8(cpu, opcode),
                0b010 => rl_r8(cpu, opcode),
                0b011 => rr_r8(cpu, opcode),
                0b100 => sla_r8(cpu, opcode),
                0b101 => sra_r8(cpu, opcode),
                0b110 => swap_r8(cpu, opcode),
                0b111 => srl_r8(cpu, opcode),
                _ => panic!("No operation exists"),
            }
        }
        _ => panic!("No operation exists"),
    }
}
