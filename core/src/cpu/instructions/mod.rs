use std::fmt;

use crate::bus::MemoryAccess;

use super::Cpu;

pub mod arithmetic_logic;
pub mod bit_operations;
pub mod branch;
pub mod load;
pub mod miscellaneous;
pub mod rotate_shift;

#[derive(Debug, PartialEq)]
pub enum R8 {
    B = 0b000,
    C = 0b001,
    D = 0b010,
    E = 0b011,
    H = 0b100,
    L = 0b101,
    HLMem = 0b110,
    A = 0b111,
}

impl From<u8> for R8 {
    fn from(value: u8) -> Self {
        match value {
            0b000 => R8::B,
            0b001 => R8::C,
            0b010 => R8::D,
            0b011 => R8::E,
            0b100 => R8::H,
            0b101 => R8::L,
            0b110 => R8::HLMem,
            0b111 => R8::A,
            _ => panic!("Invalid value was passed"),
        }
    }
}

impl fmt::Display for R8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            R8::A => write!(f, "A"),
            R8::B => write!(f, "B"),
            R8::C => write!(f, "C"),
            R8::D => write!(f, "D"),
            R8::E => write!(f, "E"),
            R8::H => write!(f, "H"),
            R8::L => write!(f, "L"),
            R8::HLMem => write!(f, "(HL)"),
        }
    }
}

impl R8 {
    fn read_r8(&self, cpu: &Cpu) -> u8 {
        match self {
            R8::A => cpu.registers.a,
            R8::B => cpu.registers.b,
            R8::C => cpu.registers.c,
            R8::D => cpu.registers.d,
            R8::E => cpu.registers.e,
            R8::H => cpu.registers.h,
            R8::L => cpu.registers.l,
            R8::HLMem => cpu.read_8(cpu.registers.hl()),
        }
    }

    fn write_r8(&self, cpu: &mut Cpu, value: u8) {
        match self {
            R8::A => cpu.registers.a = value,
            R8::B => cpu.registers.b = value,
            R8::C => cpu.registers.c = value,
            R8::D => cpu.registers.d = value,
            R8::E => cpu.registers.e = value,
            R8::H => cpu.registers.h = value,
            R8::L => cpu.registers.l = value,
            R8::HLMem => cpu.write_8(cpu.registers.hl(), value),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum R16 {
    BC = 0b00,
    DE = 0b01,
    HL = 0b10,
    SP = 0b11,
}

impl From<u8> for R16 {
    fn from(value: u8) -> R16 {
        match value {
            0b00 => R16::BC,
            0b01 => R16::DE,
            0b10 => R16::HL,
            0b11 => R16::SP,
            _ => panic!("Invalid value was passed"),
        }
    }
}

impl fmt::Display for R16 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            R16::BC => write!(f, "BC"),
            R16::DE => write!(f, "DE"),
            R16::HL => write!(f, "HL"),
            R16::SP => write!(f, "SP"),
        }
    }
}

impl R16 {
    fn read_r16(&self, cpu: &Cpu) -> u16 {
        match self {
            R16::BC => cpu.registers.bc(),
            R16::DE => cpu.registers.de(),
            R16::HL => cpu.registers.hl(),
            R16::SP => cpu.registers.sp,
        }
    }

