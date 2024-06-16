# Iron Boy

A GameBoy emulator, written in rust.

In my quest to build a GBA emulator in rust, I made a Game Boy emulator in rust.

## Todo

- [ ] Do Sound
- [ ] Cartridge Types
- [ ] Saving
- [ ] Debugger
- [ ] Make debug console with Ratatui
- [ ] Build Test Suite
- [ ] Add CLI for start up and testing
- [ ] Test Timings
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

### Mooneye GB acceptance tests

| Test                    | mooneye-gb      |
| ----------------------- | --------------- |
| add sp e timing         | :x: (deadlock?) |
| boot div dmg0           | :x:             |
| boot div dmgABCmgb      | :x:             |
| boot div S              | :x:             |
| boot div2 S             | :x:             |
| boot hwio dmg0          | :x:             |
| boot hwio dmgABCmgb     | :x:             |
| boot hwio S             | :+1:            |
| boot regs dmg0          | :+1:            |
| boot regs dmgABC        | :+1:            |
| boot regs mgb           | :+1:            |
| boot regs sgb           | :+1:            |
| boot regs sgb2          | :+1:            |
| call timing             | :+1:            |
| call timing2            | :+1:            |
| call cc_timing          | :+1:            |
| call cc_timing2         | :+1:            |
| di timing GS            | :+1:            |
| div timing              | :+1:            |
| ei sequence             | :+1:            |
| ei timing               | :+1:            |
| halt ime0 ei            | :+1:            |
| halt ime0 nointr_timing | :+1:            |
| halt ime1 timing        | :+1:            |
| halt ime1 timing2 GS    | :+1:            |
| if ie registers         | :+1:            |
| intr timing             | :+1:            |
| jp timing               | :+1:            |
| jp cc timing            | :+1:            |
| ld hl sp e timing       | :+1:            |
| oam dma_restart         | :+1:            |
| oam dma start           | :+1:            |
| oam dma timing          | :+1:            |
| pop timing              | :+1:            |
| push timing             | :+1:            |
| rapid di ei             | :+1:            |
| ret timing              | :+1:            |
| ret cc timing           | :+1:            |
| reti timing             | :+1:            |
| reti intr timing        | :+1:            |
| rst timing              | :+1:            |

#### Bits (unusable bits in memory and registers)

| Test           | mooneye-gb |
| -------------- | ---------- |
| mem oam        | :+1:       |
| reg f          | :+1:       |
| unused_hwio GS | :+1:       |

#### Instructions

| Test | mooneye-gb |
| ---- | ---------- |
| daa  | :+1:       |

#### Interrupt handling

| Test    | mooneye-gb |
| ------- | ---------- |
| ie push | :+1:       |

#### OAM DMA

| Test       | mooneye-gb |
| ---------- | ---------- |
| basic      | :+1:       |
| reg_read   | :+1:       |
| sources GS | :+1:       |

#### PPU

| Test                        | mooneye-gb |
| --------------------------- | ---------- |
| hblank ly scx timing GS     | :+1:       |
| intr 1 2 timing GS          | :+1:       |
| intr 2 0 timing             | :+1:       |
| intr 2 mode0 timing         | :+1:       |
| intr 2 mode3 timing         | :+1:       |
| intr 2 oam ok timing        | :+1:       |
| intr 2 mode0 timing sprites | :x:        |
| lcdon timing GS             | :x:        |
| lcdon write timing GS       | :x:        |
| stat irq blocking           | :x:        |
| stat lyc onoff              | :x:        |
| vblank stat intr GS         | :+1:       |

#### Serial

| Test                      | mooneye-gb |
| ------------------------- | ---------- |
| boot sclk align dmgABCmgb | :x:        |

#### Timer

| Test                 | mooneye-gb |
| -------------------- | ---------- |
| div write            | :+1:       |
| rapid toggle         | :+1:       |
| tim00 div trigger    | :+1:       |
| tim00                | :+1:       |
| tim01 div trigger    | :+1:       |
| tim01                | :+1:       |
| tim10 div trigger    | :+1:       |
| tim10                | :+1:       |
| tim11 div trigger    | :+1:       |
| tim11                | :+1:       |
| tima reload          | :+1:       |
| tima write reloading | :+1:       |
| tma write reloading  | :+1:       |

### Mooneye GB emulator-only tests

#### MBC1

| Test              | mooneye-gb |
| ----------------- | ---------- |
| bits bank1        | :+1:       |
| bits bank2        | :+1:       |
| bits mode         | :+1:       |
| bits ramg         | :+1:       |
| rom 512kb         | :+1:       |
| rom 1Mb           | :+1:       |
| rom 2Mb           | :+1:       |
| rom 4Mb           | :+1:       |
| rom 8Mb           | :+1:       |
| rom 16Mb          | :+1:       |
| ram 64kb          | :+1:       |
| ram 256kb         | :+1:       |
| multicart rom 8Mb | :+1:       |

#### MBC2

| Test        | mooneye-gb |
| ----------- | ---------- |
| bits ramg   | :+1:       |
| bits romb   | :+1:       |
| bits unused | :+1:       |
| rom 512kb   | :+1:       |
| rom 1Mb     | :+1:       |
| rom 2Mb     | :+1:       |
| ram         | :+1:       |

#### MBC5

| Test      | mooneye-gb |
| --------- | ---------- |
| rom 512kb | :+1:       |
| rom 1Mb   | :+1:       |
| rom 2Mb   | :+1:       |
| rom 4Mb   | :+1:       |
| rom 8Mb   | :+1:       |
| rom 16Mb  | :+1:       |
| rom 32Mb  | :+1:       |
| rom 64Mb  | :+1:       |

### Mooneye GB manual tests

| Test            | mooneye-gb |
| --------------- | ---------- |
| sprite priority | :+1:       |

### Mooneye GB misc tests

| Test              | mooneye-gb |
| ----------------- | ---------- |
| boot div A        |            |
| boot div cgb0     |            |
| boot div cgbABCDE |            |
| boot hwio C       |            |
| boot regs A       |            |
| boot regs cgb     |            |

#### Bits

| Test          | mooneye-gb |
| ------------- | ---------- |
| unused hwio C |            |

#### PPU

| Test               | mooneye-gb |
| ------------------ | ---------- |
| vblank stat intr C |            |
