# Iron Boy

A GameBoy emulator, written in rust.

In my quest to build a GBA emulator in rust, I made a Game Boy emulator in rust.

## Todo

- [ ] MBC1
- [ ] MBC2
- [ ] MBC3
- [ ] MBC5
- [ ] Saving
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
- [ ] Fix sound pops
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

### Mooneye GB acceptance tests

| Test                    | mooneye-gb |
| ----------------------- | ---------- |
| add sp e timing         | :x:        |
| boot div dmg0           | TODO       |
| boot div dmgABCmgb      | TODO       |
| boot div S              | TODO       |
| boot div2 S             | TODO       |
| boot hwio dmg0          | TODO       |
| boot hwio dmgABCmgb     | TODO       |
| boot hwio S             | TODO       |
| boot regs dmg0          | TODO       |
| boot regs dmgABC        | TODO       |
| boot regs mgb           | TODO       |
| boot regs sgb           | TODO       |
| boot regs sgb2          | TODO       |
| call timing             | TODO       |
| call timing2            | TODO       |
| call cc_timing          | TODO       |
| call cc_timing2         | TODO       |
| di timing GS            | TODO       |
| div timing              | TODO       |
| ei sequence             | TODO       |
| ei timing               | TODO       |
| halt ime0 ei            | TODO       |
| halt ime0 nointr_timing | TODO       |
| halt ime1 timing        | TODO       |
| halt ime1 timing2 GS    | TODO       |
| if ie registers         | TODO       |
| intr timing             | TODO       |
| jp timing               | TODO       |
| jp cc timing            | TODO       |
| ld hl sp e timing       | TODO       |
| oam dma_restart         | TODO       |
| oam dma start           | TODO       |
| oam dma timing          | TODO       |
| pop timing              | TODO       |
| push timing             | TODO       |
| rapid di ei             | TODO       |
| ret timing              | TODO       |
| ret cc timing           | TODO       |
| reti timing             | TODO       |
| reti intr timing        | TODO       |
| rst timing              | TODO       |

#### Bits (unusable bits in memory and registers)

| Test           | mooneye-gb |
| -------------- | ---------- |
| mem oam        | TODO       |
| reg f          | TODO       |
| unused_hwio GS | TODO       |

#### Instructions

| Test | mooneye-gb |
| ---- | ---------- |
| daa  | TODO       |

#### Interrupt handling

| Test    | mooneye-gb |
| ------- | ---------- |
| ie push | TODO       |

#### OAM DMA

| Test       | mooneye-gb |
| ---------- | ---------- |
| basic      | TODO       |
| reg_read   | TODO       |
| sources GS | TODO       |

#### PPU

| Test                        | mooneye-gb |
| --------------------------- | ---------- |
| hblank ly scx timing GS     | TODO       |
| intr 1 2 timing GS          | TODO       |
| intr 2 0 timing             | TODO       |
| intr 2 mode0 timing         | TODO       |
| intr 2 mode3 timing         | TODO       |
| intr 2 oam ok timing        | TODO       |
| intr 2 mode0 timing sprites | :x:        |
| lcdon timing GS             | :x:        |
| lcdon write timing GS       | :x:        |
| stat irq blocking           | :x:        |
| stat lyc onoff              | :x:        |
| vblank stat intr GS         | TODO       |

#### Serial

| Test                      | mooneye-gb |
| ------------------------- | ---------- |
| boot sclk align dmgABCmgb | :x:        |

#### Timer

| Test                 | mooneye-gb |
| -------------------- | ---------- |
| div write            | TODO       |
| rapid toggle         | TODO       |
| tim00 div trigger    | TODO       |
| tim00                | TODO       |
| tim01 div trigger    | TODO       |
| tim01                | TODO       |
| tim10 div trigger    | TODO       |
| tim10                | TODO       |
| tim11 div trigger    | TODO       |
| tim11                | TODO       |
| tima reload          | TODO       |
| tima write reloading | TODO       |
| tma write reloading  | TODO       |

### Mooneye GB emulator-only tests

#### MBC1

| Test              | mooneye-gb |
| ----------------- | ---------- |
| bits bank1        | TODO       |
| bits bank2        | TODO       |
| bits mode         | TODO       |
| bits ramg         | TODO       |
| rom 512kb         | TODO       |
| rom 1Mb           | TODO       |
| rom 2Mb           | TODO       |
| rom 4Mb           | TODO       |
| rom 8Mb           | TODO       |
| rom 16Mb          | TODO       |
| ram 64kb          | TODO       |
| ram 256kb         | TODO       |
| multicart rom 8Mb | TODO       |

#### MBC2

| Test        | mooneye-gb |
| ----------- | ---------- |
| bits ramg   | TODO       |
| bits romb   | TODO       |
| bits unused | TODO       |
| rom 512kb   | TODO       |
| rom 1Mb     | TODO       |
| rom 2Mb     | TODO       |
| ram         | TODO       |

#### MBC5

| Test      | mooneye-gb |
| --------- | ---------- |
| rom 512kb | TODO       |
| rom 1Mb   | TODO       |
| rom 2Mb   | TODO       |
| rom 4Mb   | TODO       |
| rom 8Mb   | TODO       |
| rom 16Mb  | TODO       |
| rom 32Mb  | TODO       |
| rom 64Mb  | TODO       |

### Mooneye GB manual tests

| Test            | mooneye-gb |
| --------------- | ---------- |
| sprite priority | TODO       |

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
