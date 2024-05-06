use crate::{
    mmu::{Memory, Mmu},
    registers::{CpuFlag, Registers},
};

use self::opcode::UNPREFIXED_OPCODES;

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
        self.registers.sp = self.registers.sp.wrapping_sub(2);
        self.mmu.mem_write_16(self.registers.sp, program_counter);
        self.registers.pc = 0x0040 | ((bits as u16) << 3);

        16
    }

    pub fn run_cycle(&mut self) -> u8 {
        self.update_ime_state();
        let interrupt = self.handle_interrupt();
        if interrupt == 0 {
            return interrupt;
        }

        if self.halted {
            1
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
                let value = self.fetch_word();
                self.registers.set_bc(value);
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
            0x23 => {
                let data = self.registers.hl().wrapping_add(1);
                self.registers.set_hl(data);
            }
            0x24 => self.registers.h = self.inc_8(self.registers.h),
            0x25 => self.registers.h = self.dec_8(self.registers.h),
            0x26 => self.registers.h = self.fetch_byte(),
            0x27 => self.daa(),
            0x28 => {
                if self.registers.f.contains(CpuFlag::Z) {
                    self.jr();
                } else {
                    self.registers.pc += 1;
                }
            }
            0x29 => {
                let data = self.registers.hl();
                self.add_16(data);
            }
            0x2A => self.registers.a = self.mmu.mem_read(self.registers.increment_hl()),
            0x2B => {
                let v = self.registers.hl().wrapping_sub(1);
                self.registers.set_hl(v);
            }
            0x2C => self.registers.l = self.inc_8(self.registers.l),
            0x2D => self.registers.l = self.dec_8(self.registers.l),
            0x2E => self.registers.l = self.fetch_byte(),
            0x2F => {
                self.registers.a = !self.registers.a;
                self.registers.set_flag(CpuFlag::N, true);
                self.registers.set_flag(CpuFlag::H, true);
            }
            code => panic!("Code {:#04X} not implemented", code),
        }
        (mnemonic, no_branch_tcycles)
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
        self.registers.set_flag(CpuFlag::N, false);
        self.registers
            .set_flag(CpuFlag::H, (self.registers.hl() & 0x07FF) + (data & 0x07FF) > 0x07FF);
        self.registers.set_flag(CpuFlag::C, self.registers.hl() as u32 + data as u32 > 0xFFFF);

        let result = self.registers.hl().wrapping_add(data);
        self.registers.set_hl(result)
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

    // branch
    fn jr(&mut self) {
        self.registers.pc = ((self.registers.pc as i32) + (self.fetch_byte() as i32)) as u16;
    }
}
