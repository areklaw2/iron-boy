mod alu_instructions;
mod branch_instructions;
mod lsm_instructions;
mod misc_instructions;
mod rsb_instructions;

use crate::{
    bus::{Bus, Memory},
    opcodes::{CB_PREFIXED_OPCODES_MAP, UNPREFIXED_OPCODES_MAP},
    registers::Registers,
};

use self::{
    alu_instructions::AluInstructions, branch_instructions::BranchInstructions,
    lsm_instructions::LsmInstructions, misc_instructions::MiscInstructions,
    rsb_instructions::RsbInstructions,
};

trait Instruction {
    fn get_operands<'a>(&self, mnemonic: &'a str) -> &'a str {
        let operand: &str = mnemonic.split_whitespace().nth(1).unwrap_or_default();
        operand
    }
}

pub enum ImeState {
    Disable,
    Enable,
    Staged,
    NoChange,
}

pub struct Disassembler<'a> {
    registers: Registers,
    bus: Bus,
    halted: bool,
    ime: bool,
    ei: ImeState,
    di: ImeState,
    lsm_instructions: LsmInstructions<'a>,
    alu_instructions: AluInstructions<'a>,
    rsb_instructions: RsbInstructions<'a>,
    branch_instructions: BranchInstructions<'a>,
    misc_instructions: MiscInstructions<'a>,
}

impl<'a> Memory for Disassembler<'a> {
    fn mem_read(&self, address: u16) -> u8 {
        self.bus.mem_read(address)
    }

    fn mem_read_16(&self, address: u16) -> u16 {
        self.bus.mem_read_16(address)
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        self.bus.mem_write(address, data)
    }

    fn mem_write_16(&mut self, address: u16, data: u16) {
        self.bus.mem_write_16(address, data);
    }
}

impl<'a> Disassembler<'a> {
    pub fn new(registers: &mut Registers, bus: &mut Bus) -> Self {
        Disassembler {
            registers: *registers,
            bus: *bus,
            halted: false,
            ime: false,
            ei: ImeState::NoChange,
            di: ImeState::NoChange,
            lsm_instructions: LsmInstructions::new(registers, bus),
            alu_instructions: AluInstructions::new(registers, bus),
            rsb_instructions: RsbInstructions::new(registers, bus),
            branch_instructions: BranchInstructions::new(registers, bus),
            misc_instructions: MiscInstructions::new(registers, bus),
        }
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.bus.mem_read(self.registers.pc);
        self.registers.pc += 1;
        byte
    }

    fn fetch_word(&mut self) -> u16 {
        let word = self.bus.mem_read_16(self.registers.pc);
        self.registers.pc += 2;
        word
    }

    fn push_stack(&mut self, data: u16) {
        self.registers.sp = self.registers.sp.wrapping_sub(2);
        self.mem_write_16(self.registers.sp, data);
    }

    fn pop_stack(&mut self) -> u16 {
        let data = self.mem_read_16(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(2);
        data
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

        let requested = self.mem_read(0xFFFF) & self.mem_read(0xFF0F);
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

        let mut interrupt = self.mem_read(0xFF0F);
        interrupt &= !(1 << bits);
        self.mem_write(0xFF0F, interrupt);

        let program_counter = self.registers.pc;
        self.push_stack(program_counter);
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
            self.execute()
        }
    }

