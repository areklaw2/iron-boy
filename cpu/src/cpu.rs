use std::{collections::HashMap, panic::Location, result};

use bitflags::Flags;

use crate::{
    bus::{Bus, Memory},
    opcodes::{OpCode, CB_PREFIXED_OPCODES_MAP, UNPREFIXED_OPCODES_MAP},
    registers::{CpuFlag, Registers},
};

pub struct Cpu {
    registers: Registers,
    bus: Bus,
}

impl Memory for Cpu {
    fn mem_read(&self, address: u16) -> u8 {
        self.bus.mem_read(address)
    }

    fn mem_read_16(&self, address: u16) -> u16 {
        self.bus.mem_read_16(address)
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        self.bus.mem_write(address, data)
    }

    fn mem_write_16(&mut self, address: u16, data: u16) {
        self.bus.mem_write_16(address, data);
    }
}

impl Cpu {
    pub fn new(registers: Registers, bus: Bus) -> Self {
        Cpu { registers, bus }
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.mem_read(self.registers.pc);
        self.registers.pc += 1;
        byte
    }

    fn fetch_word(&mut self) -> u16 {
        let word = self.mem_read_16(self.registers.pc);
        self.registers.pc += 2;
        word
    }

    fn handle_interrupt(&mut self) {
        todo!()
    }

    fn execute(&mut self, opcode: OpCode) -> u8 {
        // redo the opcodes in such a way that they parse the mnemonics
        match opcode.value {
            0x00 => opcode.tcycles.0,
            0x01 => self.ld_16(opcode),
            0x02 => self.ld_8(opcode),
            0x03 => self.inc_16(opcode),
            0x04 => self.inc_8(opcode),
            0x05 => self.dec_8(opcode),
            0x06 => self.ld_8(opcode),
            0x07 => self.rlca(opcode),
            0x08 => self.ld_16(opcode),
            0x09 => self.add_16(opcode),
            0x0A => self.ld_8(opcode),
            0x0B => self.dec_16(opcode),

            0x0E => self.ld_8(opcode),
            0x11 => self.ld_16(opcode),
            0x12 => self.ld_8(opcode),
            0x16 => self.ld_8(opcode),
            0x1A => self.ld_8(opcode),
            0x1E => self.ld_8(opcode),
            0x21 => self.ld_16(opcode),
            0x22 => self.ld_8(opcode),
            0x26 => self.ld_8(opcode),
            0x2A => self.ld_8(opcode),
            0x2E => self.ld_8(opcode),
            0x31 => self.ld_16(opcode),
            0x32 => self.ld_8(opcode),
            0x36 => self.ld_8(opcode),
            0x3A => self.ld_8(opcode),
            0x3E => self.ld_8(opcode),

            0xF9 => self.ld_16(opcode),
            code => panic!("Code {:#04X} not implemented", code),
        }
    }

    fn execute_cb(&self, opcode: u8) {
        todo!()
    }

    pub fn run(&self) {
        todo!()
    }

