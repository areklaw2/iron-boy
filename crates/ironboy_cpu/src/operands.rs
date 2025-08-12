use std::fmt;

use ironboy_common::MemoryInterface;

use super::Cpu;

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
    pub fn load<I: MemoryInterface>(&self, cpu: &Cpu<I>) -> u8 {
        match self {
            R8::A => cpu.registers.a,
            R8::B => cpu.registers.b,
            R8::C => cpu.registers.c,
            R8::D => cpu.registers.d,
            R8::E => cpu.registers.e,
            R8::H => cpu.registers.h,
            R8::L => cpu.registers.l,
            R8::HLMem => cpu.load_8(cpu.registers.hl()),
        }
    }

    pub fn write<I: MemoryInterface>(&self, cpu: &mut Cpu<I>, value: u8) {
        match self {
            R8::A => cpu.registers.a = value,
            R8::B => cpu.registers.b = value,
            R8::C => cpu.registers.c = value,
            R8::D => cpu.registers.d = value,
            R8::E => cpu.registers.e = value,
            R8::H => cpu.registers.h = value,
            R8::L => cpu.registers.l = value,
            R8::HLMem => cpu.store_8(cpu.registers.hl(), value),
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
    pub fn load<I: MemoryInterface>(&self, cpu: &Cpu<I>) -> u16 {
        match self {
            R16::BC => cpu.registers.bc(),
            R16::DE => cpu.registers.de(),
            R16::HL => cpu.registers.hl(),
            R16::SP => cpu.registers.sp,
        }
    }

    pub fn store<I: MemoryInterface>(&self, cpu: &mut Cpu<I>, value: u16) {
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

impl R16Stack {
    pub fn load<I: MemoryInterface>(&self, cpu: &Cpu<I>) -> u16 {
        match self {
            R16Stack::BC => cpu.registers.bc(),
            R16Stack::DE => cpu.registers.de(),
            R16Stack::HL => cpu.registers.hl(),
            R16Stack::AF => cpu.registers.af(),
        }
    }

    pub fn store<I: MemoryInterface>(&self, cpu: &mut Cpu<I>, value: u16) {
        match self {
            R16Stack::BC => cpu.registers.set_bc(value),
            R16Stack::DE => cpu.registers.set_de(value),
            R16Stack::HL => cpu.registers.set_hl(value),
            R16Stack::AF => cpu.registers.set_af(value),
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

impl R16Memory {
    pub fn load<I: MemoryInterface>(&self, cpu: &mut Cpu<I>) -> u16 {
        match self {
            R16Memory::BC => cpu.registers.bc(),
            R16Memory::DE => cpu.registers.de(),
            R16Memory::HLI => cpu.registers.increment_hl(),
            R16Memory::HLD => cpu.registers.decrement_hl(),
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
