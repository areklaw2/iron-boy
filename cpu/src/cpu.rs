use std::{collections::HashMap, panic::Location};

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

            0x08 => self.ld_16(opcode),
            0x11 => self.ld_16(opcode),
            0x21 => self.ld_16(opcode),
            0x31 => self.ld_16(opcode),

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

    fn inc_u8(&mut self, data: u8) -> u8 {
        let result = data.wrapping_add(1);
        if result == 0 {
            self.registers.f.insert(CpuFlag::ZERO);
        }
        self.registers.f.remove(CpuFlag::SUBRACTION);
        if (data & 0b0000_1111) + 1 > 0b0000_1111 {
            self.registers.f.insert(CpuFlag::HALF_CARRY);
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::opcodes;
    use bitflags::Flags;
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

    #[test]
    fn execute_ld_bc_with_a() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x02];
        cpu.registers.a = 0x44;

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.mem_read(cpu.registers.bc()), 0x44);
    }
}
