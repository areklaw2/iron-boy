use crate::{
    mmu::Mmu,
    registers::{CpuFlag, Registers},
};

use self::opcode::{CB_PREFIXED_OPCODES, UNPREFIXED_OPCODES};

mod opcode;

pub enum ImeState {
    Disable,
    Enable,
    Staged,
    NoChange,
}

pub struct Cpu {
    registers: Registers,
    mmu: Mmu,
    halted: bool,
    ime: bool,
    ei: ImeState,
    di: ImeState,
}

impl Cpu {
    pub fn new(registers: Registers, mmu: Mmu) -> Self {
        Cpu {
            registers,
            mmu,
            halted: false,
            ime: false,
            ei: ImeState::NoChange,
            di: ImeState::NoChange,
        }
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.mmu.mem_read(self.registers.pc);
        self.registers.pc += 1;
        byte
    }

    fn fetch_word(&mut self) -> u16 {
        let word = self.mmu.mem_read_16(self.registers.pc);
        self.registers.pc += 2;
        word
    }

    fn pop_stack(&mut self) -> u16 {
        let data = self.mmu.mem_read_16(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(2);
        data
    }

    fn push_stack(&mut self, data: u16) {
        self.registers.sp = self.registers.sp.wrapping_sub(2);
        self.mmu.mem_write_16(self.registers.sp, data);
    }

    fn update_ime_state(&mut self) {
        self.ei = match self.ei {
            ImeState::Staged => ImeState::Enable,
            ImeState::Enable => {
                self.ime = true;
                ImeState::NoChange
            }
            _ => ImeState::NoChange,
        };

        self.di = match self.di {
            ImeState::Staged => ImeState::Disable,
            ImeState::Disable => {
                self.ime = false;
                ImeState::NoChange
            }
            _ => ImeState::NoChange,
        };
    }

    fn handle_interrupt(&mut self) -> u8 {
        if !self.ime && !self.halted {
            return 0;
        }

        let requested = self.mmu.mem_read(0xFFFF) & self.mmu.mem_read(0xFF0F);
        if requested == 0 {
            return 0;
        }

        self.halted = false;
        if !self.ime {
            return 0;
        }
        self.ime = false;

        let bits = requested.trailing_zeros();
        if bits >= 5 {
            panic!("Invalid interrupt requested");
        }

        let mut interrupt = self.mmu.mem_read(0xFF0F);
        interrupt &= !(1 << bits);
        self.mmu.mem_write(0xFF0F, interrupt);

        let program_counter = self.registers.pc;
        self.push_stack(program_counter);
        self.registers.pc = 0x0040 | ((bits as u16) << 3);

        16
    }

    pub fn run_cycle(&mut self) -> u8 {
        self.update_ime_state();
        match self.handle_interrupt() {
            0 => {}
            ticks => return ticks,
        }

        if self.halted {
            4
        } else {
            self.execute().1
        }
    }

    fn execute(&mut self) -> (&str, u8) {
        let opcode = self.fetch_byte();
        let (mnemonic, no_branch_tcycles, branch_tcycles) = UNPREFIXED_OPCODES[opcode as usize];
        match opcode {
            0x00 => {}
            0x01 => {
                let data = self.fetch_word();
                self.registers.set_bc(data);
            }
            0x02 => self.mmu.mem_write(self.registers.bc(), self.registers.a),
            0x03 => self.registers.set_bc(self.registers.bc().wrapping_add(1)),
            0x04 => self.registers.b = self.inc_8(self.registers.b),
            0x05 => self.registers.b = self.dec_8(self.registers.b),
            0x06 => self.registers.b = self.fetch_byte(),
            0x07 => {
                self.registers.a = self.rlc(self.registers.a);
                self.registers.set_flag(CpuFlag::Z, false);
            }
            0x08 => {
                let data = self.fetch_word();
                self.mmu.mem_write_16(data, self.registers.sp);
            }
            0x09 => self.add_16(self.registers.bc()),
            0x10 => self.mmu.change_speed(),
            0x0A => self.registers.a = self.mmu.mem_read(self.registers.bc()),
            0x0B => self.registers.set_bc(self.registers.bc().wrapping_sub(1)),
            0x0C => self.registers.c = self.inc_8(self.registers.c),
            0x0D => self.registers.c = self.dec_8(self.registers.c),
            0x0E => self.registers.c = self.fetch_byte(),
            0x0F => {
                self.registers.a = self.rrc(self.registers.a);
                self.registers.set_flag(CpuFlag::Z, false);
            }
            0x11 => {
                let data = self.fetch_word();
                self.registers.set_de(data);
            }
            0x12 => self.mmu.mem_write(self.registers.de(), self.registers.a),
            0x13 => self.registers.set_de(self.registers.de().wrapping_add(1)),
            0x14 => self.registers.d = self.inc_8(self.registers.d),
            0x15 => self.registers.d = self.dec_8(self.registers.d),
            0x16 => self.registers.d = self.fetch_byte(),
            0x17 => {
                self.registers.a = self.rl(self.registers.a);
                self.registers.set_flag(CpuFlag::Z, false);
            }
            0x18 => self.jr(),
            0x19 => self.add_16(self.registers.de()),
            0x1A => self.registers.a = self.mmu.mem_read(self.registers.de()),
            0x1B => self.registers.set_de(self.registers.de().wrapping_sub(1)),
            0x1C => self.registers.e = self.inc_8(self.registers.e),
            0x1D => self.registers.e = self.dec_8(self.registers.e),
            0x1E => self.registers.e = self.fetch_byte(),
            0x1F => {
                self.registers.a = self.rr(self.registers.a);
                self.registers.set_flag(CpuFlag::Z, false);
            }
            0x20 => {
                if !self.registers.f.contains(CpuFlag::Z) {
                    self.jr();
                    return (mnemonic, branch_tcycles);
                } else {
                    self.registers.pc += 1;
                }
            }
            0x21 => {
                let data = self.fetch_word();
                self.registers.set_hl(data);
            }
            0x22 => self.mmu.mem_write(self.registers.increment_hl(), self.registers.a),
            0x23 => self.registers.set_hl(self.registers.hl().wrapping_add(1)),
            0x24 => self.registers.h = self.inc_8(self.registers.h),
            0x25 => self.registers.h = self.dec_8(self.registers.h),
            0x26 => self.registers.h = self.fetch_byte(),
            0x27 => self.daa(),
            0x28 => {
                if self.registers.f.contains(CpuFlag::Z) {
                    self.jr();
                    return (mnemonic, branch_tcycles);
                } else {
                    self.registers.pc += 1;
                }
            }
            0x29 => self.add_16(self.registers.hl()),
            0x2A => self.registers.a = self.mmu.mem_read(self.registers.increment_hl()),
            0x2B => self.registers.set_hl(self.registers.hl().wrapping_sub(1)),
            0x2C => self.registers.l = self.inc_8(self.registers.l),
            0x2D => self.registers.l = self.dec_8(self.registers.l),
            0x2E => self.registers.l = self.fetch_byte(),
            0x2F => {
                self.registers.a = !self.registers.a;
                self.registers.set_flag(CpuFlag::N, true);
                self.registers.set_flag(CpuFlag::H, true);
            }
            0x30 => {
                if !self.registers.f.contains(CpuFlag::C) {
                    self.jr();
                    return (mnemonic, branch_tcycles);
                } else {
                    self.registers.pc += 1;
                }
            }
            0x31 => self.registers.sp = self.fetch_word(),
            0x32 => self.mmu.mem_write(self.registers.decrement_hl(), self.registers.a),
            0x33 => self.registers.sp = self.registers.sp.wrapping_add(1),
            0x34 => {
                let data = self.inc_8(self.mmu.mem_read(self.registers.hl()));
                self.mmu.mem_write(self.registers.hl(), data);
            }
            0x35 => {
                let data = self.dec_8(self.mmu.mem_read(self.registers.hl()));
                self.mmu.mem_write(self.registers.hl(), data);
            }
            0x36 => {
                let data = self.fetch_byte();
                self.mmu.mem_write(self.registers.hl(), data);
            }
            0x37 => {
                self.registers.set_flag(CpuFlag::C, true);
                self.registers.set_flag(CpuFlag::H, false);
                self.registers.set_flag(CpuFlag::N, false);
            }
            0x38 => {
                if self.registers.f.contains(CpuFlag::C) {
                    self.jr();
                    return (mnemonic, branch_tcycles);
                } else {
                    self.registers.pc += 1;
                }
            }
            0x39 => self.add_16(self.registers.sp),
            0x3A => self.registers.a = self.mmu.mem_read(self.registers.decrement_hl()),
            0x3B => self.registers.sp = self.registers.sp.wrapping_sub(1),
            0x3C => self.registers.a = self.inc_8(self.registers.a),
            0x3D => self.registers.a = self.dec_8(self.registers.a),
            0x3E => self.registers.a = self.fetch_byte(),
            0x3F => {
                let carry = !self.registers.f.contains(CpuFlag::C);
                self.registers.set_flag(CpuFlag::C, carry);
                self.registers.set_flag(CpuFlag::H, false);
                self.registers.set_flag(CpuFlag::N, false);
            }
            0x40 => {}
            0x41 => self.registers.b = self.registers.c,
            0x42 => self.registers.b = self.registers.d,
            0x43 => self.registers.b = self.registers.e,
            0x44 => self.registers.b = self.registers.h,
            0x45 => self.registers.b = self.registers.l,
            0x46 => self.registers.b = self.mmu.mem_read(self.registers.hl()),
            0x47 => self.registers.b = self.registers.a,
            0x48 => self.registers.c = self.registers.b,
            0x49 => {}
            0x4A => self.registers.c = self.registers.d,
            0x4B => self.registers.c = self.registers.e,
            0x4C => self.registers.c = self.registers.h,
            0x4D => self.registers.c = self.registers.l,
            0x4E => self.registers.c = self.mmu.mem_read(self.registers.hl()),
            0x4F => self.registers.c = self.registers.a,
            0x50 => self.registers.d = self.registers.b,
            0x51 => self.registers.d = self.registers.c,
            0x52 => {}
            0x53 => self.registers.d = self.registers.e,
            0x54 => self.registers.d = self.registers.h,
            0x55 => self.registers.d = self.registers.l,
            0x56 => self.registers.d = self.mmu.mem_read(self.registers.hl()),
            0x57 => self.registers.d = self.registers.a,
            0x58 => self.registers.e = self.registers.b,
            0x59 => self.registers.e = self.registers.c,
            0x5A => self.registers.e = self.registers.d,
            0x5B => {}
            0x5C => self.registers.e = self.registers.h,
            0x5D => self.registers.e = self.registers.l,
            0x5E => self.registers.e = self.mmu.mem_read(self.registers.hl()),
            0x5F => self.registers.e = self.registers.a,
            0x60 => self.registers.h = self.registers.b,
            0x61 => self.registers.h = self.registers.c,
            0x62 => self.registers.h = self.registers.d,
            0x63 => self.registers.h = self.registers.e,
            0x64 => {}
            0x65 => self.registers.h = self.registers.l,
            0x66 => self.registers.h = self.mmu.mem_read(self.registers.hl()),
            0x67 => self.registers.h = self.registers.a,
            0x68 => self.registers.l = self.registers.b,
            0x69 => self.registers.l = self.registers.c,
            0x6A => self.registers.l = self.registers.d,
            0x6B => self.registers.l = self.registers.e,
            0x6C => self.registers.l = self.registers.h,
            0x6D => {}
            0x6E => self.registers.l = self.mmu.mem_read(self.registers.hl()),
            0x6F => self.registers.l = self.registers.a,
            0x70 => self.mmu.mem_write(self.registers.hl(), self.registers.b),
            0x71 => self.mmu.mem_write(self.registers.hl(), self.registers.c),
            0x72 => self.mmu.mem_write(self.registers.hl(), self.registers.d),
            0x73 => self.mmu.mem_write(self.registers.hl(), self.registers.e),
            0x74 => self.mmu.mem_write(self.registers.hl(), self.registers.h),
            0x75 => self.mmu.mem_write(self.registers.hl(), self.registers.l),
            0x76 => self.halted = true,
            0x77 => self.mmu.mem_write(self.registers.hl(), self.registers.a),
            0x78 => self.registers.a = self.registers.b,
            0x79 => self.registers.a = self.registers.c,
            0x7A => self.registers.a = self.registers.d,
            0x7B => self.registers.a = self.registers.e,
            0x7C => self.registers.a = self.registers.h,
            0x7D => self.registers.a = self.registers.l,
            0x7E => self.registers.a = self.mmu.mem_read(self.registers.hl()),
            0x7F => {}
            0x80 => self.add_8(self.registers.b),
            0x81 => self.add_8(self.registers.c),
            0x82 => self.add_8(self.registers.d),
            0x83 => self.add_8(self.registers.e),
            0x84 => self.add_8(self.registers.h),
            0x85 => self.add_8(self.registers.l),
            0x86 => self.add_8(self.mmu.mem_read(self.registers.hl())),
            0x87 => self.add_8(self.registers.a),
            0x88 => self.adc(self.registers.b),
            0x89 => self.adc(self.registers.c),
            0x8A => self.adc(self.registers.d),
            0x8B => self.adc(self.registers.e),
            0x8C => self.adc(self.registers.h),
            0x8D => self.adc(self.registers.l),
            0x8E => self.adc(self.mmu.mem_read(self.registers.hl())),
            0x8F => self.adc(self.registers.a),
            0x90 => self.sub(self.registers.b),
            0x91 => self.sub(self.registers.c),
            0x92 => self.sub(self.registers.d),
            0x93 => self.sub(self.registers.e),
            0x94 => self.sub(self.registers.h),
            0x95 => self.sub(self.registers.l),
            0x96 => self.sub(self.mmu.mem_read(self.registers.hl())),
            0x97 => self.sub(self.registers.a),
            0x98 => self.sbc(self.registers.b),
            0x99 => self.sbc(self.registers.c),
            0x9A => self.sbc(self.registers.d),
            0x9B => self.sbc(self.registers.e),
            0x9C => self.sbc(self.registers.h),
            0x9D => self.sbc(self.registers.l),
            0x9E => self.sbc(self.mmu.mem_read(self.registers.hl())),
            0x9F => self.sbc(self.registers.a),
            0xA0 => self.and(self.registers.b),
            0xA1 => self.and(self.registers.c),
            0xA2 => self.and(self.registers.d),
            0xA3 => self.and(self.registers.e),
            0xA4 => self.and(self.registers.h),
            0xA5 => self.and(self.registers.l),
            0xA6 => self.and(self.mmu.mem_read(self.registers.hl())),
            0xA7 => self.and(self.registers.a),
            0xA8 => self.xor(self.registers.b),
            0xA9 => self.xor(self.registers.c),
            0xAA => self.xor(self.registers.d),
            0xAB => self.xor(self.registers.e),
            0xAC => self.xor(self.registers.h),
            0xAD => self.xor(self.registers.l),
            0xAE => self.xor(self.mmu.mem_read(self.registers.hl())),
            0xAF => self.xor(self.registers.a),
            0xB0 => self.or(self.registers.b),
            0xB1 => self.or(self.registers.c),
            0xB2 => self.or(self.registers.d),
            0xB3 => self.or(self.registers.e),
            0xB4 => self.or(self.registers.h),
            0xB5 => self.or(self.registers.l),
            0xB6 => self.or(self.mmu.mem_read(self.registers.hl())),
            0xB7 => self.or(self.registers.a),
            0xB8 => self.cp(self.registers.b),
            0xB9 => self.cp(self.registers.c),
            0xBA => self.cp(self.registers.d),
            0xBB => self.cp(self.registers.e),
            0xBC => self.cp(self.registers.h),
            0xBD => self.cp(self.registers.l),
            0xBE => self.cp(self.mmu.mem_read(self.registers.hl())),
            0xBF => self.cp(self.registers.a),
            0xC0 => {
                if !self.registers.f.contains(CpuFlag::Z) {
                    self.registers.pc = self.pop_stack();
                    return (mnemonic, branch_tcycles);
                }
            }
            0xC1 => {
                let data = self.pop_stack();
                self.registers.set_bc(data);
            }
            0xC2 => {
                if !self.registers.f.contains(CpuFlag::Z) {
                    self.registers.pc = self.fetch_word();
                    return (mnemonic, branch_tcycles);
                } else {
                    self.registers.pc += 2;
                }
            }
            0xC3 => {
                self.registers.pc = self.fetch_word();
            }
            0xC4 => {
                if !self.registers.f.contains(CpuFlag::Z) {
                    self.push_stack(self.registers.pc + 2);
                    self.registers.pc = self.fetch_word();
                    return (mnemonic, branch_tcycles);
                } else {
                    self.registers.pc += 2;
                }
            }
            0xC5 => self.push_stack(self.registers.bc()),
            0xC6 => {
                let data = self.fetch_byte();
                self.add_8(data);
            }
            0xC7 => self.rst(0x00),
            0xC8 => {
                if self.registers.f.contains(CpuFlag::Z) {
                    self.registers.pc = self.pop_stack();
                    return (mnemonic, branch_tcycles);
                }
            }
            0xC9 => self.registers.pc = self.pop_stack(),
            0xCA => {
                if self.registers.f.contains(CpuFlag::Z) {
                    self.registers.pc = self.fetch_word();
                    return (mnemonic, branch_tcycles);
                } else {
                    self.registers.pc += 2;
                }
            }
            0xCB => return self.execute_cb(), // Still wondering if i need to add here
            0xCC => {
                if self.registers.f.contains(CpuFlag::Z) {
                    self.push_stack(self.registers.pc + 2);
                    self.registers.pc = self.fetch_word();
                    return (mnemonic, branch_tcycles);
                } else {
                    self.registers.pc += 2;
                }
            }
            0xCD => {
                self.push_stack(self.registers.pc + 2);
                self.registers.pc = self.fetch_word();
            }
            0xCE => {
                let data = self.fetch_byte();
                self.adc(data);
            }
            0xCF => self.rst(0x08),
            0xD0 => {
                if !self.registers.f.contains(CpuFlag::C) {
                    self.registers.pc = self.pop_stack();
                    return (mnemonic, branch_tcycles);
                }
            }
            0xD1 => {
                let data = self.pop_stack();
                self.registers.set_de(data);
            }
            0xD2 => {
                if !self.registers.f.contains(CpuFlag::C) {
                    self.registers.pc = self.fetch_word();
                    return (mnemonic, branch_tcycles);
                } else {
                    self.registers.pc += 2;
                }
            }
            0xD4 => {
                if !self.registers.f.contains(CpuFlag::C) {
                    self.push_stack(self.registers.pc + 2);
                    self.registers.pc = self.fetch_word();
                    return (mnemonic, branch_tcycles);
                } else {
                    self.registers.pc += 2;
                }
            }
            0xD5 => self.push_stack(self.registers.de()),
            0xD6 => {
                let data = self.fetch_byte();
                self.sub(data);
            }
            0xD7 => self.rst(0x10),
            0xD8 => {
                if self.registers.f.contains(CpuFlag::C) {
                    self.registers.pc = self.pop_stack();
                    return (mnemonic, branch_tcycles);
                }
            }
            0xD9 => {
                self.registers.pc = self.pop_stack();
                self.ei = ImeState::Enable;
            }
            0xDA => {
                if self.registers.f.contains(CpuFlag::C) {
                    self.registers.pc = self.fetch_word();
                    return (mnemonic, branch_tcycles);
                } else {
                    self.registers.pc += 2;
                }
            }
            0xDC => {
                if self.registers.f.contains(CpuFlag::C) {
                    self.push_stack(self.registers.pc + 2);
                    self.registers.pc = self.fetch_word();
                    return (mnemonic, branch_tcycles);
                } else {
                    self.registers.pc += 2;
                }
            }
            0xDE => {
                let data = self.fetch_byte();
                self.sub(data);
            }
            0xDF => self.rst(0x18),
            0xE0 => {
                let a = 0xFF00 | self.fetch_byte() as u16;
                self.mmu.mem_write(a, self.registers.a);
            }
            0xE1 => {
                let data = self.pop_stack();
                self.registers.set_hl(data);
            }
            0xE2 => self.mmu.mem_write(0xFF00 | self.registers.c as u16, self.registers.a),
            0xE5 => self.push_stack(self.registers.hl()),
            0xE6 => {
                let data = self.fetch_byte();
                self.and(data);
            }
            0xE7 => self.rst(0x20),
            0xE8 => self.registers.sp = self.add_16_signed(self.registers.sp),
            0xE9 => self.registers.pc = self.registers.hl(),
            0xEA => {
                let address = self.fetch_word();
                self.mmu.mem_write(address, self.registers.a);
            }
            0xEE => {
                {
                    let data = self.fetch_byte();
                    self.xor(data);
                };
            }
            0xEF => self.rst(0x28),
            0xF0 => {
                let address = 0xFF00 | self.fetch_byte() as u16;
                self.registers.a = self.mmu.mem_read(address);
            }
            0xF1 => {
                let data = self.pop_stack() & 0xFFF0;
                self.registers.set_af(data)
            }
            0xF2 => self.registers.a = self.mmu.mem_read(0xFF00 | self.registers.c as u16),
            0xF3 => {
                self.di = ImeState::Staged;
            }
            0xF5 => self.push_stack(self.registers.af()),
            0xF6 => {
                let data = self.fetch_byte();
                self.or(data);
            }
            0xF7 => self.rst(0x30),
            0xF8 => {
                let data = self.add_16_signed(self.registers.sp);
                self.registers.set_hl(data)
            }
            0xF9 => self.registers.sp = self.registers.hl(),
            0xFA => {
                let data = self.fetch_word();
                self.registers.a = self.mmu.mem_read(data);
            }
            0xFB => self.ei = ImeState::Staged,
            0xFE => {
                let data = self.fetch_byte();
                self.cp(data);
            }
            0xFF => self.rst(0x38),
            code => panic!("Code {:#04X} not implemented", code),
        }
        (mnemonic, no_branch_tcycles)
    }

    fn execute_cb(&mut self) -> (&str, u8) {
        let opcode = self.fetch_byte();
        let (mnemonic, tcycles) = CB_PREFIXED_OPCODES[opcode as usize];
        match opcode {
            0x00 => self.registers.b = self.rlc(self.registers.b),
            0x01 => self.registers.c = self.rlc(self.registers.c),
            0x02 => self.registers.d = self.rlc(self.registers.d),
            0x03 => self.registers.e = self.rlc(self.registers.e),
            0x04 => self.registers.h = self.rlc(self.registers.h),
            0x05 => self.registers.l = self.rlc(self.registers.l),
            0x06 => {
                let data = self.rlc(self.mmu.mem_read(self.registers.hl()));
                self.mmu.mem_write(self.registers.hl(), data);
            }
            0x07 => self.registers.a = self.rlc(self.registers.a),
            0x08 => self.registers.b = self.rrc(self.registers.b),
            0x09 => self.registers.c = self.rrc(self.registers.c),
            0x0A => self.registers.d = self.rrc(self.registers.d),
            0x0B => self.registers.e = self.rrc(self.registers.e),
            0x0C => self.registers.h = self.rrc(self.registers.h),
            0x0D => self.registers.l = self.rrc(self.registers.l),
            0x0E => {
                let data = self.rrc(self.mmu.mem_read(self.registers.hl()));
                self.mmu.mem_write(self.registers.hl(), data);
            }
            0x0F => self.registers.a = self.rrc(self.registers.a),
            0x10 => self.registers.b = self.rl(self.registers.b),
            0x11 => self.registers.c = self.rl(self.registers.c),
            0x12 => self.registers.d = self.rl(self.registers.d),
            0x13 => self.registers.e = self.rl(self.registers.e),
            0x14 => self.registers.h = self.rl(self.registers.h),
            0x15 => self.registers.l = self.rl(self.registers.l),
            0x16 => {
                let data = self.rl(self.mmu.mem_read(self.registers.hl()));
                self.mmu.mem_write(self.registers.hl(), data);
            }
            0x17 => self.registers.a = self.rl(self.registers.a),
            0x18 => self.registers.b = self.rr(self.registers.b),
            0x19 => self.registers.c = self.rr(self.registers.c),
            0x1A => self.registers.d = self.rr(self.registers.d),
            0x1B => self.registers.e = self.rr(self.registers.e),
            0x1C => self.registers.h = self.rr(self.registers.h),
            0x1D => self.registers.l = self.rr(self.registers.l),
            0x1E => {
                let data = self.rr(self.mmu.mem_read(self.registers.hl()));
                self.mmu.mem_write(self.registers.hl(), data);
            }
            0x1F => self.registers.a = self.rr(self.registers.a),
            0x20 => self.registers.b = self.sla(self.registers.b),
            0x21 => self.registers.c = self.sla(self.registers.c),
            0x22 => self.registers.d = self.sla(self.registers.d),
            0x23 => self.registers.e = self.sla(self.registers.e),
            0x24 => self.registers.h = self.sla(self.registers.h),
            0x25 => self.registers.l = self.sla(self.registers.l),
            0x26 => {
                let data = self.sla(self.mmu.mem_read(self.registers.hl()));
                self.mmu.mem_write(self.registers.hl(), data);
            }
            0x27 => self.registers.a = self.sla(self.registers.a),
            0x28 => self.registers.b = self.sra(self.registers.b),
            0x29 => self.registers.c = self.sra(self.registers.c),
            0x2A => self.registers.d = self.sra(self.registers.d),
            0x2B => self.registers.e = self.sra(self.registers.e),
            0x2C => self.registers.h = self.sra(self.registers.h),
            0x2D => self.registers.l = self.sra(self.registers.l),
            0x2E => {
                let data = self.sra(self.mmu.mem_read(self.registers.hl()));
                self.mmu.mem_write(self.registers.hl(), data);
            }
            0x2F => self.registers.a = self.sra(self.registers.a),
            0x30 => self.registers.b = self.swap(self.registers.b),
            0x31 => self.registers.c = self.swap(self.registers.c),
            0x32 => self.registers.d = self.swap(self.registers.d),
            0x33 => self.registers.e = self.swap(self.registers.e),
            0x34 => self.registers.h = self.swap(self.registers.h),
            0x35 => self.registers.l = self.swap(self.registers.l),
            0x36 => {
                let data = self.swap(self.mmu.mem_read(self.registers.hl()));
                self.mmu.mem_write(self.registers.hl(), data);
            }
            0x37 => self.registers.a = self.swap(self.registers.a),
            0x38 => self.registers.b = self.srl(self.registers.b),
            0x39 => self.registers.c = self.srl(self.registers.c),
            0x3A => self.registers.d = self.srl(self.registers.d),
            0x3B => self.registers.e = self.srl(self.registers.e),
            0x3C => self.registers.h = self.srl(self.registers.h),
            0x3D => self.registers.l = self.srl(self.registers.l),
            0x3E => {
                let data = self.srl(self.mmu.mem_read(self.registers.hl()));
                self.mmu.mem_write(self.registers.hl(), data);
            }
            0x3F => self.registers.a = self.srl(self.registers.a),
            0x40 => self.bit(self.registers.b, 0),
            0x41 => self.bit(self.registers.c, 0),
            0x42 => self.bit(self.registers.d, 0),
            0x43 => self.bit(self.registers.e, 0),
            0x44 => self.bit(self.registers.h, 0),
            0x45 => self.bit(self.registers.l, 0),
            0x46 => {
                let data = self.mmu.mem_read(self.registers.hl());
                self.bit(data, 0)
            }
            0x47 => self.bit(self.registers.a, 0),
            0x48 => self.bit(self.registers.b, 1),
            0x49 => self.bit(self.registers.c, 1),
            0x4A => self.bit(self.registers.d, 1),
            0x4B => self.bit(self.registers.e, 1),
            0x4C => self.bit(self.registers.h, 1),
            0x4D => self.bit(self.registers.l, 1),
            0x4E => {
                let data = self.mmu.mem_read(self.registers.hl());
                self.bit(data, 1)
            }
            0x4F => self.bit(self.registers.a, 1),
            0x50 => self.bit(self.registers.b, 2),
            0x51 => self.bit(self.registers.c, 2),
            0x52 => self.bit(self.registers.d, 2),
            0x53 => self.bit(self.registers.e, 2),
            0x54 => self.bit(self.registers.h, 2),
            0x55 => self.bit(self.registers.l, 2),
            0x56 => {
                let data = self.mmu.mem_read(self.registers.hl());
                self.bit(data, 2)
            }
            0x57 => self.bit(self.registers.a, 2),
            0x58 => self.bit(self.registers.b, 3),
            0x59 => self.bit(self.registers.c, 3),
            0x5A => self.bit(self.registers.d, 3),
            0x5B => self.bit(self.registers.e, 3),
            0x5C => self.bit(self.registers.h, 3),
            0x5D => self.bit(self.registers.l, 3),
            0x5E => {
                let data = self.mmu.mem_read(self.registers.hl());
                self.bit(data, 3)
            }
            0x5F => self.bit(self.registers.a, 3),
            0x60 => self.bit(self.registers.b, 4),
            0x61 => self.bit(self.registers.c, 4),
            0x62 => self.bit(self.registers.d, 4),
            0x63 => self.bit(self.registers.e, 4),
            0x64 => self.bit(self.registers.h, 4),
            0x65 => self.bit(self.registers.l, 4),
            0x66 => {
                let data = self.mmu.mem_read(self.registers.hl());
                self.bit(data, 4)
            }
            0x67 => self.bit(self.registers.a, 4),
            0x68 => self.bit(self.registers.b, 5),
            0x69 => self.bit(self.registers.c, 5),
            0x6A => self.bit(self.registers.d, 5),
            0x6B => self.bit(self.registers.e, 5),
            0x6C => self.bit(self.registers.h, 5),
            0x6D => self.bit(self.registers.l, 5),
            0x6E => {
                let data = self.mmu.mem_read(self.registers.hl());
                self.bit(data, 5)
            }
            0x6F => self.bit(self.registers.a, 5),
            0x70 => self.bit(self.registers.b, 6),
            0x71 => self.bit(self.registers.c, 6),
            0x72 => self.bit(self.registers.d, 6),
            0x73 => self.bit(self.registers.e, 6),
            0x74 => self.bit(self.registers.h, 6),
            0x75 => self.bit(self.registers.l, 6),
            0x76 => {
                let data = self.mmu.mem_read(self.registers.hl());
                self.bit(data, 6)
            }
            0x77 => self.bit(self.registers.a, 6),
            0x78 => self.bit(self.registers.b, 7),
            0x79 => self.bit(self.registers.c, 7),
            0x7A => self.bit(self.registers.d, 7),
            0x7B => self.bit(self.registers.e, 7),
            0x7C => self.bit(self.registers.h, 7),
            0x7D => self.bit(self.registers.l, 7),
            0x7E => {
                let data = self.mmu.mem_read(self.registers.hl());
                self.bit(data, 7)
            }
            0x7F => self.bit(self.registers.a, 7),
            0x80 => self.registers.b = self.registers.b & !(1 << 0),
            0x81 => self.registers.c = self.registers.c & !(1 << 0),
            0x82 => self.registers.d = self.registers.d & !(1 << 0),
            0x83 => self.registers.e = self.registers.e & !(1 << 0),
            0x84 => self.registers.h = self.registers.h & !(1 << 0),
            0x85 => self.registers.l = self.registers.l & !(1 << 0),
            0x86 => {
                let data = self.mmu.mem_read(self.registers.hl()) & !(1 << 0);
                self.mmu.mem_write(self.registers.hl(), data)
            }
            0x87 => self.registers.a = self.registers.a & !(1 << 0),
            0x88 => self.registers.b = self.registers.b & !(1 << 1),
            0x89 => self.registers.c = self.registers.c & !(1 << 1),
            0x8A => self.registers.d = self.registers.d & !(1 << 1),
            0x8B => self.registers.e = self.registers.e & !(1 << 1),
            0x8C => self.registers.h = self.registers.h & !(1 << 1),
            0x8D => self.registers.l = self.registers.l & !(1 << 1),
            0x8E => {
                let data = self.mmu.mem_read(self.registers.hl()) & !(1 << 1);
                self.mmu.mem_write(self.registers.hl(), data)
            }
            0x8F => self.registers.a = self.registers.a & !(1 << 1),
            0x90 => self.registers.b = self.registers.b & !(1 << 2),
            0x91 => self.registers.c = self.registers.c & !(1 << 2),
            0x92 => self.registers.d = self.registers.d & !(1 << 2),
            0x93 => self.registers.e = self.registers.e & !(1 << 2),
            0x94 => self.registers.h = self.registers.h & !(1 << 2),
            0x95 => self.registers.l = self.registers.l & !(1 << 2),
            0x96 => {
                let data = self.mmu.mem_read(self.registers.hl()) & !(1 << 2);
                self.mmu.mem_write(self.registers.hl(), data)
            }
            0x97 => self.registers.a = self.registers.a & !(1 << 2),
            0x98 => self.registers.b = self.registers.b & !(1 << 3),
            0x99 => self.registers.c = self.registers.c & !(1 << 3),
            0x9A => self.registers.d = self.registers.d & !(1 << 3),
            0x9B => self.registers.e = self.registers.e & !(1 << 3),
            0x9C => self.registers.h = self.registers.h & !(1 << 3),
            0x9D => self.registers.l = self.registers.l & !(1 << 3),
            0x9E => {
                let data = self.mmu.mem_read(self.registers.hl()) & !(1 << 3);
                self.mmu.mem_write(self.registers.hl(), data)
            }
            0x9F => self.registers.a = self.registers.a & !(1 << 3),
            0xA0 => self.registers.b = self.registers.b & !(1 << 4),
            0xA1 => self.registers.c = self.registers.c & !(1 << 4),
            0xA2 => self.registers.d = self.registers.d & !(1 << 4),
            0xA3 => self.registers.e = self.registers.e & !(1 << 4),
            0xA4 => self.registers.h = self.registers.h & !(1 << 4),
            0xA5 => self.registers.l = self.registers.l & !(1 << 4),
            0xA6 => {
                let data = self.mmu.mem_read(self.registers.hl()) & !(1 << 4);
                self.mmu.mem_write(self.registers.hl(), data)
            }
            0xA7 => self.registers.a = self.registers.a & !(1 << 4),
            0xA8 => self.registers.b = self.registers.b & !(1 << 5),
            0xA9 => self.registers.c = self.registers.c & !(1 << 5),
            0xAA => self.registers.d = self.registers.d & !(1 << 5),
            0xAB => self.registers.e = self.registers.e & !(1 << 5),
            0xAC => self.registers.h = self.registers.h & !(1 << 5),
            0xAD => self.registers.l = self.registers.l & !(1 << 5),
            0xAE => {
                let data = self.mmu.mem_read(self.registers.hl()) & !(1 << 5);
                self.mmu.mem_write(self.registers.hl(), data)
            }
            0xAF => self.registers.a = self.registers.a & !(1 << 5),
            0xB0 => self.registers.b = self.registers.b & !(1 << 6),
            0xB1 => self.registers.c = self.registers.c & !(1 << 6),
            0xB2 => self.registers.d = self.registers.d & !(1 << 6),
            0xB3 => self.registers.e = self.registers.e & !(1 << 6),
            0xB4 => self.registers.h = self.registers.h & !(1 << 6),
            0xB5 => self.registers.l = self.registers.l & !(1 << 6),
            0xB6 => {
                let data = self.mmu.mem_read(self.registers.hl()) & !(1 << 6);
                self.mmu.mem_write(self.registers.hl(), data)
            }
            0xB7 => self.registers.a = self.registers.a & !(1 << 6),
            0xB8 => self.registers.b = self.registers.b & !(1 << 7),
            0xB9 => self.registers.c = self.registers.c & !(1 << 7),
            0xBA => self.registers.d = self.registers.d & !(1 << 7),
            0xBB => self.registers.e = self.registers.e & !(1 << 7),
            0xBC => self.registers.h = self.registers.h & !(1 << 7),
            0xBD => self.registers.l = self.registers.l & !(1 << 7),
            0xBE => {
                let data = self.mmu.mem_read(self.registers.hl()) & !(1 << 7);
                self.mmu.mem_write(self.registers.hl(), data)
            }
            0xBF => self.registers.a = self.registers.a & !(1 << 7),
            0xC0 => self.registers.b = self.registers.b | (1 << 0),
            0xC1 => self.registers.c = self.registers.c | (1 << 0),
            0xC2 => self.registers.d = self.registers.d | (1 << 0),
            0xC3 => self.registers.e = self.registers.e | (1 << 0),
            0xC4 => self.registers.h = self.registers.h | (1 << 0),
            0xC5 => self.registers.l = self.registers.l | (1 << 0),
            0xC6 => {
                let data = self.mmu.mem_read(self.registers.hl()) | (1 << 0);
                self.mmu.mem_write(self.registers.hl(), data)
            }
            0xC7 => self.registers.a = self.registers.a | (1 << 0),
            0xC8 => self.registers.b = self.registers.b | (1 << 1),
            0xC9 => self.registers.c = self.registers.c | (1 << 1),
            0xCA => self.registers.d = self.registers.d | (1 << 1),
            0xCB => self.registers.e = self.registers.e | (1 << 1),
            0xCC => self.registers.h = self.registers.h | (1 << 1),
            0xCD => self.registers.l = self.registers.l | (1 << 1),
            0xCE => {
                let data = self.mmu.mem_read(self.registers.hl()) | (1 << 1);
                self.mmu.mem_write(self.registers.hl(), data)
            }
            0xCF => self.registers.a = self.registers.a | (1 << 1),
            0xD0 => self.registers.b = self.registers.b | (1 << 2),
            0xD1 => self.registers.c = self.registers.c | (1 << 2),
            0xD2 => self.registers.d = self.registers.d | (1 << 2),
            0xD3 => self.registers.e = self.registers.e | (1 << 2),
            0xD4 => self.registers.h = self.registers.h | (1 << 2),
            0xD5 => self.registers.l = self.registers.l | (1 << 2),
            0xD6 => {
                let data = self.mmu.mem_read(self.registers.hl()) | (1 << 2);
                self.mmu.mem_write(self.registers.hl(), data)
            }
            0xD7 => self.registers.a = self.registers.a | (1 << 2),
            0xD8 => self.registers.b = self.registers.b | (1 << 3),
            0xD9 => self.registers.c = self.registers.c | (1 << 3),
            0xDA => self.registers.d = self.registers.d | (1 << 3),
            0xDB => self.registers.e = self.registers.e | (1 << 3),
            0xDC => self.registers.h = self.registers.h | (1 << 3),
            0xDD => self.registers.l = self.registers.l | (1 << 3),
            0xDE => {
                let data = self.mmu.mem_read(self.registers.hl()) | (1 << 3);
                self.mmu.mem_write(self.registers.hl(), data)
            }
            0xDF => self.registers.a = self.registers.a | (1 << 3),
            0xE0 => self.registers.b = self.registers.b | (1 << 4),
            0xE1 => self.registers.c = self.registers.c | (1 << 4),
            0xE2 => self.registers.d = self.registers.d | (1 << 4),
            0xE3 => self.registers.e = self.registers.e | (1 << 4),
            0xE4 => self.registers.h = self.registers.h | (1 << 4),
            0xE5 => self.registers.l = self.registers.l | (1 << 4),
            0xE6 => {
                let data = self.mmu.mem_read(self.registers.hl()) | (1 << 4);
                self.mmu.mem_write(self.registers.hl(), data)
            }
            0xE7 => self.registers.a = self.registers.a | (1 << 4),
            0xE8 => self.registers.b = self.registers.b | (1 << 5),
            0xE9 => self.registers.c = self.registers.c | (1 << 5),
            0xEA => self.registers.d = self.registers.d | (1 << 5),
            0xEB => self.registers.e = self.registers.e | (1 << 5),
            0xEC => self.registers.h = self.registers.h | (1 << 5),
            0xED => self.registers.l = self.registers.l | (1 << 5),
            0xEE => {
                let data = self.mmu.mem_read(self.registers.hl()) | (1 << 5);
                self.mmu.mem_write(self.registers.hl(), data)
            }
            0xEF => self.registers.a = self.registers.a | (1 << 5),
            0xF0 => self.registers.b = self.registers.b | (1 << 6),
            0xF1 => self.registers.c = self.registers.c | (1 << 6),
            0xF2 => self.registers.d = self.registers.d | (1 << 6),
            0xF3 => self.registers.e = self.registers.e | (1 << 6),
            0xF4 => self.registers.h = self.registers.h | (1 << 6),
            0xF5 => self.registers.l = self.registers.l | (1 << 6),
            0xF6 => {
                let data = self.mmu.mem_read(self.registers.hl()) | (1 << 6);
                self.mmu.mem_write(self.registers.hl(), data)
            }
            0xF7 => self.registers.a = self.registers.a | (1 << 6),
            0xF8 => self.registers.b = self.registers.b | (1 << 7),
            0xF9 => self.registers.c = self.registers.c | (1 << 7),
            0xFA => self.registers.d = self.registers.d | (1 << 7),
            0xFB => self.registers.e = self.registers.e | (1 << 7),
            0xFC => self.registers.h = self.registers.h | (1 << 7),
            0xFD => self.registers.l = self.registers.l | (1 << 7),
            0xFE => {
                let data = self.mmu.mem_read(self.registers.hl()) | (1 << 7);
                self.mmu.mem_write(self.registers.hl(), data)
            }
            0xFF => self.registers.a = self.registers.a | (1 << 7),
        }
        (mnemonic, tcycles)
    }

    // ALU
    fn inc_8(&mut self, data: u8) -> u8 {
        let result = data.wrapping_add(1);
        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, (data & 0x0F) + 1 > 0x0F);
        result
    }

