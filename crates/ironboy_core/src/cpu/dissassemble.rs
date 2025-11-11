use crate::cpu::operands::{Condition, R8, R16, R16Memory, R16Stack};

pub fn ld_r16_imm16(opcode: u8, next_word: u16) -> String {
    let destination = (opcode & 0b0011_0000) >> 4;
    let register = R16::from(destination).to_string();
    format!("LD {register},{:#04X}", next_word)
}

pub fn ld_r16mem_a(opcode: u8) -> String {
    let destination = (opcode & 0b0011_0000) >> 4;
    let register = R16Memory::from(destination).to_string();
    format!("LD [{register}],A")
}

pub fn ld_a_r16mem(opcode: u8) -> String {
    let source = (opcode & 0b0011_0000) >> 4;
    let register = R16Memory::from(source).to_string();
    format!("LD A,[{register}]")
}

pub fn ld_imm16_sp(next_word: u16) -> String {
    format!("LD {:#04X},SP", next_word)
}

pub fn ld_r8_imm8(opcode: u8, next_byte: u8) -> String {
    let destination = (opcode & 0b0011_1000) >> 3;
    let register = R8::from(destination).to_string();
    format!("LD {register},{:#04X}", next_byte)
}

pub fn ld_r8_r8(opcode: u8) -> String {
    let destination = (opcode & 0b0011_1000) >> 3;
    let source = opcode & 0b0000_0111;
    let register1 = R8::from(destination).to_string();
    let register2 = R8::from(source).to_string();
    format!("LD {register1},{register2}")
}

pub fn ldh_cmem_a() -> String {
    "LD [FF00+C],A".to_string()
}

pub fn ldh_imm8mem_a(next_byte: u8) -> String {
    format!("LD [FF00+{:#04X}],A", next_byte)
}

pub fn ld_imm16mem_a(next_word: u16) -> String {
    format!("LD [{:#04X}],A", next_word)
}

pub fn ldh_a_cmem() -> String {
    "LD A,[FF00+C]".to_string()
}

pub fn ldh_a_imm8mem(next_byte: u8) -> String {
    format!("LD A,[FF00+{:#04X}]", next_byte)
}

pub fn ld_a_imm16mem(next_word: u16) -> String {
    format!("LD A,[{:#04X}]", next_word)
}

pub fn ld_hl_sp_plus_signed_imm8(next_byte: u8) -> String {
    format!("LD HL,SP+{:#04X}", next_byte)
}

pub fn ld_sp_hl() -> String {
    "LD SP,HL".to_string()
}

pub fn pop_r16_stk(opcode: u8) -> String {
    let register = (opcode & 0b0011_0000) >> 4;
    let register = R16Stack::from(register).to_string();
    format!("POP {register}")
}

pub fn push_r16_stk(opcode: u8) -> String {
    let register = (opcode & 0b0011_0000) >> 4;
    let register = R16Stack::from(register).to_string();
    format!("PUSH {register}")
}

pub fn inc_r16(opcode: u8) -> String {
    let operand = (opcode & 0b0011_0000) >> 4;
    let register = R16::from(operand).to_string();
    format!("INC {register}")
}

pub fn inc_r8(opcode: u8) -> String {
    let operand = (opcode & 0b0011_1000) >> 3;
    let register = R8::from(operand).to_string();
    format!("INC {register}")
}

pub fn dec_r16(opcode: u8) -> String {
    let operand = (opcode & 0b0011_0000) >> 4;
    let register = R16::from(operand);
    format!("DEC {register}")
}

pub fn dec_r8(opcode: u8) -> String {
    let operand = (opcode & 0b0011_1000) >> 3;
    let register = R8::from(operand);
    format!("DEC {register}")
}

pub fn add_hl_r16(opcode: u8) -> String {
    let operand = (opcode & 0b0011_0000) >> 4;
    let register = R16::from(operand).to_string();
    format!("ADD HL,{register}")
}

pub fn add_sp_signed_imm8() -> String {
    "ADD SP,u8".to_string()
}

pub fn add_a_r8(opcode: u8) -> String {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand).to_string();
    format!("ADD A,{register}")
}

pub fn adc_a_r8(opcode: u8) -> String {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand).to_string();
    format!("ADC A,{register}")
}

pub fn sub_a_r8(opcode: u8) -> String {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand).to_string();
    format!("SUB A,{register}")
}

pub fn sbc_a_r8(opcode: u8) -> String {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand).to_string();
    format!("SBC A,{register}")
}

pub fn and_a_r8(opcode: u8) -> String {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand).to_string();
    format!("AND A,{register}")
}

pub fn xor_a_r8(opcode: u8) -> String {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand).to_string();
    format!("XOR A,{register}")
}

pub fn or_a_r8(opcode: u8) -> String {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand).to_string();
    format!("OR A,{register}")
}

pub fn cp_a_r8(opcode: u8) -> String {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand).to_string();
    format!("CP A,{register}")
}

pub fn add_a_imm8(next_byte: u8) -> String {
    format!("ADD A,{:#04X}", next_byte)
}

pub fn adc_a_imm8(next_byte: u8) -> String {
    format!("ADC A,{:#04X}", next_byte)
}

pub fn sub_a_imm8(next_byte: u8) -> String {
    format!("SUB A,{:#04X}", next_byte)
}

pub fn sbc_a_imm8(next_byte: u8) -> String {
    format!("SBC A,{:#04X}", next_byte)
}

pub fn and_a_imm8(next_byte: u8) -> String {
    format!("AND A,{:#04X}", next_byte)
}

pub fn xor_a_imm8(next_byte: u8) -> String {
    format!("XOR A,{:#04X}", next_byte)
}

