use self::registers::Registers;

pub mod instructions;
mod registers;

pub struct Cpu {
    registers: Registers,
    fetch_data: u16,
    memory_destination: u16,
    current_opcode: u8,
    halted: bool,
    stepping: bool,
}

pub fn cpu_init() {}
pub fn cpu_step() {}
