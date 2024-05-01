use crate::{
    bus::{Bus, Memory},
    opcodes::OpCode,
    registers::{CpuFlag, Registers},
};

use super::Instruction;

pub struct RsbInstructions<'a> {
    registers: &'a mut Registers,
    bus: &'a mut Bus,
}

impl<'a> Instruction for RsbInstructions<'a> {}

impl<'a> RsbInstructions<'a> {
    pub fn new(registers: &mut Registers, bus: &mut Bus) -> Self {
        RsbInstructions { registers, bus }
    }

    pub fn rlca(&mut self, opcode: &OpCode) -> u8 {
        self.registers.set_flag(CpuFlag::ZERO, false);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(CpuFlag::HALF_CARRY, false);
        self.registers.set_flag(CpuFlag::CARRY, self.registers.a & 0x80 == 0x80);

        let last_bit = if self.registers.f.contains(CpuFlag::CARRY) { 0x01 } else { 0 };

        self.registers.a = self.registers.a << 1 | last_bit;

        opcode.tcycles.0
    }

    pub fn rla(&mut self, opcode: &OpCode) -> u8 {
        self.registers.set_flag(CpuFlag::ZERO, false);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(CpuFlag::HALF_CARRY, false);

        let last_bit = if self.registers.f.contains(CpuFlag::CARRY) { 0x01 } else { 0 };

        self.registers.set_flag(CpuFlag::CARRY, self.registers.a & 0x80 == 0x80);
        self.registers.a = self.registers.a << 1 | last_bit;

        opcode.tcycles.0
    }

    pub fn rrca(&mut self, opcode: &OpCode) -> u8 {
        self.registers.set_flag(CpuFlag::ZERO, false);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(CpuFlag::HALF_CARRY, false);
        self.registers.set_flag(CpuFlag::CARRY, self.registers.a & 0x01 == 0x01);

        let first_bit = if self.registers.f.contains(CpuFlag::CARRY) { 0x80 } else { 0 };

        self.registers.a = first_bit | self.registers.a >> 1;

        opcode.tcycles.0
    }

    pub fn rra(&mut self, opcode: &OpCode) -> u8 {
        self.registers.set_flag(CpuFlag::ZERO, false);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(CpuFlag::HALF_CARRY, false);

        let first_bit = if self.registers.f.contains(CpuFlag::CARRY) { 0x80 } else { 0 };

        self.registers.set_flag(CpuFlag::CARRY, self.registers.a & 0x01 == 0x01);
        self.registers.a = first_bit | self.registers.a >> 1;

        opcode.tcycles.0
    }
}
