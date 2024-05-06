use cpu::{
    cpu::Cpu,
    mmu::Bus,
    opcode::{self, Opcode},
    registers::{self, Registers},
};

fn main() {
    let opcode = Opcode::new(0x01, "YOP", (1, 1), 1, vec![|| println!("Hello")]);
    for step in opcode.steps {
        step();
    }
}
