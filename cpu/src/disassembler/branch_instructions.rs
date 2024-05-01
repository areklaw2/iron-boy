use crate::{
    bus::{Bus, Memory},
    opcodes::OpCode,
    registers::{CpuFlag, Registers},
};

use super::{ImeState, Instruction};

pub struct BranchInstructions<'a> {
    registers: &'a mut Registers,
    bus: &'a mut Bus,
}

impl<'a> Instruction for BranchInstructions<'a> {}

impl<'a> BranchInstructions<'a> {
    pub fn new(registers: &mut Registers, bus: &mut Bus) -> Self {
        BranchInstructions { registers, bus }
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.bus.mem_read(self.registers.pc);
        self.registers.pc += 1;
        byte
    }

    fn fetch_word(&mut self) -> u16 {
        let word = self.bus.mem_read_16(self.registers.pc);
        self.registers.pc += 2;
        word
    }

    fn pop_stack(&mut self) -> u16 {
        let data = self.bus.mem_read_16(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(2);
        data
    }

    fn push_stack(&mut self, data: u16) {
        self.registers.sp = self.registers.sp.wrapping_sub(2);
        self.bus.mem_write_16(self.registers.sp, data);
    }

    pub fn jr(&mut self, opcode: &&OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let jump = ((self.registers.pc as i32) + (self.fetch_byte() as i32)) as u16;
        match operands {
            "i8" => {
                self.registers.pc = jump;
                return opcode.tcycles.0;
            }
            "NZ,i8" => {
                if !self.registers.f.contains(CpuFlag::ZERO) {
                    self.registers.pc = jump;
                    return opcode.tcycles.1;
                }
            }
            "Z,i8" => {
                if self.registers.f.contains(CpuFlag::ZERO) {
                    self.registers.pc = jump;
                    return opcode.tcycles.1;
                }
            }
            "NC,i8" => {
                if !self.registers.f.contains(CpuFlag::CARRY) {
                    self.registers.pc = jump;
                    return opcode.tcycles.1;
                }
            }
            "C,i8" => {
                if self.registers.f.contains(CpuFlag::CARRY) {
                    self.registers.pc = jump;
                    return opcode.tcycles.1;
                }
            }
            op => panic!("Operands not valid: {op}"),
        }

        self.registers.pc += 1;
        opcode.tcycles.0
    }

    pub fn jp(&mut self, opcode: &&OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        match operands {
            "NZ,u16" => {
                if !self.registers.f.contains(CpuFlag::ZERO) {
                    self.registers.pc = self.fetch_word();
                    return opcode.tcycles.1;
                }
            }
            "u16" => {
                self.registers.pc = self.fetch_word();
                return opcode.tcycles.0;
            }
            "Z,u16" => {
                if self.registers.f.contains(CpuFlag::ZERO) {
                    self.registers.pc = self.fetch_word();
                    return opcode.tcycles.1;
                }
            }
            "NC,u16" => {
                if !self.registers.f.contains(CpuFlag::CARRY) {
                    self.registers.pc = self.fetch_word();
                    return opcode.tcycles.1;
                }
            }
            "C,u16" => {
                if self.registers.f.contains(CpuFlag::CARRY) {
                    self.registers.pc = self.fetch_word();
                    return opcode.tcycles.1;
                }
            }
            op => panic!("Operands not valid: {op}"),
        }

        self.registers.pc += 2;
        opcode.tcycles.0
    }

    pub fn ret(&mut self, opcode: &&OpCode, ei: &mut ImeState) -> u8 {
        let operands = match self.get_operands(opcode.mnemonic) {
            operand if operand == "" && opcode.mnemonic == "RET" => "RET",
            operand if operand == "" && opcode.mnemonic == "RETI" => "RETI",
            operand => operand,
        };

        match operands {
            "NZ" => {
                if !self.registers.f.contains(CpuFlag::ZERO) {
                    self.registers.pc = self.pop_stack();
                    return opcode.tcycles.1;
                }
            }
            "Z" => {
                if self.registers.f.contains(CpuFlag::ZERO) {
                    self.registers.pc = self.pop_stack();
                    return opcode.tcycles.1;
                }
            }
            "RET" => self.registers.pc = self.pop_stack(),
            "NC" => {
                if !self.registers.f.contains(CpuFlag::CARRY) {
                    self.registers.pc = self.pop_stack();
                    return opcode.tcycles.1;
                }
            }
            "C" => {
                if self.registers.f.contains(CpuFlag::CARRY) {
                    self.registers.pc = self.pop_stack();
                    return opcode.tcycles.1;
                }
            }
            "RETI" => {
                self.registers.pc = self.pop_stack();
                *ei = ImeState::Staged;
            }
            op => panic!("Operands not valid: {op}"),
        }

        opcode.tcycles.0
    }

    pub fn call(&mut self, opcode: &&OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        match operands {
            "NZ,u16" => {
                if !self.registers.f.contains(CpuFlag::ZERO) {
                    self.push_stack(self.registers.pc);
                    self.registers.pc = self.fetch_word();
                    return opcode.tcycles.1;
                }
            }
            "Z,u16" => {
                if self.registers.f.contains(CpuFlag::ZERO) {
                    self.push_stack(self.registers.pc);
                    self.registers.pc = self.fetch_word();
                    return opcode.tcycles.1;
                }
            }
            "u16" => {
                self.push_stack(self.registers.pc);
                self.registers.pc = self.fetch_word();
                return opcode.tcycles.0;
            }
            "NC,u16" => {
                if !self.registers.f.contains(CpuFlag::CARRY) {
                    self.push_stack(self.registers.pc);
                    self.registers.pc = self.fetch_word();
                    return opcode.tcycles.1;
                }
            }
            "C,u16" => {
                if self.registers.f.contains(CpuFlag::CARRY) {
                    self.push_stack(self.registers.pc);
                    self.registers.pc = self.fetch_word();
                    return opcode.tcycles.1;
                }
            }
            op => panic!("Operands not valid: {op}"),
        }

        self.registers.pc += 2;
        opcode.tcycles.0
    }

    pub fn rst(&mut self, opcode: &&OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        match operands {
            "00h" => {
                self.push_stack(self.registers.pc);
                self.registers.pc = 0x00;
            }
            "08h" => {
                self.push_stack(self.registers.pc);
                self.registers.pc = 0x08;
            }
            "10h" => {
                self.push_stack(self.registers.pc);
                self.registers.pc = 0x10;
            }
            "18h" => {
                self.push_stack(self.registers.pc);
                self.registers.pc = 0x18;
            }
            "20h" => {
                self.push_stack(self.registers.pc);
                self.registers.pc = 0x20;
            }
            "28h" => {
                self.push_stack(self.registers.pc);
                self.registers.pc = 0x28;
            }
            "30h" => {
                self.push_stack(self.registers.pc);
                self.registers.pc = 0x30;
            }
            "38h" => {
                self.push_stack(self.registers.pc);
                self.registers.pc = 0x38;
            }
            op => panic!("Operands not valid: {op}"),
        }

        opcode.tcycles.0
    }
}