    fn get_operands<'a>(&self, mnemonic: &'a str) -> (&'a str, &'a str) {
        let operands: Vec<&str> = mnemonic
            .split_whitespace()
            .nth(1)
            .unwrap_or_default()
            .split(',')
            .collect();

        (operands[0], operands[1])
    }

    fn get_operand<'a>(&self, mnemonic: &'a str) -> &'a str {
        let operand: &str = mnemonic.split_whitespace().nth(1).unwrap_or_default();
        operand
    }

    fn ld_16(&mut self, opcode: OpCode) -> u8 {
        let (operand1, operand2) = self.get_operands(opcode.mnemonic);
        match (operand1, operand2) {
            ("BC", "u16") => {
                let value = self.fetch_word();
                self.registers.set_bc(value)
            }
            ("(u16)", "SP") => {
                let address = self.fetch_word();
                self.mem_write_16(address, self.registers.sp);
            }
            ("DE", "u16") => {
                let value = self.fetch_word();
                self.registers.set_de(value)
            }
            ("HL", "u16") => {
                let value = self.fetch_word();
                self.registers.set_hl(value)
            }
            ("SP", "u16") => self.registers.sp = self.fetch_word(),
            ("SP", "HL") => self.registers.sp = self.registers.hl(),
            (op1, op2) => panic!("Operands not valid: {op1}, {op2}"),
        }
        opcode.tcycles.0
    }

    fn ld_8(&mut self, opcode: OpCode) -> u8 {
        let (operand1, operand2) = self.get_operands(opcode.mnemonic);
        match (operand1, operand2) {
            ("(BC)", "A") => self.mem_write(self.registers.bc(), self.registers.a),
            ("B", "u8") => self.registers.b = self.fetch_byte(),
            ("A", "(BC)") => self.registers.a = self.mem_read(self.registers.bc()),
            ("C", "u8") => self.registers.c = self.fetch_byte(),
            ("(DE)", "A") => self.mem_write(self.registers.de(), self.registers.a),
            ("D", "u8") => self.registers.d = self.fetch_byte(),
            ("A", "(DE)") => self.registers.a = self.mem_read(self.registers.de()),
            ("E", "u8") => self.registers.e = self.fetch_byte(),
            ("(HL+)", "A") => {
                let address = self.registers.increment_hl();
                self.mem_write(address, self.registers.a);
            }
            ("H", "u8") => self.registers.h = self.fetch_byte(),
            ("A", "(HL+)") => {
                let address = self.registers.increment_hl();
                self.registers.a = self.mem_read(address);
            }
            ("L", "u8") => self.registers.l = self.fetch_byte(),
            ("(HL-)", "A") => {
                let address = self.registers.decrement_hl();
                self.mem_write(address, self.registers.a);
            }
            ("(HL)", "u8") => {
                let value = self.fetch_byte();
                self.mem_write(self.registers.hl(), value);
            }
            ("A", "(HL-)") => {
                let address = self.registers.decrement_hl();
                self.registers.a = self.mem_read(address);
            }
            ("A", "u8") => self.registers.a = self.fetch_byte(),

            (op1, op2) => panic!("Operands not valid: {op1}, {op2}"),
        }
        opcode.tcycles.0
    }

    fn inc_16(&mut self, opcode: OpCode) -> u8 {
        let operand = self.get_operand(opcode.mnemonic);
        match operand {
            "BC" => self.registers.set_bc(self.registers.bc().wrapping_add(1)),
            op => panic!("Operands not valid: {op}"),
        };
        opcode.tcycles.0
    }

    fn inc_8(&mut self, opcode: OpCode) -> u8 {
        let operand = self.get_operand(opcode.mnemonic);
        let data;
        let result;
        match operand {
            "B" => {
                data = self.registers.b;
                self.registers.b = self.registers.b.wrapping_add(1);
                result = self.registers.b;
            }
            op => panic!("Operands not valid: {op}"),
        };

        self.registers.set_flag(CpuFlag::ZERO, result == 0);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers
            .set_flag(CpuFlag::HALF_CARRY, (data & 0x0F) + 1 > 0x0F);

        opcode.tcycles.0
    }

    fn dec_16(&mut self, opcode: OpCode) -> u8 {
        let operand = self.get_operand(opcode.mnemonic);
        match operand {
            "BC" => self.registers.set_bc(self.registers.bc().wrapping_sub(1)),
            op => panic!("Operands not valid: {op}"),
        };
        opcode.tcycles.0
    }

    fn dec_8(&mut self, opcode: OpCode) -> u8 {
        let operand = self.get_operand(opcode.mnemonic);
        let data;
        let result;
        match operand {
            "B" => {
                data = self.registers.b;
                self.registers.b = self.registers.b.wrapping_sub(1);
                result = self.registers.b;
            }
            op => panic!("Operands not valid: {op}"),
        };

        self.registers.set_flag(CpuFlag::ZERO, result == 0);
        self.registers.set_flag(CpuFlag::SUBRACTION, true);
        self.registers
            .set_flag(CpuFlag::HALF_CARRY, (data & 0x0F) == 0);

        opcode.tcycles.0
    }

    fn add_16(&mut self, opcode: OpCode) -> u8 {
        let (operand1, operand2) = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        match (operand1, operand2) {
            ("HL", "BC") => {
                (data1, data2) = (self.registers.hl(), self.registers.bc());
                let result = self.registers.hl().wrapping_add(self.registers.bc());
                self.registers.set_hl(result);
            }
            (op1, op2) => panic!("Operands not valid: {op1}, {op2}"),
        };

        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(
            CpuFlag::HALF_CARRY,
            (data1 & 0x07FF) + (data2 & 0x07FF) > 0x07FF,
        );
        self.registers
            .set_flag(CpuFlag::CARRY, data1 > 0xFFFF - data2);
        opcode.tcycles.0
    }

    fn rlca(&mut self, opcode: OpCode) -> u8 {
        self.registers.set_flag(CpuFlag::ZERO, false);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(CpuFlag::HALF_CARRY, false);
        self.registers
            .set_flag(CpuFlag::CARRY, self.registers.a & 0x80 == 0x80);

        let last_bit = if self.registers.f.contains(CpuFlag::CARRY) {
            1
        } else {
            0
        };

        self.registers.a = self.registers.a << 1 | last_bit;

        opcode.tcycles.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::opcodes;
    use utils::Mode;

    fn get_cpu() -> Cpu {
        let registers = Registers::new(Mode::Monochrome);
        let bus = Bus::new();
        let cpu = Cpu::new(registers, bus);
        cpu
    }

    #[test]
    fn execute_nop() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x00];
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 4)
    }

    #[test]
    fn execute_ld_bc_with_u16() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x01];
        cpu.mem_write_16(cpu.registers.pc, 0x4423);

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 12);
        assert_eq!(cpu.registers.bc(), 0x4423);
    }

    #[test]
    fn execute_ld_value_at_bc_with_a() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x02];
        cpu.registers.a = 0x44;

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.mem_read(cpu.registers.bc()), 0x44);
    }

    #[test]
    fn execute_inc_bc() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x03];
        cpu.registers.set_bc(0x4544);

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.bc(), 0x4545);
    }

    #[test]
    fn execute_inc_b() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x04];

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.b = 0x45;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.b, 0x46);
        assert_eq!(cpu.registers.f.bits(), 0b0000_0000);

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.b = 0b0001_1111;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.b, 0x20);
        assert_eq!(cpu.registers.f.bits(), 0b0010_0000);

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.b = 0xFF;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.b, 0);
        assert_eq!(cpu.registers.f.bits(), 0b1010_0000);
    }

    #[test]
    fn execute_dec_b() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x05];

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.b = 0x31;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.b, 0x30);
        assert_eq!(cpu.registers.f.bits(), 0b0100_0000);

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.b = 0x01;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.b, 0);
        assert_eq!(cpu.registers.f.bits(), 0b1100_0000);

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.b = 0;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.b, 0xFF);
        assert_eq!(cpu.registers.f.bits(), 0b0110_0000);
    }

    #[test]
    fn execute_ld_b_with_u8() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x06];
        cpu.mem_write_16(cpu.registers.pc, 0x4423);

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.b, 0x23);
    }

    #[test]
    fn execute_rlca() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x07];

        cpu.registers.a = 0x44;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(cpu.registers.a, 0x88);
        assert_eq!(cpu.registers.f.bits(), 0b0000_0000);
        assert_eq!(tcylcles, 4);

        cpu.registers.a = 0x88;
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(cpu.registers.a, 0x11);
        assert_eq!(cpu.registers.f.bits(), 0b0001_0000);
        assert_eq!(tcylcles, 4);
    }

    #[test]
    fn execute_ld_u16_with_sp() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x08];
        cpu.mem_write_16(cpu.registers.pc, 0x4423);
        cpu.registers.sp = 0x5555;

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 20);
        assert_eq!(cpu.mem_read_16(0x4423), 0x5555);
    }

    #[test]
    fn execute_add_hl_with_bc() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x09];

        cpu.registers.set_hl(0x00FF);
        cpu.registers.set_bc(0x7C00);
        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.hl(), 0x7CFF);
        assert_eq!(cpu.registers.f.bits(), 0b0000_0000);

        cpu.registers.set_hl(0x07FF);
        cpu.registers.set_bc(0x7C00);
        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.hl(), 0x83FF);
        assert_eq!(cpu.registers.f.bits(), 0b0010_0000);

        cpu.registers.set_hl(0x00FF);
        cpu.registers.set_bc(0xFF01);
        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.hl(), 0x0000);
        assert_eq!(cpu.registers.f.bits(), 0b0011_0000);
    }

    #[test]
    fn execute_ld_a_with_value_at_bc() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x0A];
        cpu.mem_write(cpu.registers.bc(), 0x44);

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.a, 0x44);
    }

    #[test]
    fn execute_dec_bc() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x0B];
        cpu.registers.set_bc(0x4544);

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.bc(), 0x4543);
    }

    #[test]
    fn execute_ld_de_with_u16() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x11];
        cpu.mem_write_16(cpu.registers.pc, 0x4423);

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 12);
        assert_eq!(cpu.registers.de(), 0x4423);
    }

    #[test]
    fn execute_ld_hl_with_u16() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x21];
        cpu.mem_write_16(cpu.registers.pc, 0x4423);

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 12);
        assert_eq!(cpu.registers.hl(), 0x4423);
    }

    #[test]
    fn execute_ld_sp_with_u16() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x31];
        cpu.mem_write_16(cpu.registers.pc, 0x4423);

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 12);
        assert_eq!(cpu.registers.sp, 0x4423);
    }

    #[test]
    fn execute_ld_sp_with_hl() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0xF9];
        cpu.registers.set_hl(0x4423);

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.sp, 0x4423);
    }
}
