# Iron Boy

A GameBoy emulator, written in Rust.

In my quest to build a GBA emulator in Rust, I made a Game Boy emulator in Rust.

## Getting Started

Make sure you have the latest version of [Rust](https://www.rust-lang.org/tools/install) installed

### Mac OS with homebrew

`brew install sdl2`

### Ubuntu

`sudo apt update && sudo apt install -y libsdl2`

### Running

`cargo run <rom file path>`

- You can also build a release and run the executable as well

## Keymappings

| Joypad | Keyboard    |
| ------ | ----------- |
| A      | X           |
| B      | Z           |
| Start  | Space       |
| Select | Enter       |
| Up     | Up Arrow    |
| Down   | Down Arrow  |
| Left   | Left Arrow  |
| Right  | Right Arrow |

## Tests

### Blargg's tests

| Test         | Status |
| ------------ | ------ |
| cpu instrs   | :+1:   |
| instr timing | :+1:   |

\* Other Blargg tests failing or can not test due to not supporting CGB at the moment

### DMG Acid test

| Test     | Status |
| -------- | ------ |
| DMG Acid | :+1:   |

## Acknowledgements and Sources

- [Pan Docs](https://gbdev.io/pandocs/About.html)
- [Game Boy Opcodes](https://izik1.github.io/gbops/)
- [gbz80(7) — CPU opcode reference](https://rgbds.gbdev.io/docs/v0.7.0/gbz80.7)
- [Ultimate Game Boy Talk](https://www.youtube.com/watch?v=HyzD8pNlpwI&t=1488s)
- [A journey into GameBoy emulation](https://robertovaccari.com/blog/2020_09_26_gameboy/)
- [GBEDG](https://hacktix.github.io/GBEDG/)
- [GhostSonic Reddit Post on Sound](https://www.reddit.com/r/EmuDev/comments/5gkwi5/comment/dat3zni/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button)
- [Game Boy Sound Emulation](https://nightshade256.github.io/2021/03/27/gb-sound-emulation.html)

### Awesome Emulators

These are some awesome emulators by some really smart people that helped me get to this point.

- [LLG_gbemu](https://github.com/rockytriton/LLD_gbemu)
- [gb-rs](https://github.com/simias/gb-rs)
- [rboy](https://github.com/mvdnes/rboy)
- [gaemboi](https://github.com/mario-hess/gaemboi)
