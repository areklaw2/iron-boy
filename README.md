# Iron Boy

A GameBoy emulator, written in rust.

In my quest to build a GBA emulator in rust, I made a Game Boy emulator in rust.

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
