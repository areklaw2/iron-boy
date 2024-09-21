use crate::cpu::Cpu;

use super::{R16, R8};

pub fn add_a_r8(cpu: &mut Cpu) -> u8 {
    let data1 = cpu.registers.a;
    let operand = cpu.current_opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data2 = register.read_r8(cpu);
    let result = data1.wrapping_add(data2);
    cpu.registers.a = result;

    cpu.registers.f.set_z(result == 0);
    cpu.registers.f.set_n(false);
    cpu.registers.f.set_h((data1 & 0x0F) + (data2 & 0x0F) > 0x0F);
    cpu.registers.f.set_c(data1 as u16 + data2 as u16 > 0xFF);
    if register == R8::HLMem {
        8
    } else {
        4
    }
}

pub fn add_a_imm8(cpu: &mut Cpu) -> u8 {
    let data1 = cpu.registers.a;
    let data2 = cpu.fetch_byte();
    let result = data1.wrapping_add(data2);
    cpu.registers.a = result;

    cpu.registers.f.set_z(result == 0);
    cpu.registers.f.set_n(false);
    cpu.registers.f.set_h((data1 as u8 & 0x0F) + (data2 as u8 & 0x0F) > 0x0F);
    cpu.registers.f.set_c(data1 as u16 + data2 as u16 > 0xFF);
    8
}

pub fn add_hl_r16(cpu: &mut Cpu) -> u8 {
    let data1 = cpu.registers.hl();
    let operand = (cpu.current_opcode & 0b0011_0000) >> 4;
    let data2 = R16::from(operand).read_r16(cpu);
    let result = data1.wrapping_add(data2);

    cpu.registers.set_hl(result);
    cpu.registers.f.set_n(false);
    cpu.registers.f.set_h((data1 & 0x07FF) + (data2 & 0x07FF) > 0x07FF);
    cpu.registers.f.set_c(data1 as u32 + data2 as u32 > 0xFFFF);
    8
}

pub fn add_sp_imm8(cpu: &mut Cpu) -> u8 {
    let data1 = cpu.registers.sp;
    let data2 = cpu.fetch_byte() as i8 as i16 as u16;
    let result = data1.wrapping_add(data2);
    cpu.registers.sp = result;

    cpu.registers.f.set_z(false);
    cpu.registers.f.set_n(false);
    cpu.registers.f.set_h((data1 & 0x000F) + (data2 & 0x000F) > 0x000F);
    cpu.registers.f.set_c((data1 & 0x00FF) + (data2 & 0x00FF) > 0x00FF);
    16
}

pub fn adc_a_r8(cpu: &mut Cpu) -> u8 {
    let data1 = cpu.registers.a;
    let operand = cpu.current_opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data2 = register.read_r8(cpu);
    let carry = if cpu.registers.f.c { 1 } else { 0 };
    let result = data1.wrapping_add(data2).wrapping_add(carry);
    cpu.registers.a = result;

    cpu.registers.f.set_z(result == 0);
    cpu.registers.f.set_n(false);
    cpu.registers.f.set_h((data1 & 0x0F) + (data2 & 0x0F) + carry > 0x0F);
    cpu.registers.f.set_c(data1 as u16 + data2 as u16 + carry as u16 > 0xFF);
    if register == R8::HLMem {
        8
    } else {
        4
    }
}

pub fn adc_a_imm8(cpu: &mut Cpu) -> u8 {
    let data1 = cpu.registers.a;
    let data2 = cpu.fetch_byte();
    let carry = if cpu.registers.f.c { 1 } else { 0 };
    let result = data1.wrapping_add(data2).wrapping_add(carry);
    cpu.registers.a = result;

    cpu.registers.f.set_z(result == 0);
    cpu.registers.f.set_n(false);
    cpu.registers.f.set_h((data1 & 0x0F) + (data2 & 0x0F) + carry > 0x0F);
    cpu.registers.f.set_c(data1 as u16 + data2 as u16 + carry as u16 > 0xFF);
    8
}

