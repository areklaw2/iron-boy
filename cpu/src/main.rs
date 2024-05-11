use std::env;

use cpu::{cartridge::Cartridge, cpu::instructions};

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid input")
    }

    let _cartridge = Cartridge::load(&args[1]);

    let ins = instructions::instruction_name(&instructions::InstructionType::ADC);
    println!("{}", ins)
}