    fn dec_8(&mut self, data: u8) -> u8 {
        let result = data.wrapping_sub(1);
        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, (data & 0x0F) + 1 > 0x0F);
        result
    }

    fn add_16(&mut self, data: u16) {
        let result = self.registers.hl().wrapping_add(data);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers
            .set_flag(CpuFlag::H, (self.registers.hl() & 0x07FF) + (data & 0x07FF) > 0x07FF);
        self.registers.set_flag(CpuFlag::C, self.registers.hl() as u32 + data as u32 > 0xFFFF);
        self.registers.set_hl(result);
    }

    fn add_16_signed(&mut self, data: u16) -> u16 {
        let signed = self.fetch_byte() as i8 as i16 as u16;
        self.registers.set_flag(CpuFlag::Z, false);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers
            .set_flag(CpuFlag::H, (self.registers.sp & 0x000F) + (data & 0x000F) > 0x000F);
        self.registers
            .set_flag(CpuFlag::C, (self.registers.sp & 0x00FF) + (data & 0x00FF) > 0x00FF);
        data.wrapping_add(signed)
    }

    fn add_8(&mut self, data: u8) {
        let result = self.registers.a.wrapping_add(data);
        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, (self.registers.a & 0x0F) + (data & 0x0F) > 0x0F);
        self.registers.set_flag(CpuFlag::C, self.registers.a as u16 + data as u16 > 0xFF);
        self.registers.a = result;
    }

    fn adc(&mut self, data: u8) {
        let carry = if self.registers.f.contains(CpuFlag::C) { 1 } else { 0 };
        let result = self.registers.a.wrapping_add(data).wrapping_add(carry);
        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers
            .set_flag(CpuFlag::H, (self.registers.a & 0x0F) + (data & 0x0F) + carry > 0x0F);
        self.registers
            .set_flag(CpuFlag::C, self.registers.a as u16 + data as u16 + (carry as u16) > 0xFF);
        self.registers.a = result;
    }

    fn sub(&mut self, data: u8) {
        let result = self.registers.a.wrapping_sub(data);
        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, true);
        self.registers.set_flag(CpuFlag::H, (self.registers.a & 0x0F) < (data & 0x0F));
        self.registers.set_flag(CpuFlag::C, (self.registers.a as u16) < (data as u16));
        self.registers.a = result;
    }

    fn sbc(&mut self, data: u8) {
        let carry = if self.registers.f.contains(CpuFlag::C) { 1 } else { 0 };
        let result = self.registers.a.wrapping_sub(data).wrapping_sub(carry);
        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, true);
        self.registers.set_flag(CpuFlag::H, (self.registers.a & 0x0F) < (data & 0x0F) + carry);
        self.registers
            .set_flag(CpuFlag::C, (self.registers.a as u16) < (data as u16) + (carry as u16));
        self.registers.a = result;
    }

    fn and(&mut self, data: u8) {
        let result = self.registers.a & data;
        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, true);
        self.registers.set_flag(CpuFlag::C, false);
        self.registers.a = result;
    }

    fn or(&mut self, data: u8) {
        let result = self.registers.a | data;
        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, false);
        self.registers.a = result;
    }

    fn xor(&mut self, data: u8) {
        let result = self.registers.a ^ data;
        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, false);
        self.registers.a = result;
    }

    fn cp(&mut self, data: u8) {
        let result = self.registers.a ^ data;
        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, true);
        self.registers.set_flag(CpuFlag::H, (self.registers.a & 0x0F) < (data & 0x0F));
        self.registers.set_flag(CpuFlag::C, (self.registers.a as u16) < (data as u16));
        self.registers.a = result;
    }

    fn daa(&mut self) {
        let mut a = self.registers.a;
        let mut correction = if self.registers.f.contains(CpuFlag::C) { 0x60 } else { 0x00 };

        if self.registers.f.contains(CpuFlag::H) {
            correction |= 0x06;
        }

        if !self.registers.f.contains(CpuFlag::N) {
            if a & 0x0F > 0x09 {
                correction |= 0x06;
            };
            if a > 0x99 {
                correction |= 0x60;
            }
        }
        a = a.wrapping_add(correction);
        self.registers.set_flag(CpuFlag::Z, a == 0);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, correction >= 0x60);
        self.registers.a = a;
    }

    // RSB
    fn rotate_shift_flag_update(&mut self, result: u8, carry: bool) {
        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, carry);
    }

    fn rlc(&mut self, data: u8) -> u8 {
        let carry = data & 0x80 == 0x80;
        let result = (data << 1) | (if carry { 1 } else { 0 });
        self.rotate_shift_flag_update(result, carry);
        result
    }

    fn rl(&mut self, data: u8) -> u8 {
        let carry = data & 0x80 == 0x80;
        let result = (data << 1) | (if self.registers.f.contains(CpuFlag::C) { 1 } else { 0 });
        self.rotate_shift_flag_update(result, carry);
        result
    }

    fn rrc(&mut self, data: u8) -> u8 {
        let carry = data & 0x01 == 0x01;
        let result = (data >> 1) | (if carry { 0x80 } else { 0 });
        self.rotate_shift_flag_update(result, carry);
        result
    }

    fn rr(&mut self, data: u8) -> u8 {
        let carry = data & 0x01 == 0x01;
        let result = (data >> 1) | (if self.registers.f.contains(CpuFlag::C) { 0x80 } else { 0 });
        self.rotate_shift_flag_update(result, carry);
        result
    }

    fn sla(&mut self, data: u8) -> u8 {
        let carry = data & 0x80 == 0x80;
        let result = data << 1;
        self.rotate_shift_flag_update(result, carry);
        result
    }

    fn sra(&mut self, data: u8) -> u8 {
        let carry = data & 0x01 == 0x01;
        let result = (data >> 1) | (data & 0x80);
        self.rotate_shift_flag_update(result, carry);
        result
    }

    fn swap(&mut self, data: u8) -> u8 {
        self.registers.set_flag(CpuFlag::Z, data == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, false);
        (data >> 4) | (data << 4)
    }

    fn srl(&mut self, data: u8) -> u8 {
        let carry = data & 0x01 == 0x01;
        let result = data >> 1;
        self.rotate_shift_flag_update(result, carry);
        result
    }

    fn bit(&mut self, data: u8, bit: u8) {
        let result = data & (1 << (bit as u32)) == 0;
        self.registers.set_flag(CpuFlag::Z, result);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
    }

    // branch
    fn jr(&mut self) {
        self.registers.pc = ((self.registers.pc as u32 as i32) + (self.fetch_byte() as i8 as i32)) as u16;
    }

    fn rst(&mut self, data: u16) {
        self.push_stack(self.registers.pc);
        self.registers.pc = data
    }
}
