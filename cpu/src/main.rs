use std::env;

use cpu::cartridge::Cartridge;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid input")
    }

    let cartridge = Cartridge::load(&args[1]);
}
