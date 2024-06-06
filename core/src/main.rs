use core::{game_boy::GameBoy, SCREEN_HEIGHT, SCREEN_WIDTH};
use std::env;

const SCALE: u32 = 4;
const WINDOW_WIDTH: u32 = (SCREEN_WIDTH as u32) * SCALE;
const WINDOW_HEIGHT: u32 = (SCREEN_HEIGHT as u32) * SCALE;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        println!("Invalid input");
        return;
    }

    let _cpu = build_game_boy(&args[1], true);
}

fn build_game_boy(filename: &str, dmg: bool) -> Box<GameBoy> {
    let game_boy = match dmg {
        true => GameBoy::new_dmg(filename),
        false => GameBoy::new_cgb(filename),
    };
    Box::new(game_boy)
}
