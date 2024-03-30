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
            0x01 => {
                let value = self.fetch_word();
                self.registers.set_bc(value);
                opcode.tcycles.0
            }
            0x02 => {
                self.mem_write(self.registers.bc(), self.registers.a);
                opcode.tcycles.0
            }
            0x03 => {
                self.registers.set_bc(self.registers.bc().wrapping_add(1));
                opcode.tcycles.0
            }
            0x04 => {
                self.registers.b = self.inc_u8(self.registers.b);
                opcode.tcycles.0
            }
            code => panic!("Code {:#04X} not implemented", code),
        }
    }

    fn execute_cb(&self, opcode: u8) {
        todo!()
    }

    pub fn run(&self) {
        todo!()
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
    fn execute_ld_bc_with_a() {
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
        cpu.registers.set_bc(0x543E);

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.bc(), 0x543F);
    }

    fn execute_inc_b() {
        let mut cpu = get_cpu();
        let ref opcode = opcodes::UNPREFIXED_OPCODES[0x03];
        cpu.registers.set_bc(0xFF);
        cpu.registers.f = CpuFlag::from_bits_truncate(0);

        let tcylcles = cpu.execute(*opcode);
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.bc(), 0x0);
        assert_eq!(cpu.registers.f.bits(), 0b1000_0000)
    }
}