pub fn sub_a_r8(cpu: &mut Cpu) -> u8 {
    let data1 = cpu.registers.a;
    let operand = cpu.current_opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data2 = register.read_r8(cpu);
    let result = data1.wrapping_sub(data2);
    cpu.registers.a = result;

    cpu.registers.f.set_z(result == 0);
    cpu.registers.f.set_n(true);
    cpu.registers.f.set_h((data1 & 0x0F) < (data2 & 0x0F));
    cpu.registers.f.set_c((data1 as u16) < (data2 as u16));
    if register == R8::HLMem {
        8
    } else {
        4
    }
}

pub fn sub_a_imm8(cpu: &mut Cpu) -> u8 {
    let data1 = cpu.registers.a;
    let data2 = cpu.fetch_byte();
    let result = data1.wrapping_sub(data2);
    cpu.registers.a = result;

    cpu.registers.f.set_z(result == 0);
    cpu.registers.f.set_n(true);
    cpu.registers.f.set_h((data1 & 0x0F) < (data2 & 0x0F));
    cpu.registers.f.set_c((data1 as u16) < (data2 as u16));
    8
}

pub fn sbc_a_r8(cpu: &mut Cpu) -> u8 {
    let data1 = cpu.registers.a;
    let operand = cpu.current_opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data2 = register.read_r8(cpu);
    let carry = if cpu.registers.f.c { 1 } else { 0 };
    let result = data1.wrapping_sub(data2).wrapping_sub(carry);
    cpu.registers.a = result;

    cpu.registers.f.set_z(result == 0);
    cpu.registers.f.set_n(true);
    cpu.registers.f.set_h((data1 & 0x0F) < (data2 & 0x0F) + carry);
    cpu.registers.f.set_c((data1 as u16) < (data2 as u16) + carry as u16);
    if register == R8::HLMem {
        8
    } else {
        4
    }
}

pub fn sbc_a_imm8(cpu: &mut Cpu) -> u8 {
    let data1 = cpu.registers.a;
    let data2 = cpu.fetch_byte();
    let carry = if cpu.registers.f.c { 1 } else { 0 };
    let result = data1.wrapping_sub(data2).wrapping_sub(carry);
    cpu.registers.a = result;

    cpu.registers.f.set_z(result == 0);
    cpu.registers.f.set_n(true);
    cpu.registers.f.set_h((data1 & 0x0F) < (data2 & 0x0F) + carry);
    cpu.registers.f.set_c((data1 as u16) < (data2 as u16) + carry as u16);
    8
}

pub fn and_a_r8(cpu: &mut Cpu) -> u8 {
    let operand = cpu.current_opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data = register.read_r8(cpu);
    let result = cpu.registers.a & data;
    cpu.registers.a = result;

    cpu.registers.f.set_z(result == 0);
    cpu.registers.f.set_n(false);
    cpu.registers.f.set_h(true);
    cpu.registers.f.set_c(false);
    if register == R8::HLMem {
        8
    } else {
        4
    }
}

pub fn and_a_imm8(cpu: &mut Cpu) -> u8 {
    let data = cpu.fetch_byte();
    let result = cpu.registers.a & data;
    cpu.registers.a = result;

    cpu.registers.f.set_z(result == 0);
    cpu.registers.f.set_n(false);
    cpu.registers.f.set_h(true);
    cpu.registers.f.set_c(false);
    8
}

pub fn xor_a_r8(cpu: &mut Cpu) -> u8 {
    let operand = cpu.current_opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data = register.read_r8(cpu);
    let result = cpu.registers.a ^ data;
    cpu.registers.a = result;

    cpu.registers.f.set_z(result == 0);
    cpu.registers.f.set_n(false);
    cpu.registers.f.set_h(false);
    cpu.registers.f.set_c(false);
    if register == R8::HLMem {
        8
    } else {
        4
    }
}

