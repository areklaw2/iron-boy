use std::{collections::HashMap, panic::Location};

use crate::{
    bus::{Bus, Memory},
    opcodes::{OpCode, CB_PREFIXED_OPCODES_MAP, UNPREFIXED_OPCODES_MAP},
    registers::Registers,
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
            code => panic!("Code {:#04X} not implemented", code),
        }
    }

    fn execute_cb(&self, opcode: u8) {
        todo!()
    }

    pub fn run(&self) {
        todo!()
    }
}
