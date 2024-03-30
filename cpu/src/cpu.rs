use std::collections::HashMap;

use crate::{
    bus::{Bus, Memory},
    opcode::{OpCode, CB_PREFIXED_OPCODES_MAP, UNPREFIXED_OPCODES_MAP},
    register::Registers,
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

    fn fetch(&mut self) -> u8 {
        let opcode = self.mem_read(self.registers.pc);
        let _ = self.registers.pc.wrapping_add(1);
        opcode
    }

    fn handle_interrupt(&mut self) {
        todo!()
    }

    fn execute(&self, opcode: OpCode) -> u8 {
        match opcode.value {
            0x00 => opcode.tcycles.0,
            code => panic!("Code {:2X} not implemented", code),
        }
    }

    fn execute_cb(&self, opcode: u8) {
        todo!()
    }

    pub fn run(&self) {
        todo!()
    }
}