pub fn xor_a_imm8(cpu: &mut Cpu) -> u8 {
    let data = cpu.fetch_byte();
    let result = cpu.registers.a ^ data;
    cpu.registers.a = result;

    cpu.registers.f.set_z(result == 0);
    cpu.registers.f.set_n(false);
    cpu.registers.f.set_h(false);
    cpu.registers.f.set_c(false);
    8
}

pub fn or_a_r8(cpu: &mut Cpu) -> u8 {
    let operand = cpu.current_opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data = register.read_r8(cpu);
    let result = cpu.registers.a | data;
    cpu.registers.a = result;

    cpu.registers.f.set_z(result == 0);
    cpu.registers.f.set_n(false);
    cpu.registers.f.set_h(false);
    cpu.registers.f.set_c(false);
    if register == R8::HLMem {
        8
    } else {
        4
    }
}

pub fn or_a_imm8(cpu: &mut Cpu) -> u8 {
    let data = cpu.fetch_byte();
    let result = cpu.registers.a | data;
    cpu.registers.a = result;

    cpu.registers.f.set_z(result == 0);
    cpu.registers.f.set_n(false);
    cpu.registers.f.set_h(false);
    cpu.registers.f.set_c(false);
    8
}

pub fn cp_a_r8(cpu: &mut Cpu) -> u8 {
    let data1 = cpu.registers.a;
    let operand = cpu.current_opcode & 0b0000_0111;
    let register = R8::from(operand);
    let data2 = register.read_r8(cpu);
    let result = data1.wrapping_sub(data2);

    cpu.registers.f.set_z(result == 0);
    cpu.registers.f.set_n(true);
    cpu.registers.f.set_h((data1 & 0x0F) < (data2 & 0x0F));
    cpu.registers.f.set_c((data1 as u16) < (data2 as u16));
    if register == R8::HLMem {
        8
    } else {
        4
    }
}

pub fn cp_a_imm8(cpu: &mut Cpu) -> u8 {
    let data1 = cpu.registers.a;
    let data2 = cpu.fetch_byte();
    let result = data1.wrapping_sub(data2);

    cpu.registers.f.set_z(result == 0);
    cpu.registers.f.set_n(true);
    cpu.registers.f.set_h((data1 & 0x0F) < (data2 & 0x0F));
    cpu.registers.f.set_c((data1 as u16) < (data2 as u16));
    8
}

pub fn inc_r16(cpu: &mut Cpu) -> u8 {
    let operand = (cpu.current_opcode & 0b0011_0000) >> 4;
    let register = R16::from(operand);
    let data = register.read_r16(cpu).wrapping_add(1);
    register.write_r16(cpu, data);
    8
}

pub fn inc_r8(cpu: &mut Cpu) -> u8 {
    let operand = (cpu.current_opcode & 0b0011_1000) >> 3;
    let register = R8::from(operand);
    let data = register.read_r8(cpu);
    let result = data.wrapping_add(1);
    register.write_r8(cpu, result);

    cpu.registers.f.set_z(result == 0);
    cpu.registers.f.set_n(false);
    cpu.registers.f.set_h((data & 0x0F) + 1 > 0x0F);
    if register == R8::HLMem {
        12
    } else {
        4
    }
}

pub fn dec_r16(cpu: &mut Cpu) -> u8 {
    let operand = (cpu.current_opcode & 0b0011_0000) >> 4;
    let register = R16::from(operand);
    let data = register.read_r16(cpu).wrapping_sub(1);
    register.write_r16(cpu, data);
    8
}

pub fn dec_r8(cpu: &mut Cpu) -> u8 {
    let operand = (cpu.current_opcode & 0b0011_1000) >> 3;
    let register = R8::from(operand);
    let data = register.read_r8(cpu);
    let result = data.wrapping_sub(1);
    register.write_r8(cpu, result);

    cpu.registers.f.set_z(result == 0);
    cpu.registers.f.set_n(true);
    cpu.registers.f.set_h((data & 0x0F) == 0);
    if register == R8::HLMem {
        12
    } else {
        4
    }
}
