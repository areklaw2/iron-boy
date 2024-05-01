use cpu::{
    bus::Bus,
    cpu::Cpu,
    opcodes,
    registers::{self, Registers},
};

fn main() {
    let registers = Registers::new(utils::Mode::Monochrome);
    let bus = Bus::new();
    let Cpu = Cpu::new(registers, bus);
}
