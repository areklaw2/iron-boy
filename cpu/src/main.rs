use cpu::{
    bus::Bus,
    disassembler::Disassembler,
    opcodes,
    registers::{self, Registers},
};

fn main() {
    let registers = Registers::new(utils::Mode::Monochrome);
    let bus = Bus::new();
    let disassembler = Disassembler::new(registers, bus);
}