    fn write_r16(&self, cpu: &mut Cpu, value: u16) {
        match self {
            R16::BC => cpu.registers.set_bc(value),
            R16::DE => cpu.registers.set_de(value),
            R16::HL => cpu.registers.set_hl(value),
            R16::SP => cpu.registers.sp = value,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum R16Stack {
    BC = 0b00,
    DE = 0b01,
    HL = 0b10,
    AF = 0b11,
}

impl From<u8> for R16Stack {
    fn from(value: u8) -> R16Stack {
        match value {
            0b00 => R16Stack::BC,
            0b01 => R16Stack::DE,
            0b10 => R16Stack::HL,
            0b11 => R16Stack::AF,
            _ => panic!("Invalid value was passed"),
        }
    }
}

impl fmt::Display for R16Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            R16Stack::BC => write!(f, "BC"),
            R16Stack::DE => write!(f, "DE"),
            R16Stack::HL => write!(f, "HL"),
            R16Stack::AF => write!(f, "AF"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum R16Memory {
    BC = 0b00,
    DE = 0b01,
    HLI = 0b10,
    HLD = 0b11,
}

impl From<u8> for R16Memory {
    fn from(value: u8) -> R16Memory {
        match value {
            0b00 => R16Memory::BC,
            0b01 => R16Memory::DE,
            0b10 => R16Memory::HLI,
            0b11 => R16Memory::HLD,
            _ => panic!("Invalid value was passed"),
        }
    }
}

impl fmt::Display for R16Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            R16Memory::BC => write!(f, "BC"),
            R16Memory::DE => write!(f, "DE"),
            R16Memory::HLI => write!(f, "HL+"),
            R16Memory::HLD => write!(f, "HL-"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Condition {
    NZ = 0b00,
    Z = 0b01,
    NC = 0b10,
    C = 0b11,
}

impl From<u8> for Condition {
    fn from(value: u8) -> Condition {
        match value {
            0b000 => Condition::NZ,
            0b001 => Condition::Z,
            0b010 => Condition::NC,
            0b011 => Condition::C,
            _ => panic!("Invalid value was passed"),
        }
    }
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Condition::NZ => write!(f, "NZ"),
            Condition::Z => write!(f, "Z"),
            Condition::NC => write!(f, "NC"),
            Condition::C => write!(f, "C"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    None,
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
    JrImm8,
    JrCondImm8,
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
    AddSpImm8,
    LdHlSpPlusImm8,
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
            0x18 => Instruction::JrImm8,
            0x20 | 0x28 | 0x30 | 0x38 => Instruction::JrCondImm8,
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
            0xE8 => Instruction::AddSpImm8,
            0xF8 => Instruction::LdHlSpPlusImm8,
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
    pub fn disassemble(&self, opcode: u8, next_byte: u8) -> String {
        match self {
            Instruction::LdR16Imm16 => {
                let destination = (opcode & 0b0011_0000) >> 4;
                let register = R16::from(destination).to_string();
                format!("LD {register},u16")
            }
            Instruction::LdR16MemA => {
                let destination = (opcode & 0b0011_0000) >> 4;
                let register = R16Memory::from(destination).to_string();
                format!("LD ({register}),A")
            }
            Instruction::LdAR16Mem => {
                let source = (opcode & 0b0011_0000) >> 4;
                let register = R16Memory::from(source).to_string();
                format!("LD A,({register})")
            }
            Instruction::LdImm16Sp => "LD u16,SP".to_string(),
            Instruction::LdR8Imm8 => {
                let destination = (opcode & 0b0011_1000) >> 3;
                let register = R8::from(destination).to_string();
                format!("LD {register},u8")
            }
            Instruction::LdR8R8 => {
                let destination = (opcode & 0b0011_1000) >> 3;
                let source = opcode & 0b0000_0111;
                let register1 = R8::from(destination).to_string();
                let register2 = R8::from(source).to_string();
                format!("LD {register1},{register2}")
            }
            Instruction::LdhCMemA => "LD (FF00+C),A".to_string(),
            Instruction::LdhImm8MemA => "LD (FF00+u8),A".to_string(),
            Instruction::LdImm16MemA => "LD (u16),A".to_string(),
            Instruction::LdhACMem => "LD A,(FF00+C)".to_string(),
            Instruction::LdhAImm8Mem => "LD A,(FF00+u8)".to_string(),
            Instruction::LdAImm16Mem => "LD A,(u16)".to_string(),
            Instruction::LdHlSpPlusImm8 => "LD HL,SP+i8".to_string(),
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
            Instruction::AddSpImm8 => "ADD SP,u8".to_string(),
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
            Instruction::AddAImm8 => "ADD A,u8".to_string(),
            Instruction::AdcAImm8 => "ADC A,u8".to_string(),
            Instruction::SubAImm8 => "SUB A,u8".to_string(),
            Instruction::SbcAImm8 => "SBC A,u8".to_string(),
            Instruction::AndAImm8 => "AND A,u8".to_string(),
            Instruction::XorAImm8 => "XOR A,u8".to_string(),
            Instruction::OrAImm8 => "OR A,u8".to_string(),
            Instruction::CpAImm8 => "CP A,u8".to_string(),
            Instruction::Rlca => "RLCA".to_string(),
            Instruction::Rrca => "RRCA".to_string(),
            Instruction::Rla => "RLA".to_string(),
            Instruction::Rra => "RRA".to_string(),
            Instruction::JrImm8 => "JR i8".to_string(),
            Instruction::JrCondImm8 => {
                let cond = (opcode & 0b0001_1000) >> 3;
                let cond = Condition::from(cond).to_string();
                format!("JR {cond},i8")
            }
            Instruction::JpCondImm16 => {
                let cond = (opcode & 0b0001_1000) >> 3;
                let cond = Condition::from(cond).to_string();
                format!("JP {cond},u16")
            }
            Instruction::JpImm16 => "JP u16".to_string(),
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
                format!("CALL {cond},u16")
            }
            Instruction::CallImm16 => "CALL u16".to_string(),
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
            Instruction::None => "Instruction not implemented".to_string(),
        }
    }
}
