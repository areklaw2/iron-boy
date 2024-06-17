# Iron Boy

A GameBoy emulator, written in rust.

In my quest to build a GBA emulator in rust, I made a Game Boy emulator in rust.

## Todo

- [ ] MBC1
- [ ] MBC2
- [ ] MBC3
- [ ] MBC5
- [ ] Saving
- [ ] Refactor sound to use Sdl2 instead of Cpal and blip_buf
- [ ] Refactor video to just expose screen buffers to sdl2
- [ ] Logging
- [ ] Debugger
- [ ] Tile Map window in debugger
- [ ] Make debug console with Ratatui
- [ ] Build Test Suite
- [ ] Add CLI for start up and testing
- [ ] Test Timings
- [ ] Refactor JoyPad and Lcd
- [ ] Option to use green colors
- [ ] Error Handling
- [ ] CGB support

## Tests

### Blargg's tests

| Test           | Status  |
| -------------- | ------- |
| cgb sound      | N/A\*   |
| cpu instrs     | :+1:    |
| dmg sound      | TODO    |
| instr timing   | :+1:    |
| interrupt time | N/A\*   |
| mem timing     | failing |
| mem timing 2   | failing |
| oam bug        | failing |

\* Can not test until CGB is supported.

### DMG Acid test

| Test     | Status |
| -------- | ------ |
| DMG Acid | :+1:   |
