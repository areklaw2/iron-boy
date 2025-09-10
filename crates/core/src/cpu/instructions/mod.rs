use crate::cpu::operands::{Condition, R8, R16, R16Memory, R16Stack};

pub mod arithmetic_logic;
pub mod bit_operations;
pub mod branch;
pub mod load;
pub mod miscellaneous;
pub mod rotate_shift;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Nop,
    LdR16Imm16,
    LdR16MemA,
    LdAR16Mem,
    LdImm16Sp,
    IncR16,
    DecR16,
    AddHlR16,
    IncR8,
    DecR8,
    LdR8Imm8,
    Rlca,
    Rrca,
    Rla,
    Rra,
    Daa,
    Cpl,
    Scf,
    Ccf,
    JrSignedImm8,
    JrCondSignedImm8,
    Stop,
    LdR8R8,
    Halt,
    AddAR8,
    AdcAR8,
    SubAR8,
    SbcAR8,
    AndAR8,
    XorAR8,
    OrAR8,
    CpAR8,
    AddAImm8,
    AdcAImm8,
    SubAImm8,
    SbcAImm8,
    AndAImm8,
    XorAImm8,
    OrAImm8,
    CpAImm8,
    RetCond,
    Ret,
    Reti,
    JpCondImm16,
    JpImm16,
    JpHl,
    CallCondImm16,
    CallImm16,
    RstTgt3,
    PopR16Stk,
    PushR16Stk,
    Prefix,
    LdhCMemA,
    LdhImm8MemA,
    LdImm16MemA,
    LdhACMem,
    LdhAImm8Mem,
    LdAImm16Mem,
    AddSpSignedImm8,
    LdHlSpPlusSignedImm8,
    LdSpHl,
    Di,
    Ei,
}

impl From<u8> for Instruction {
    fn from(opcode: u8) -> Instruction {
        match opcode {
            0x00 => Instruction::Nop,
            0x01 | 0x11 | 0x21 | 0x31 => Instruction::LdR16Imm16,
            0x02 | 0x12 | 0x22 | 0x32 => Instruction::LdR16MemA,
            0x03 | 0x13 | 0x23 | 0x33 => Instruction::IncR16,
            0x04 | 0x14 | 0x24 | 0x34 | 0x0C | 0x1C | 0x2C | 0x3C => Instruction::IncR8,
            0x05 | 0x15 | 0x25 | 0x35 | 0x0D | 0x1D | 0x2D | 0x3D => Instruction::DecR8,
            0x06 | 0x16 | 0x26 | 0x36 | 0x0E | 0x1E | 0x2E | 0x3E => Instruction::LdR8Imm8,
            0x10 => Instruction::Stop,
            0x07 => Instruction::Rlca,
            0x17 => Instruction::Rla,
            0x27 => Instruction::Daa,
            0x37 => Instruction::Scf,
            0x08 => Instruction::LdImm16Sp,
            0x18 => Instruction::JrSignedImm8,
            0x20 | 0x28 | 0x30 | 0x38 => Instruction::JrCondSignedImm8,
            0x09 | 0x19 | 0x29 | 0x39 => Instruction::AddHlR16,
            0x0A | 0x1A | 0x2A | 0x3A => Instruction::LdAR16Mem,
            0x0B | 0x1B | 0x2B | 0x3B => Instruction::DecR16,
            0x0F => Instruction::Rrca,
            0x1F => Instruction::Rra,
            0x2F => Instruction::Cpl,
            0x3F => Instruction::Ccf,
            0x40..=0x75 | 0x77..=0x7F => Instruction::LdR8R8,
            0x76 => Instruction::Halt,
            0x80..=0x87 => Instruction::AddAR8,
            0x88..=0x8F => Instruction::AdcAR8,
            0x90..=0x97 => Instruction::SubAR8,
            0x98..=0x9F => Instruction::SbcAR8,
            0xA0..=0xA7 => Instruction::AndAR8,
            0xA8..=0xAF => Instruction::XorAR8,
            0xB0..=0xB7 => Instruction::OrAR8,
            0xB8..=0xBF => Instruction::CpAR8,
            0xC0 | 0xC8 | 0xD0 | 0xD8 => Instruction::RetCond,
            0xE0 => Instruction::LdhImm8MemA,
            0xF0 => Instruction::LdhAImm8Mem,
            0xC1 | 0xD1 | 0xE1 | 0xF1 => Instruction::PopR16Stk,
            0xC2 | 0xCA | 0xD2 | 0xDA => Instruction::JpCondImm16,
            0xE2 => Instruction::LdhCMemA,
            0xF2 => Instruction::LdhACMem,
            0xC3 => Instruction::JpImm16,
            0xF3 => Instruction::Di,
            0xC4 | 0xCC | 0xD4 | 0xDC => Instruction::CallCondImm16,
            0xC5 | 0xD5 | 0xE5 | 0xF5 => Instruction::PushR16Stk,
            0xC6 => Instruction::AddAImm8,
            0xD6 => Instruction::SubAImm8,
            0xE6 => Instruction::AndAImm8,
            0xF6 => Instruction::OrAImm8,
            0xC7 | 0xD7 | 0xE7 | 0xF7 | 0xCF | 0xDF | 0xEF | 0xFF => Instruction::RstTgt3,
            0xE8 => Instruction::AddSpSignedImm8,
            0xF8 => Instruction::LdHlSpPlusSignedImm8,
            0xC9 => Instruction::Ret,
            0xD9 => Instruction::Reti,
            0xE9 => Instruction::JpHl,
            0xF9 => Instruction::LdSpHl,
            0xEA => Instruction::LdImm16MemA,
            0xFA => Instruction::LdAImm16Mem,
            0xCB => Instruction::Prefix,
            0xFB => Instruction::Ei,
            0xCD => Instruction::CallImm16,
            0xCE => Instruction::AdcAImm8,
            0xDE => Instruction::SbcAImm8,
            0xEE => Instruction::XorAImm8,
            0xFE => Instruction::CpAImm8,
            code => panic!("Code {:#04X} not implemented", code),
        }
    }
}

