# Iron Boy

Iron Boy is a Game Boy/Game Boy Color emulator written in the Rust Programming Language.

## Motivation

I started this project as a way to work on a both fun and challenging project. I had always had a love for video games and wanted to understand how video game consoles and by extension how computers worked on a deeper level. This project was not only fun but it served as my gateway into computer architecture and low level systems.

## Demo

https://github.com/user-attachments/assets/77a8e0a9-0890-4eb4-aa1e-8904ef2df1bd

## Features

- [x] Game Boy/Game Boy Color hardware support
  - [x] CPU (Sharp SM83)
  - [x] Memory Bus
  - [x] PPU
  - [x] APU
  - [x] Timer
  - [x] Serial Data Transfer (outputs data can’t connect to anything)
  - [x] JoyPad
  - [x] Cartridges
    - [x] MBC1
    - [x] MBC2
    - [x] MBC3 (with Real Time Clock)
    - [x] MBC5 (no rumble)
- [ ] Scheduler based game Loop
- [ ] Game savestates
- [ ] Screenshots
- [ ] Graphics Views
  - [ ] Palette Viewer
  - [ ] Sprite Viewer
  - [ ] Tile Viewer
  - [ ] Backround Only Viewer
  - [ ] Window Only Viewer
- [ ] Audio Channel Visualizer
- [ ] Executed Instruction Log
- [ ] Fast Forwarding

## Getting Started

Make sure you have the latest version of [Rust](https://www.rust-lang.org/tools/install) installed

### Mac OS with homebrew

`brew install sdl2`

### Ubuntu

`sudo apt update && sudo apt install -y libsdl2`

### Running

`cargo run -- <rom file path>`

- You can also build a release and run the executable as well

## Key Mappings

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

| [Blargg's Tests](https://github.com/retrio/gb-test-roms) | IronBoy            |
| -------------------------------------------------------- | ------------------ |
| cpu instrs                                               | :white_check_mark: |
| instr timing                                             | :white_check_mark: |
| mem timing                                               | :white_check_mark: |
| mem timing 2                                             | :white_check_mark: |
| interrupt_time                                           | :white_check_mark: |
| dmg sound                                                | :white_check_mark: |
| cgb sound                                                | :white_check_mark: |
| oam bug                                                  | :x:                |
| halt bug                                                 | :x:                |

[Single Step Tests](https://github.com/SingleStepTests/sm83) :white_check_mark:

[DMG Acid Test](media/dmg-acid.png) :white_check_mark:

[CGB Acid Test](media/cgb-acid.png) :white_check_mark:

## Acknowledgements and Sources

- [Pan Docs](https://gbdev.io/pandocs/About.html)
- [Game Boy Opcodes](https://izik1.github.io/gbops/)
- [gbz80(7) — CPU opcode reference](https://rgbds.gbdev.io/docs/v0.7.0/gbz80.7)
- [Ultimate Game Boy Talk](https://www.youtube.com/watch?v=HyzD8pNlpwI&t=1488s)
- [A journey into GameBoy emulation](https://robertovaccari.com/blog/2020_09_26_gameboy/)
- [GBEDG](https://hacktix.github.io/GBEDG/)
- [GhostSonic Reddit Post on Sound](https://www.reddit.com/r/EmuDev/comments/5gkwi5/comment/dat3zni/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button)
- [Game Boy Sound Emulation](https://nightshade256.github.io/2021/03/27/gb-sound-emulation.html)
- [Game Boy: Complete Technical Reference](https://gekkio.fi/files/gb-docs/gbctr.pdf)
- [LLD Gameboy Emulator Tutorial](https://github.com/rockytriton/LLD_gbemu)

### Awesome Emulators

These are some awesome emulators by some really smart people that helped me get to this point.

- [gb-rs](https://github.com/simias/gb-rs)
- [rboy](https://github.com/mvdnes/rboy)
- [gaemboi](https://github.com/mario-hess/gaemboi)
- [mooneye-gb](https://github.com/Gekkio/mooneye-gb)