pub fn or_a_imm8(next_byte: u8) -> String {
    format!("OR A,{:#04X}", next_byte)
}

pub fn cp_a_imm8(next_byte: u8) -> String {
    format!("CP A,{:#04X}", next_byte)
}

pub fn rlca() -> String {
    "RLCA".to_string()
}

pub fn rrca() -> String {
    "RRCA".to_string()
}

pub fn rla() -> String {
    "RLA".to_string()
}

pub fn rra() -> String {
    "RRA".to_string()
}

pub fn rlc_r8(opcode: u8) -> String {
    let operand = opcode & 0b0000_0111;
    let register = &R8::from(operand).to_string();
    format!("RLC {register}")
}

pub fn rrc_r8(opcode: u8) -> String {
    let operand = opcode & 0b0000_0111;
    let register = &R8::from(operand).to_string();
    format!("RRC {register}")
}

pub fn rl_r8(opcode: u8) -> String {
    let operand = opcode & 0b0000_0111;
    let register = &R8::from(operand).to_string();
    format!("RL {register}")
}

pub fn rr_r8(opcode: u8) -> String {
    let operand = opcode & 0b0000_0111;
    let register = &R8::from(operand).to_string();
    format!("RR {register}")
}

pub fn sla_r8(opcode: u8) -> String {
    let operand = opcode & 0b0000_0111;
    let register = &R8::from(operand).to_string();
    format!("SLA {register}")
}

pub fn sra_r8(opcode: u8) -> String {
    let operand = opcode & 0b0000_0111;
    let register = &R8::from(operand).to_string();
    format!("SRA {register}")
}

pub fn swap_r8(opcode: u8) -> String {
    let operand = opcode & 0b0000_0111;
    let register = &R8::from(operand).to_string();
    format!("SWAP {register}")
}

pub fn srl_r8(opcode: u8) -> String {
    let operand = opcode & 0b0000_0111;
    let register = &R8::from(operand).to_string();
    format!("SRL {register}")
}

pub fn jr_signed_imm8(next_byte: u8) -> String {
    format!("JR {:#04X}", next_byte)
}

pub fn jr_cond_signed_imm8(opcode: u8, next_byte: u8) -> String {
    let cond = (opcode & 0b0001_1000) >> 3;
    let cond = Condition::from(cond).to_string();
    format!("JR {cond},{:#04X}", next_byte)
}

pub fn jp_imm16(next_word: u16) -> String {
    format!("JP {:#04X}", next_word)
}

pub fn jp_cond_imm16(opcode: u8, next_word: u16) -> String {
    let cond = (opcode & 0b0001_1000) >> 3;
    let cond = Condition::from(cond).to_string();
    format!("JP {cond},{:#04X}", next_word)
}

pub fn jp_hl() -> String {
    "JP HL".to_string()
}

pub fn ret() -> String {
    "RET".to_string()
}

pub fn ret_cond(opcode: u8) -> String {
    let cond = (opcode & 0b0001_1000) >> 3;
    let cond = Condition::from(cond).to_string();
    format!("RET {cond}")
}

pub fn reti() -> String {
    "RETI".to_string()
}

pub fn call_imm16(next_word: u16) -> String {
    format!("CALL {:#04X}", next_word)
}

pub fn call_cond_imm16(opcode: u8, next_word: u16) -> String {
    let cond = (opcode & 0b0001_1000) >> 3;
    let cond = Condition::from(cond).to_string();
    format!("CALL {cond},{:#04X}", next_word)
}

pub fn rst_tgt3(opcode: u8) -> String {
    let target = ((opcode & 0b0011_1000) >> 3) / 8;
    format!("RST {:02}h", target)
}

pub fn bit_b3_r8(opcode: u8) -> String {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand).to_string();
    let bit_index = (opcode & 0b0011_1000) >> 3;
    format!("BIT {},{}", bit_index, register)
}
pub fn res_b3_r8(opcode: u8) -> String {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand).to_string();
    let bit_index = (opcode & 0b0011_1000) >> 3;
    format!("RES {},{}", bit_index, register)
}
pub fn set_b3_r8(opcode: u8) -> String {
    let operand = opcode & 0b0000_0111;
    let register = R8::from(operand).to_string();
    let bit_index = (opcode & 0b0011_1000) >> 3;
    format!("SET {},{}", bit_index, register)
}

pub fn daa() -> String {
    "DAA".to_string()
}

pub fn cpl() -> String {
    "CPL".to_string()
}

pub fn scf() -> String {
    "SCF".to_string()
}

pub fn ccf() -> String {
    "CCF".to_string()
}

pub fn nop() -> String {
    "NOP".to_string()
}

pub fn stop() -> String {
    "STOP".to_string()
}

pub fn halt() -> String {
    "HALT".to_string()
}

pub fn di() -> String {
    "DI".to_string()
}

pub fn ei() -> String {
    "EI".to_string()
}

pub fn prefix(next_byte: u8) -> String {
    let opcode = next_byte;
    let operation = (opcode & 0b1100_0000) >> 6;
    match operation {
        0b01 => bit_b3_r8(opcode),
        0b10 => res_b3_r8(opcode),
        0b11 => set_b3_r8(opcode),
        0b00 => {
            let operation = (opcode & 0b0011_1000) >> 3;
            match operation {
                0b000 => rlc_r8(opcode),
                0b001 => rrc_r8(opcode),
                0b010 => rl_r8(opcode),
                0b011 => rr_r8(opcode),
                0b100 => sla_r8(opcode),
                0b101 => sra_r8(opcode),
                0b110 => swap_r8(opcode),
                0b111 => srl_r8(opcode),
                _ => "Instruction not implemented".to_string(),
            }
        }
        _ => "Instruction not implemented".to_string(),
    }
}