impl Instruction {
    pub fn disassemble(&self, opcode: u8, next_byte: u8, next_word: u16) -> String {
        match self {
            Instruction::LdR16Imm16 => {
                let destination = (opcode & 0b0011_0000) >> 4;
                let register = R16::from(destination).to_string();
                format!("LD {register},{:#04X}", next_word)
            }
            Instruction::LdR16MemA => {
                let destination = (opcode & 0b0011_0000) >> 4;
                let register = R16Memory::from(destination).to_string();
                format!("LD [{register}],A")
            }
            Instruction::LdAR16Mem => {
                let source = (opcode & 0b0011_0000) >> 4;
                let register = R16Memory::from(source).to_string();
                format!("LD A,[{register}]")
            }
            Instruction::LdImm16Sp => format!("LD {:#04X},SP", next_word),
            Instruction::LdR8Imm8 => {
                let destination = (opcode & 0b0011_1000) >> 3;
                let register = R8::from(destination).to_string();
                format!("LD {register},{:#04X}", next_byte)
            }
            Instruction::LdR8R8 => {
                let destination = (opcode & 0b0011_1000) >> 3;
                let source = opcode & 0b0000_0111;
                let register1 = R8::from(destination).to_string();
                let register2 = R8::from(source).to_string();
                format!("LD {register1},{register2}")
            }
            Instruction::LdhCMemA => "LD [FF00+C],A".to_string(),
            Instruction::LdhImm8MemA => format!("LD [FF00+{:#04X}],A", next_byte),
            Instruction::LdImm16MemA => format!("LD [{:#04X}],A", next_word),
            Instruction::LdhACMem => "LD A,[FF00+C]".to_string(),
            Instruction::LdhAImm8Mem => format!("LD A,[FF00+{:#04X}]", next_byte),
            Instruction::LdAImm16Mem => format!("LD A,[{:#04X}]", next_word),
            Instruction::LdHlSpPlusSignedImm8 => format!("LD HL,SP+{:#04X}", next_byte),
            Instruction::LdSpHl => "LD SP,HL".to_string(),
            Instruction::PopR16Stk => {
                let register = (opcode & 0b0011_0000) >> 4;
                let register = R16Stack::from(register).to_string();
                format!("POP {register}")
            }
            Instruction::PushR16Stk => {
                let register = (opcode & 0b0011_0000) >> 4;
                let register = R16Stack::from(register).to_string();
                format!("PUSH {register}")
            }
            Instruction::IncR16 => {
                let operand = (opcode & 0b0011_0000) >> 4;
                let register = R16::from(operand).to_string();
                format!("INC {register}")
            }
            Instruction::IncR8 => {
                let operand = (opcode & 0b0011_1000) >> 3;
                let register = R8::from(operand).to_string();
                format!("INC {register}")
            }
            Instruction::DecR16 => {
                let operand = (opcode & 0b0011_0000) >> 4;
                let register = R16::from(operand);
                format!("DEC {register}")
            }
            Instruction::DecR8 => {
                let operand = (opcode & 0b0011_1000) >> 3;
                let register = R8::from(operand);
                format!("DEC {register}")
            }
            Instruction::Daa => "DAA".to_string(),
            Instruction::Cpl => "CPL".to_string(),
            Instruction::Scf => "SCF".to_string(),
            Instruction::Ccf => "CCF".to_string(),
            Instruction::AddHlR16 => {
                let operand = (opcode & 0b0011_0000) >> 4;
                let register = R16::from(operand).to_string();
                format!("ADD HL,{register}")
            }
            Instruction::AddSpSignedImm8 => "ADD SP,u8".to_string(),
            Instruction::AddAR8 => {
                let operand = opcode & 0b0000_0111;
                let register = R8::from(operand).to_string();
                format!("ADD A,{register}")
            }
            Instruction::AdcAR8 => {
                let operand = opcode & 0b0000_0111;
                let register = R8::from(operand).to_string();
                format!("ADC A,{register}")
            }
            Instruction::SubAR8 => {
                let operand = opcode & 0b0000_0111;
                let register = R8::from(operand).to_string();
                format!("SUB A,{register}")
            }
            Instruction::SbcAR8 => {
                let operand = opcode & 0b0000_0111;
                let register = R8::from(operand).to_string();
                format!("SBC A,{register}")
            }
            Instruction::AndAR8 => {
                let operand = opcode & 0b0000_0111;
                let register = R8::from(operand).to_string();
                format!("AND A,{register}")
            }
            Instruction::XorAR8 => {
                let operand = opcode & 0b0000_0111;
                let register = R8::from(operand).to_string();
                format!("XOR A,{register}")
            }
            Instruction::OrAR8 => {
                let operand = opcode & 0b0000_0111;
                let register = R8::from(operand).to_string();
                format!("OR A,{register}")
            }
            Instruction::CpAR8 => {
                let operand = opcode & 0b0000_0111;
                let register = R8::from(operand).to_string();
                format!("CP A,{register}")
            }
            Instruction::AddAImm8 => format!("ADD A,{:#04X}", next_byte),
            Instruction::AdcAImm8 => format!("ADC A,{:#04X}", next_byte),
            Instruction::SubAImm8 => format!("SUB A,{:#04X}", next_byte),
            Instruction::SbcAImm8 => format!("SBC A,{:#04X}", next_byte),
            Instruction::AndAImm8 => format!("AND A,{:#04X}", next_byte),
            Instruction::XorAImm8 => format!("XOR A,{:#04X}", next_byte),
            Instruction::OrAImm8 => format!("OR A,{:#04X}", next_byte),
            Instruction::CpAImm8 => format!("CP A,{:#04X}", next_byte),
            Instruction::Rlca => "RLCA".to_string(),
            Instruction::Rrca => "RRCA".to_string(),
            Instruction::Rla => "RLA".to_string(),
            Instruction::Rra => "RRA".to_string(),
            Instruction::JrSignedImm8 => format!("JR {:#04X}", next_byte),
            Instruction::JrCondSignedImm8 => {
                let cond = (opcode & 0b0001_1000) >> 3;
                let cond = Condition::from(cond).to_string();
                format!("JR {cond},{:#04X}", next_byte)
            }
            Instruction::JpCondImm16 => {
                let cond = (opcode & 0b0001_1000) >> 3;
                let cond = Condition::from(cond).to_string();
                format!("JP {cond},{:#04X}", next_word)
            }
            Instruction::JpImm16 => format!("JP {:#04X}", next_word),
            Instruction::JpHl => "JP HL".to_string(),
            Instruction::RetCond => {
                let cond = (opcode & 0b0001_1000) >> 3;
                let cond = Condition::from(cond).to_string();
                format!("RET {cond}")
            }
            Instruction::Ret => "RET".to_string(),
            Instruction::Reti => "RETI".to_string(),
            Instruction::CallCondImm16 => {
                let cond = (opcode & 0b0001_1000) >> 3;
                let cond = Condition::from(cond).to_string();
                format!("CALL {cond},{:#04X}", next_word)
            }
            Instruction::CallImm16 => format!("CALL {:#04X}", next_word),
            Instruction::RstTgt3 => {
                let target = ((opcode & 0b0011_1000) >> 3) / 8;
                format!("RST {:02}h", target)
            }
            Instruction::Nop => "NOP".to_string(),
            Instruction::Stop => "STOP".to_string(),
            Instruction::Halt => "HALT".to_string(),
            Instruction::Di => "DI".to_string(),
            Instruction::Ei => "EI".to_string(),
            Instruction::Prefix => {
                let opcode = next_byte;
                let operation = (opcode & 0b1100_0000) >> 6;
                match operation {
                    0b01 => {
                        let operand = opcode & 0b0000_0111;
                        let register = R8::from(operand).to_string();
                        let bit_index = (opcode & 0b0011_1000) >> 3;
                        format!("BIT {},{}", bit_index, register)
                    }
                    0b10 => {
                        let operand = opcode & 0b0000_0111;
                        let register = R8::from(operand).to_string();
                        let bit_index = (opcode & 0b0011_1000) >> 3;
                        format!("RES {},{}", bit_index, register)
                    }
                    0b11 => {
                        let operand = opcode & 0b0000_0111;
                        let register = R8::from(operand).to_string();
                        let bit_index = (opcode & 0b0011_1000) >> 3;
                        format!("SET {},{}", bit_index, register)
                    }
                    0b00 => {
                        let operation = (opcode & 0b0011_1000) >> 3;
                        match operation {
                            0b000 => {
                                let operand = opcode & 0b0000_0111;
                                let register = &R8::from(operand).to_string();
                                format!("RLC {register}")
                            }
                            0b001 => {
                                let operand = opcode & 0b0000_0111;
                                let register = &R8::from(operand).to_string();
                                format!("RRC {register}")
                            }
                            0b010 => {
                                let operand = opcode & 0b0000_0111;
                                let register = &R8::from(operand).to_string();
                                format!("RL {register}")
                            }
                            0b011 => {
                                let operand = opcode & 0b0000_0111;
                                let register = &R8::from(operand).to_string();
                                format!("RR {register}")
                            }
                            0b100 => {
                                let operand = opcode & 0b0000_0111;
                                let register = &R8::from(operand).to_string();
                                format!("SLA {register}")
                            }
                            0b101 => {
                                let operand = opcode & 0b0000_0111;
                                let register = &R8::from(operand).to_string();
                                format!("SRA {register}")
                            }
                            0b110 => {
                                let operand = opcode & 0b0000_0111;
                                let register = &R8::from(operand).to_string();
                                format!("SWAP {register}")
                            }
                            0b111 => {
                                let operand = opcode & 0b0000_0111;
                                let register = &R8::from(operand).to_string();
                                format!("SRL {register}")
                            }
                            _ => "Instruction not implemented".to_string(),
                        }
                    }
                    _ => "Instruction not implemented".to_string(),
                }
            }
        }
    }
}