    fn execute(&mut self) -> u8 {
        let value = self.fetch_byte();
        let opcode = UNPREFIXED_OPCODES_MAP.get(&value).unwrap();
        match value {
            0x00 => self.misc_instructions.nop(opcode),
            0x01 => self.lsm_instructions.ld_16(opcode),
            0x02 => self.lsm_instructions.ld_8(opcode),
            0x03 => self.alu_instructions.inc_16(opcode),
            0x04 => self.alu_instructions.inc_8(opcode),
            0x05 => self.alu_instructions.dec_8(opcode),
            0x06 => self.lsm_instructions.ld_8(opcode),
            0x07 => self.rsb_instructions.rlca(opcode),
            0x08 => self.lsm_instructions.ld_16(opcode),
            0x09 => self.alu_instructions.add_16(opcode),
            0x0A => self.lsm_instructions.ld_8(opcode),
            0x0B => self.alu_instructions.dec_16(opcode),
            0x0C => self.alu_instructions.inc_8(opcode),
            0x0D => self.alu_instructions.dec_8(opcode),
            0x0E => self.lsm_instructions.ld_8(opcode),
            0x0F => self.rsb_instructions.rrca(opcode),
            0x10 => self.misc_instructions.stop(opcode),
            0x11 => self.lsm_instructions.ld_16(opcode),
            0x12 => self.lsm_instructions.ld_8(opcode),
            0x13 => self.alu_instructions.inc_16(opcode),
            0x14 => self.alu_instructions.inc_8(opcode),
            0x15 => self.alu_instructions.dec_8(opcode),
            0x16 => self.lsm_instructions.ld_8(opcode),
            0x17 => self.rsb_instructions.rla(opcode),
            0x18 => self.branch_instructions.jr(opcode),
            0x19 => self.alu_instructions.add_16(opcode),
            0x1A => self.lsm_instructions.ld_8(opcode),
            0x1B => self.alu_instructions.dec_16(opcode),
            0x1C => self.alu_instructions.inc_8(opcode),
            0x1D => self.alu_instructions.dec_8(opcode),
            0x1E => self.lsm_instructions.ld_8(opcode),
            0x1F => self.rsb_instructions.rra(opcode),
            0x20 => self.branch_instructions.jr(opcode),
            0x21 => self.lsm_instructions.ld_16(opcode),
            0x22 => self.lsm_instructions.ld_8(opcode),
            0x23 => self.alu_instructions.inc_16(opcode),
            0x24 => self.alu_instructions.inc_8(opcode),
            0x25 => self.alu_instructions.dec_8(opcode),
            0x26 => self.lsm_instructions.ld_8(opcode),
            0x27 => self.alu_instructions.daa(opcode),
            0x28 => self.branch_instructions.jr(opcode),
            0x29 => self.alu_instructions.add_16(opcode),
            0x2A => self.lsm_instructions.ld_8(opcode),
            0x2B => self.alu_instructions.dec_16(opcode),
            0x2C => self.alu_instructions.inc_8(opcode),
            0x2D => self.alu_instructions.dec_8(opcode),
            0x2E => self.lsm_instructions.ld_8(opcode),
            0x2F => self.alu_instructions.cpl(opcode),
            0x30 => self.branch_instructions.jr(opcode),
            0x31 => self.lsm_instructions.ld_16(opcode),
            0x32 => self.lsm_instructions.ld_8(opcode),
            0x33 => self.alu_instructions.inc_16(opcode),
            0x34 => self.alu_instructions.inc_8(opcode),
            0x35 => self.alu_instructions.dec_8(opcode),
            0x36 => self.lsm_instructions.ld_8(opcode),
            0x37 => self.alu_instructions.scf(opcode),
            0x38 => self.branch_instructions.jr(opcode),
            0x39 => self.alu_instructions.add_16(opcode),
            0x3A => self.lsm_instructions.ld_8(opcode),
            0x3B => self.alu_instructions.dec_16(opcode),
            0x3C => self.alu_instructions.inc_8(opcode),
            0x3D => self.alu_instructions.dec_8(opcode),
            0x3E => self.lsm_instructions.ld_8(opcode),
            0x3F => self.alu_instructions.ccf(opcode),
            0x40 => self.lsm_instructions.ld_8(opcode),
            0x41 => self.lsm_instructions.ld_8(opcode),
            0x42 => self.lsm_instructions.ld_8(opcode),
            0x43 => self.lsm_instructions.ld_8(opcode),
            0x44 => self.lsm_instructions.ld_8(opcode),
            0x45 => self.lsm_instructions.ld_8(opcode),
            0x46 => self.lsm_instructions.ld_8(opcode),
            0x47 => self.lsm_instructions.ld_8(opcode),
            0x48 => self.lsm_instructions.ld_8(opcode),
            0x49 => self.lsm_instructions.ld_8(opcode),
            0x4A => self.lsm_instructions.ld_8(opcode),
            0x4B => self.lsm_instructions.ld_8(opcode),
            0x4C => self.lsm_instructions.ld_8(opcode),
            0x4D => self.lsm_instructions.ld_8(opcode),
            0x4E => self.lsm_instructions.ld_8(opcode),
            0x4F => self.lsm_instructions.ld_8(opcode),
            0x50 => self.lsm_instructions.ld_8(opcode),
            0x51 => self.lsm_instructions.ld_8(opcode),
            0x52 => self.lsm_instructions.ld_8(opcode),
            0x53 => self.lsm_instructions.ld_8(opcode),
            0x54 => self.lsm_instructions.ld_8(opcode),
            0x55 => self.lsm_instructions.ld_8(opcode),
            0x56 => self.lsm_instructions.ld_8(opcode),
            0x57 => self.lsm_instructions.ld_8(opcode),
            0x58 => self.lsm_instructions.ld_8(opcode),
            0x59 => self.lsm_instructions.ld_8(opcode),
            0x5A => self.lsm_instructions.ld_8(opcode),
            0x5B => self.lsm_instructions.ld_8(opcode),
            0x5C => self.lsm_instructions.ld_8(opcode),
            0x5D => self.lsm_instructions.ld_8(opcode),
            0x5E => self.lsm_instructions.ld_8(opcode),
            0x5F => self.lsm_instructions.ld_8(opcode),
            0x60 => self.lsm_instructions.ld_8(opcode),
            0x61 => self.lsm_instructions.ld_8(opcode),
            0x62 => self.lsm_instructions.ld_8(opcode),
            0x63 => self.lsm_instructions.ld_8(opcode),
            0x64 => self.lsm_instructions.ld_8(opcode),
            0x65 => self.lsm_instructions.ld_8(opcode),
            0x66 => self.lsm_instructions.ld_8(opcode),
            0x67 => self.lsm_instructions.ld_8(opcode),
            0x68 => self.lsm_instructions.ld_8(opcode),
            0x69 => self.lsm_instructions.ld_8(opcode),
            0x6A => self.lsm_instructions.ld_8(opcode),
            0x6B => self.lsm_instructions.ld_8(opcode),
            0x6C => self.lsm_instructions.ld_8(opcode),
            0x6D => self.lsm_instructions.ld_8(opcode),
            0x6E => self.lsm_instructions.ld_8(opcode),
            0x6F => self.lsm_instructions.ld_8(opcode),
            0x70 => self.lsm_instructions.ld_8(opcode),
            0x71 => self.lsm_instructions.ld_8(opcode),
            0x72 => self.lsm_instructions.ld_8(opcode),
            0x73 => self.lsm_instructions.ld_8(opcode),
            0x74 => self.lsm_instructions.ld_8(opcode),
            0x75 => self.lsm_instructions.ld_8(opcode),
            0x76 => self.misc_instructions.halt(opcode, &mut self.halted),
            0x77 => self.lsm_instructions.ld_8(opcode),
            0x78 => self.lsm_instructions.ld_8(opcode),
            0x79 => self.lsm_instructions.ld_8(opcode),
            0x7A => self.lsm_instructions.ld_8(opcode),
            0x7B => self.lsm_instructions.ld_8(opcode),
            0x7C => self.lsm_instructions.ld_8(opcode),
            0x7D => self.lsm_instructions.ld_8(opcode),
            0x7E => self.lsm_instructions.ld_8(opcode),
            0x7F => self.lsm_instructions.ld_8(opcode),
            0x80 => self.alu_instructions.add_8(opcode),
            0x81 => self.alu_instructions.add_8(opcode),
            0x82 => self.alu_instructions.add_8(opcode),
            0x83 => self.alu_instructions.add_8(opcode),
            0x84 => self.alu_instructions.add_8(opcode),
            0x85 => self.alu_instructions.add_8(opcode),
            0x86 => self.alu_instructions.add_8(opcode),
            0x87 => self.alu_instructions.add_8(opcode),
            0x88 => self.alu_instructions.adc(opcode),
            0x89 => self.alu_instructions.adc(opcode),
            0x8A => self.alu_instructions.adc(opcode),
            0x8B => self.alu_instructions.adc(opcode),
            0x8C => self.alu_instructions.adc(opcode),
            0x8E => self.alu_instructions.adc(opcode),
            0x8D => self.alu_instructions.adc(opcode),
            0x8F => self.alu_instructions.adc(opcode),
            0x90 => self.alu_instructions.sub(opcode),
            0x91 => self.alu_instructions.sub(opcode),
            0x92 => self.alu_instructions.sub(opcode),
            0x93 => self.alu_instructions.sub(opcode),
            0x94 => self.alu_instructions.sub(opcode),
            0x95 => self.alu_instructions.sub(opcode),
            0x96 => self.alu_instructions.sub(opcode),
            0x97 => self.alu_instructions.sub(opcode),
            0x98 => self.alu_instructions.sbc(opcode),
            0x99 => self.alu_instructions.sbc(opcode),
            0x9A => self.alu_instructions.sbc(opcode),
            0x9B => self.alu_instructions.sbc(opcode),
            0x9C => self.alu_instructions.sbc(opcode),
            0x9E => self.alu_instructions.sbc(opcode),
            0x9D => self.alu_instructions.sbc(opcode),
            0x9F => self.alu_instructions.sbc(opcode),
            0xA0 => self.alu_instructions.and(opcode),
            0xA1 => self.alu_instructions.and(opcode),
            0xA2 => self.alu_instructions.and(opcode),
            0xA3 => self.alu_instructions.and(opcode),
            0xA4 => self.alu_instructions.and(opcode),
            0xA5 => self.alu_instructions.and(opcode),
            0xA6 => self.alu_instructions.and(opcode),
            0xA7 => self.alu_instructions.and(opcode),
            0xA8 => self.alu_instructions.xor(opcode),
            0xA9 => self.alu_instructions.xor(opcode),
            0xAA => self.alu_instructions.xor(opcode),
            0xAB => self.alu_instructions.xor(opcode),
            0xAC => self.alu_instructions.xor(opcode),
            0xAD => self.alu_instructions.xor(opcode),
            0xAE => self.alu_instructions.xor(opcode),
            0xAF => self.alu_instructions.xor(opcode),
            0xB0 => self.alu_instructions.or(opcode),
            0xB1 => self.alu_instructions.or(opcode),
            0xB2 => self.alu_instructions.or(opcode),
            0xB3 => self.alu_instructions.or(opcode),
            0xB4 => self.alu_instructions.or(opcode),
            0xB5 => self.alu_instructions.or(opcode),
            0xB6 => self.alu_instructions.or(opcode),
            0xB7 => self.alu_instructions.or(opcode),
            0xB8 => self.alu_instructions.cp(opcode),
            0xB9 => self.alu_instructions.cp(opcode),
            0xBA => self.alu_instructions.cp(opcode),
            0xBB => self.alu_instructions.cp(opcode),
            0xBC => self.alu_instructions.cp(opcode),
            0xBD => self.alu_instructions.cp(opcode),
            0xBE => self.alu_instructions.cp(opcode),
            0xBF => self.alu_instructions.cp(opcode),
            0xC0 => self.branch_instructions.ret(opcode, &mut self.ei),
            0xC1 => self.lsm_instructions.pop(opcode),
            0xC2 => self.branch_instructions.jp(opcode),
            0xC3 => self.branch_instructions.jp(opcode),
            0xC4 => self.branch_instructions.call(opcode),
            0xC5 => self.lsm_instructions.push(opcode),
            0xC6 => self.alu_instructions.add_8(opcode),
            0xC7 => self.branch_instructions.rst(opcode),
            0xC8 => self.branch_instructions.ret(opcode, &mut self.ei),
            0xC9 => self.branch_instructions.ret(opcode, &mut self.ei),
            0xCA => self.branch_instructions.jp(opcode),
            0xCB => opcode.tcycles.0 + self.execute_cb(), // ?? not sure on this
            0xCC => self.branch_instructions.call(opcode),
            0xCD => self.branch_instructions.call(opcode),
            0xCE => self.alu_instructions.adc(opcode),
            0xCF => self.branch_instructions.rst(opcode),
            0xD0 => self.branch_instructions.ret(opcode, &mut self.ei),
            0xD1 => self.lsm_instructions.pop(opcode),
            0xD2 => self.branch_instructions.jp(opcode),
            0xD4 => self.branch_instructions.call(opcode),
            0xD5 => self.lsm_instructions.push(opcode),
            0xD6 => self.alu_instructions.sub(opcode),
            0xD7 => self.branch_instructions.rst(opcode),
            0xD8 => self.branch_instructions.ret(opcode, &mut self.ei),
            0xD9 => self.branch_instructions.ret(opcode, &mut self.ei),
            0xDA => self.branch_instructions.jp(opcode),
            0xDC => self.branch_instructions.call(opcode),
            0xDE => self.alu_instructions.sbc(opcode),
            0xDF => self.branch_instructions.rst(opcode),
            0xE0 => self.lsm_instructions.ld_8(opcode),
            0xE1 => self.lsm_instructions.pop(opcode),
            0xE2 => self.lsm_instructions.ld_8(opcode),
            0xE5 => self.lsm_instructions.push(opcode),
            0xE6 => self.alu_instructions.and(opcode),
            0xE7 => self.branch_instructions.rst(opcode),
            0xE8 => self.alu_instructions.add_16(opcode),
            0xE9 => self.branch_instructions.jp(opcode),
            0xEA => self.lsm_instructions.ld_8(opcode),
            0xEE => self.alu_instructions.xor(opcode),
            0xEF => self.branch_instructions.rst(opcode),
            0xF0 => self.lsm_instructions.ld_8(opcode),
            0xF1 => self.lsm_instructions.pop(opcode),
            0xF2 => self.lsm_instructions.ld_8(opcode),
            0xF3 => self.misc_instructions.di(opcode, &mut self.di),
            0xF5 => self.lsm_instructions.push(opcode),
            0xF6 => self.alu_instructions.or(opcode),
            0xF7 => self.branch_instructions.rst(opcode),
            0xF8 => self.alu_instructions.add_16(opcode),
            0xF9 => self.branch_instructions.jp(opcode),
            0xFA => self.lsm_instructions.ld_8(opcode),
            0xFB => self.misc_instructions.ei(opcode, &mut self.ei),
            0xFE => self.alu_instructions.xor(opcode),
            0xFF => self.branch_instructions.rst(opcode),
            code => panic!("Code {:#04X} not implemented", code),
        }
    }

    fn execute_cb(&mut self) -> u8 {
        let value = self.fetch_byte();
        let opcode = CB_PREFIXED_OPCODES_MAP.get(&value).unwrap();
        match opcode.value {
            code => panic!("Code {:#04X} not implemented", code),
        }
    }
}
