use std::env;

use cpu::{
    bus::Bus,
    cartridge::Cartridge,
    cpu::{registers::Registers, Cpu},
};
use utils::Mode;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid input")
    }

    let cartridge = Cartridge::load(&args[1]);
    println!("Cartridge loaded..");

    // refactor this
    let registers = Registers::new(Mode::Monochrome);
    let bus = Bus::new(cartridge);
    let mut cpu = Cpu::new(bus, registers);

    for _ in 0..=30 {
        cpu.cycle();
    }
}
