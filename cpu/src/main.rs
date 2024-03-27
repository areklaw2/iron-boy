use cpu::opcode::{self, OpCode};

fn main() {
    println!("{:?}", *opcode::CB_PREFIXED_OPCODE_MAP)
}
