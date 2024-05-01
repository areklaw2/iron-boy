use crate::{
    bus::{self, Bus, Memory},
    opcodes::OpCode,
    registers::{CpuFlag, Registers},
};

use super::Instructions;

pub struct RsbInstructions {}

impl Instructions for RsbInstructions {}

impl RsbInstructions {
    pub fn new() -> Self {
        RsbInstructions {}
    }

    pub fn rlca(&mut self, opcode: &OpCode, registers: &mut Registers) -> u8 {
        registers.set_flag(CpuFlag::ZERO, false);
        registers.set_flag(CpuFlag::SUBRACTION, false);
        registers.set_flag(CpuFlag::HALF_CARRY, false);
        registers.set_flag(CpuFlag::CARRY, registers.a & 0x80 == 0x80);

        let last_bit = if registers.f.contains(CpuFlag::CARRY) { 0x01 } else { 0 };

        registers.a = registers.a << 1 | last_bit;

        opcode.tcycles.0
    }

    pub fn rlc(&mut self, opcode: &OpCode, registers: &mut Registers, bus: &mut Bus) -> u8 {
        registers.set_flag(CpuFlag::SUBRACTION, false);
        registers.set_flag(CpuFlag::HALF_CARRY, false);

        let operands = self.get_operands(opcode.mnemonic);
        match operands {
            "B" => {
                registers.set_flag(CpuFlag::CARRY, registers.b & 0x80 == 0x80);
                let last_bit = if registers.f.contains(CpuFlag::CARRY) { 0x01 } else { 0 };
                registers.b = registers.b << 1 | last_bit;
                registers.set_flag(CpuFlag::ZERO, registers.b == 0);
            }
            "C" => {
                registers.set_flag(CpuFlag::CARRY, registers.c & 0x80 == 0x80);
                let last_bit = if registers.f.contains(CpuFlag::CARRY) { 0x01 } else { 0 };
                registers.c = registers.c << 1 | last_bit;
                registers.set_flag(CpuFlag::ZERO, registers.c == 0);
            }
            "D" => {
                registers.set_flag(CpuFlag::CARRY, registers.d & 0x80 == 0x80);
                let last_bit = if registers.f.contains(CpuFlag::CARRY) { 0x01 } else { 0 };
                registers.d = registers.d << 1 | last_bit;
                registers.set_flag(CpuFlag::ZERO, registers.d == 0);
            }
            "E" => {
                registers.set_flag(CpuFlag::CARRY, registers.e & 0x80 == 0x80);
                let last_bit = if registers.f.contains(CpuFlag::CARRY) { 0x01 } else { 0 };
                registers.e = registers.e << 1 | last_bit;
                registers.set_flag(CpuFlag::ZERO, registers.e == 0);
            }
            "H" => {
                registers.set_flag(CpuFlag::CARRY, registers.h & 0x80 == 0x80);
                let last_bit = if registers.f.contains(CpuFlag::CARRY) { 0x01 } else { 0 };
                registers.h = registers.h << 1 | last_bit;
                registers.set_flag(CpuFlag::ZERO, registers.h == 0);
            }
            "L" => {
                registers.set_flag(CpuFlag::CARRY, registers.l & 0x80 == 0x80);
                let last_bit = if registers.f.contains(CpuFlag::CARRY) { 0x01 } else { 0 };
                registers.l = registers.l << 1 | last_bit;
                registers.set_flag(CpuFlag::ZERO, registers.l == 0);
            }
            "(HL)" => {
                registers.set_flag(CpuFlag::CARRY, bus.mem_read(registers.hl()) & 0x80 == 0x80);
                let last_bit = if registers.f.contains(CpuFlag::CARRY) { 0x01 } else { 0 };
                bus.mem_write(registers.hl(), bus.mem_read(registers.hl()) << 1 | last_bit);
                registers.set_flag(CpuFlag::ZERO, registers.e == 0);
            }
            "A" => {
                registers.set_flag(CpuFlag::CARRY, registers.a & 0x80 == 0x80);
                let last_bit = if registers.f.contains(CpuFlag::CARRY) { 0x01 } else { 0 };
                registers.a = registers.a << 1 | last_bit;
                registers.set_flag(CpuFlag::ZERO, registers.a == 0);
            }
            op => panic!("Operands not valid: {op}"),
        }

        opcode.tcycles.0
    }

    pub fn rla(&mut self, opcode: &OpCode, registers: &mut Registers) -> u8 {
        registers.set_flag(CpuFlag::ZERO, false);
        registers.set_flag(CpuFlag::SUBRACTION, false);
        registers.set_flag(CpuFlag::HALF_CARRY, false);

        let last_bit = if registers.f.contains(CpuFlag::CARRY) { 0x01 } else { 0 };

        registers.set_flag(CpuFlag::CARRY, registers.a & 0x80 == 0x80);
        registers.a = registers.a << 1 | last_bit;

        opcode.tcycles.0
    }

    pub fn rrca(&mut self, opcode: &OpCode, registers: &mut Registers) -> u8 {
        registers.set_flag(CpuFlag::ZERO, false);
        registers.set_flag(CpuFlag::SUBRACTION, false);
        registers.set_flag(CpuFlag::HALF_CARRY, false);
        registers.set_flag(CpuFlag::CARRY, registers.a & 0x01 == 0x01);

        let first_bit = if registers.f.contains(CpuFlag::CARRY) { 0x80 } else { 0 };

        registers.a = first_bit | registers.a >> 1;

        opcode.tcycles.0
    }

    pub fn rra(&mut self, opcode: &OpCode, registers: &mut Registers) -> u8 {
        registers.set_flag(CpuFlag::ZERO, false);
        registers.set_flag(CpuFlag::SUBRACTION, false);
        registers.set_flag(CpuFlag::HALF_CARRY, false);

        let first_bit = if registers.f.contains(CpuFlag::CARRY) { 0x80 } else { 0 };

        registers.set_flag(CpuFlag::CARRY, registers.a & 0x01 == 0x01);
        registers.a = first_bit | registers.a >> 1;

        opcode.tcycles.0
    }
}
