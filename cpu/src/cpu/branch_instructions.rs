use crate::{
    bus::Bus,
    opcodes::OpCode,
    registers::{CpuFlag, Registers},
};

use super::{ImeState, Instructions};

pub struct BranchInstructions {}

impl Instructions for BranchInstructions {}

impl BranchInstructions {
    pub fn new() -> Self {
        BranchInstructions {}
    }

    pub fn jr(&mut self, opcode: &OpCode, registers: &mut Registers, bus: &mut Bus) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let jump = ((registers.pc as i32) + (Self::fetch_byte(registers, bus) as i32)) as u16;
        match operands {
            "i8" => {
                registers.pc = jump;
                return opcode.tcycles.0;
            }
            "NZ,i8" => {
                if !registers.f.contains(CpuFlag::ZERO) {
                    registers.pc = jump;
                    return opcode.tcycles.1;
                }
            }
            "Z,i8" => {
                if registers.f.contains(CpuFlag::ZERO) {
                    registers.pc = jump;
                    return opcode.tcycles.1;
                }
            }
            "NC,i8" => {
                if !registers.f.contains(CpuFlag::CARRY) {
                    registers.pc = jump;
                    return opcode.tcycles.1;
                }
            }
            "C,i8" => {
                if registers.f.contains(CpuFlag::CARRY) {
                    registers.pc = jump;
                    return opcode.tcycles.1;
                }
            }
            op => panic!("Operands not valid: {op}"),
        }

        registers.pc += 1;
        opcode.tcycles.0
    }

    pub fn jp(&mut self, opcode: &OpCode, registers: &mut Registers, bus: &mut Bus) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        match operands {
            "NZ,u16" => {
                if !registers.f.contains(CpuFlag::ZERO) {
                    registers.pc = Self::fetch_word(registers, bus);
                    return opcode.tcycles.1;
                }
            }
            "u16" => {
                registers.pc = Self::fetch_word(registers, bus);
                return opcode.tcycles.0;
            }
            "Z,u16" => {
                if registers.f.contains(CpuFlag::ZERO) {
                    registers.pc = Self::fetch_word(registers, bus);
                    return opcode.tcycles.1;
                }
            }
            "NC,u16" => {
                if !registers.f.contains(CpuFlag::CARRY) {
                    registers.pc = Self::fetch_word(registers, bus);
                    return opcode.tcycles.1;
                }
            }
            "C,u16" => {
                if registers.f.contains(CpuFlag::CARRY) {
                    registers.pc = Self::fetch_word(registers, bus);
                    return opcode.tcycles.1;
                }
            }
            op => panic!("Operands not valid: {op}"),
        }

        registers.pc += 2;
        opcode.tcycles.0
    }

    pub fn ret(&mut self, opcode: &OpCode, registers: &mut Registers, bus: &mut Bus, ei: &mut ImeState) -> u8 {
        let operands = match self.get_operands(opcode.mnemonic) {
            operand if operand == "" && opcode.mnemonic == "RET" => "RET",
            operand if operand == "" && opcode.mnemonic == "RETI" => "RETI",
            operand => operand,
        };

        match operands {
            "NZ" => {
                if !registers.f.contains(CpuFlag::ZERO) {
                    registers.pc = Self::pop_stack(registers, bus);
                    return opcode.tcycles.1;
                }
            }
            "Z" => {
                if registers.f.contains(CpuFlag::ZERO) {
                    registers.pc = Self::pop_stack(registers, bus);
                    return opcode.tcycles.1;
                }
            }
            "RET" => registers.pc = Self::pop_stack(registers, bus),
            "NC" => {
                if !registers.f.contains(CpuFlag::CARRY) {
                    registers.pc = Self::pop_stack(registers, bus);
                    return opcode.tcycles.1;
                }
            }
            "C" => {
                if registers.f.contains(CpuFlag::CARRY) {
                    registers.pc = Self::pop_stack(registers, bus);
                    return opcode.tcycles.1;
                }
            }
            "RETI" => {
                registers.pc = Self::pop_stack(registers, bus);
                *ei = ImeState::Staged;
            }
            op => panic!("Operands not valid: {op}"),
        }

        opcode.tcycles.0
    }

    pub fn call(&mut self, opcode: &OpCode, registers: &mut Registers, bus: &mut Bus) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        match operands {
            "NZ,u16" => {
                if !registers.f.contains(CpuFlag::ZERO) {
                    Self::push_stack(registers.pc, registers, bus);
                    registers.pc = Self::fetch_word(registers, bus);
                    return opcode.tcycles.1;
                }
            }
            "Z,u16" => {
                if registers.f.contains(CpuFlag::ZERO) {
                    Self::push_stack(registers.pc, registers, bus);
                    registers.pc = Self::fetch_word(registers, bus);
                    return opcode.tcycles.1;
                }
            }
            "u16" => {
                Self::push_stack(registers.pc, registers, bus);
                registers.pc = Self::fetch_word(registers, bus);
                return opcode.tcycles.0;
            }
            "NC,u16" => {
                if !registers.f.contains(CpuFlag::CARRY) {
                    Self::push_stack(registers.pc, registers, bus);
                    registers.pc = Self::fetch_word(registers, bus);
                    return opcode.tcycles.1;
                }
            }
            "C,u16" => {
                if registers.f.contains(CpuFlag::CARRY) {
                    Self::push_stack(registers.pc, registers, bus);
                    registers.pc = Self::fetch_word(registers, bus);
                    return opcode.tcycles.1;
                }
            }
            op => panic!("Operands not valid: {op}"),
        }

        registers.pc += 2;
        opcode.tcycles.0
    }

    pub fn rst(&mut self, opcode: &OpCode, registers: &mut Registers, bus: &mut Bus) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        match operands {
            "00h" => {
                Self::push_stack(registers.pc, registers, bus);
                registers.pc = 0x00;
            }
            "08h" => {
                Self::push_stack(registers.pc, registers, bus);
                registers.pc = 0x08;
            }
            "10h" => {
                Self::push_stack(registers.pc, registers, bus);
                registers.pc = 0x10;
            }
            "18h" => {
                Self::push_stack(registers.pc, registers, bus);
                registers.pc = 0x18;
            }
            "20h" => {
                Self::push_stack(registers.pc, registers, bus);
                registers.pc = 0x20;
            }
            "28h" => {
                Self::push_stack(registers.pc, registers, bus);
                registers.pc = 0x28;
            }
            "30h" => {
                Self::push_stack(registers.pc, registers, bus);
                registers.pc = 0x30;
            }
            "38h" => {
                Self::push_stack(registers.pc, registers, bus);
                registers.pc = 0x38;
            }
            op => panic!("Operands not valid: {op}"),
        }

        opcode.tcycles.0
    }
}
