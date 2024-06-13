# Iron Boy

A GameBoy emulator, written in rust.

In my quest to build a GBA emulator in rust, I made a Game Boy emulator in rust.

## Todo

- [ ] Debugger
- [ ] Make debug console with Ratatui
- [ ] Build Test Suite
- [ ] Add CLI for start up and testing
- [ ] Test Timings
- [ ] Do Sound
- [ ] Finish Joypad
- [ ] Option to use green colors
- [ ] Tile Map window
- [ ] Logging
- [ ] Error Handling
- [ ] CGB support

## Tests

### Blargg's tests

| Test           | Status  |
| -------------- | ------- |
| cgb sound      | N/A\*   |
| cpu instrs     | passing |
| dmg sound      | TODO    |
| instr timing   | passing |
| interrupt time | N/A\*   |
| mem timing     | failing |
| mem timing 2   | failing |
| oam bug        | failing |

\* Can not test until CGB is supported.

### Mooneye Test Suite

| Test                  | Statis |
| --------------------- | ------ |
| acceptance\bits       | TODO   |
| acceptance\instr      | TODO   |
| acceptance\interrupts | TODO   |
| acceptance\oam_dma    | TODO   |
| acceptance\ppu        | TODO   |
| acceptance\serial     | TODO   |
| acceptance\timer      | TODO   |
| acceptance\           | TODO   |
| emulator_only\mbc1    | TODO   |
| emulator_only\mbc2    | TODO   |
| emulator_only\mbc5    | TODO   |
| manual-only\          | TODO   |
| other                 | TODO   |
