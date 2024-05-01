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
    alu_instructions::AluInstructions, branch_instructions::BranchInstructions, lsm_instructions::LsmInstructions,
    misc_instructions::MiscInstructions, rsb_instructions::RsbInstructions,
};

trait Instructions {
    fn get_operands<'a>(&self, mnemonic: &'a str) -> &'a str {
        let operand: &str = mnemonic.split_whitespace().nth(1).unwrap_or_default();
        operand
    }

    fn fetch_byte(registers: &mut Registers, bus: &mut Bus) -> u8 {
        let byte = bus.mem_read(registers.pc);
        registers.pc += 1;
        byte
    }

    fn fetch_word(registers: &mut Registers, bus: &mut Bus) -> u16 {
        let word = bus.mem_read_16(registers.pc);
        registers.pc += 2;
        word
    }

    fn pop_stack(registers: &mut Registers, bus: &mut Bus) -> u16 {
        let data = bus.mem_read_16(registers.sp);
        registers.sp = registers.sp.wrapping_add(2);
        data
    }

    fn push_stack(data: u16, registers: &mut Registers, bus: &mut Bus) {
        registers.sp = registers.sp.wrapping_sub(2);
        bus.mem_write_16(registers.sp, data);
    }
}

pub enum ImeState {
    Disable,
    Enable,
    Staged,
    NoChange,
}

pub struct Cpu {
    registers: Registers,
    bus: Bus,
    halted: bool,
    ime: bool,
    ei: ImeState,
    di: ImeState,
    lsm_instructions: LsmInstructions,
    alu_instructions: AluInstructions,
    rsb_instructions: RsbInstructions,
    branch_instructions: BranchInstructions,
    misc_instructions: MiscInstructions,
}

impl Memory for Cpu {
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

impl Cpu {
    pub fn new(registers: Registers, bus: Bus) -> Self {
        Cpu {
            registers,
            bus,
            halted: false,
            ime: false,
            ei: ImeState::NoChange,
            di: ImeState::NoChange,
            lsm_instructions: LsmInstructions::new(),
            alu_instructions: AluInstructions::new(),
            rsb_instructions: RsbInstructions::new(),
            branch_instructions: BranchInstructions::new(),
            misc_instructions: MiscInstructions::new(),
        }
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
        self.registers.sp = self.registers.sp.wrapping_sub(2);
        self.mem_write_16(self.registers.sp, program_counter);
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
        let value = self.bus.mem_read(self.registers.pc);
        self.registers.pc += 1;

        let opcode = UNPREFIXED_OPCODES_MAP.get(&value).unwrap();
        match value {
            0x00 => self.misc_instructions.nop(opcode),
            0x01 => self.lsm_instructions.ld_16(opcode, &mut self.registers, &mut self.bus),
            0x02 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x03 => self.alu_instructions.inc_16(opcode, &mut self.registers),
            0x04 => self.alu_instructions.inc_8(opcode, &mut self.registers, &mut self.bus),
            0x05 => self.alu_instructions.dec_8(opcode, &mut self.registers, &mut self.bus),
            0x06 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x07 => self.rsb_instructions.rlca(opcode, &mut self.registers),
            0x08 => self.lsm_instructions.ld_16(opcode, &mut self.registers, &mut self.bus),
            0x09 => self.alu_instructions.add_16(opcode, &mut self.registers),
            0x0A => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x0B => self.alu_instructions.dec_16(opcode, &mut self.registers),
            0x0C => self.alu_instructions.inc_8(opcode, &mut self.registers, &mut self.bus),
            0x0D => self.alu_instructions.dec_8(opcode, &mut self.registers, &mut self.bus),
            0x0E => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x0F => self.rsb_instructions.rrca(opcode, &mut self.registers),
            0x10 => self.misc_instructions.stop(opcode),
            0x11 => self.lsm_instructions.ld_16(opcode, &mut self.registers, &mut self.bus),
            0x12 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x13 => self.alu_instructions.inc_16(opcode, &mut self.registers),
            0x14 => self.alu_instructions.inc_8(opcode, &mut self.registers, &mut self.bus),
            0x15 => self.alu_instructions.dec_8(opcode, &mut self.registers, &mut self.bus),
            0x16 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x17 => self.rsb_instructions.rla(opcode, &mut self.registers),
            0x18 => self.branch_instructions.jr(opcode, &mut self.registers, &mut self.bus),
            0x19 => self.alu_instructions.add_16(opcode, &mut self.registers),
            0x1A => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x1B => self.alu_instructions.dec_16(opcode, &mut self.registers),
            0x1C => self.alu_instructions.inc_8(opcode, &mut self.registers, &mut self.bus),
            0x1D => self.alu_instructions.dec_8(opcode, &mut self.registers, &mut self.bus),
            0x1E => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x1F => self.rsb_instructions.rra(opcode, &mut self.registers),
            0x20 => self.branch_instructions.jr(opcode, &mut self.registers, &mut self.bus),
            0x21 => self.lsm_instructions.ld_16(opcode, &mut self.registers, &mut self.bus),
            0x22 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x23 => self.alu_instructions.inc_16(opcode, &mut self.registers),
            0x24 => self.alu_instructions.inc_8(opcode, &mut self.registers, &mut self.bus),
            0x25 => self.alu_instructions.dec_8(opcode, &mut self.registers, &mut self.bus),
            0x26 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x27 => self.alu_instructions.daa(opcode, &mut self.registers, &mut self.bus),
            0x28 => self.branch_instructions.jr(opcode, &mut self.registers, &mut self.bus),
            0x29 => self.alu_instructions.add_16(opcode, &mut self.registers),
            0x2A => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x2B => self.alu_instructions.dec_16(opcode, &mut self.registers),
            0x2C => self.alu_instructions.inc_8(opcode, &mut self.registers, &mut self.bus),
            0x2D => self.alu_instructions.dec_8(opcode, &mut self.registers, &mut self.bus),
            0x2E => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x2F => self.alu_instructions.cpl(opcode, &mut self.registers, &mut self.bus),
            0x30 => self.branch_instructions.jr(opcode, &mut self.registers, &mut self.bus),
            0x31 => self.lsm_instructions.ld_16(opcode, &mut self.registers, &mut self.bus),
            0x32 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x33 => self.alu_instructions.inc_16(opcode, &mut self.registers),
            0x34 => self.alu_instructions.inc_8(opcode, &mut self.registers, &mut self.bus),
            0x35 => self.alu_instructions.dec_8(opcode, &mut self.registers, &mut self.bus),
            0x36 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x37 => self.alu_instructions.scf(opcode, &mut self.registers, &mut self.bus),
            0x38 => self.branch_instructions.jr(opcode, &mut self.registers, &mut self.bus),
            0x39 => self.alu_instructions.add_16(opcode, &mut self.registers),
            0x3A => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x3B => self.alu_instructions.dec_16(opcode, &mut self.registers),
            0x3C => self.alu_instructions.inc_8(opcode, &mut self.registers, &mut self.bus),
            0x3D => self.alu_instructions.dec_8(opcode, &mut self.registers, &mut self.bus),
            0x3E => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x3F => self.alu_instructions.ccf(opcode, &mut self.registers, &mut self.bus),
            0x40 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x41 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x42 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x43 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x44 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x45 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x46 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x47 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x48 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x49 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x4A => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x4B => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x4C => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x4D => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x4E => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x4F => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x50 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x51 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x52 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x53 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x54 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x55 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x56 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x57 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x58 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x59 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x5A => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x5B => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x5C => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x5D => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x5E => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x5F => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x60 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x61 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x62 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x63 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x64 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x65 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x66 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x67 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x68 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x69 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x6A => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x6B => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x6C => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x6D => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x6E => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x6F => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x70 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x71 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x72 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x73 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x74 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x75 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x76 => self.misc_instructions.halt(opcode, &mut self.halted),
            0x77 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x78 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x79 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x7A => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x7B => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x7C => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x7D => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x7E => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x7F => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0x80 => self.alu_instructions.add_8(opcode, &mut self.registers, &mut self.bus),
            0x81 => self.alu_instructions.add_8(opcode, &mut self.registers, &mut self.bus),
            0x82 => self.alu_instructions.add_8(opcode, &mut self.registers, &mut self.bus),
            0x83 => self.alu_instructions.add_8(opcode, &mut self.registers, &mut self.bus),
            0x84 => self.alu_instructions.add_8(opcode, &mut self.registers, &mut self.bus),
            0x85 => self.alu_instructions.add_8(opcode, &mut self.registers, &mut self.bus),
            0x86 => self.alu_instructions.add_8(opcode, &mut self.registers, &mut self.bus),
            0x87 => self.alu_instructions.add_8(opcode, &mut self.registers, &mut self.bus),
            0x88 => self.alu_instructions.adc(opcode, &mut self.registers, &mut self.bus),
            0x89 => self.alu_instructions.adc(opcode, &mut self.registers, &mut self.bus),
            0x8A => self.alu_instructions.adc(opcode, &mut self.registers, &mut self.bus),
            0x8B => self.alu_instructions.adc(opcode, &mut self.registers, &mut self.bus),
            0x8C => self.alu_instructions.adc(opcode, &mut self.registers, &mut self.bus),
            0x8E => self.alu_instructions.adc(opcode, &mut self.registers, &mut self.bus),
            0x8D => self.alu_instructions.adc(opcode, &mut self.registers, &mut self.bus),
            0x8F => self.alu_instructions.adc(opcode, &mut self.registers, &mut self.bus),
            0x90 => self.alu_instructions.sub(opcode, &mut self.registers, &mut self.bus),
            0x91 => self.alu_instructions.sub(opcode, &mut self.registers, &mut self.bus),
            0x92 => self.alu_instructions.sub(opcode, &mut self.registers, &mut self.bus),
            0x93 => self.alu_instructions.sub(opcode, &mut self.registers, &mut self.bus),
            0x94 => self.alu_instructions.sub(opcode, &mut self.registers, &mut self.bus),
            0x95 => self.alu_instructions.sub(opcode, &mut self.registers, &mut self.bus),
            0x96 => self.alu_instructions.sub(opcode, &mut self.registers, &mut self.bus),
            0x97 => self.alu_instructions.sub(opcode, &mut self.registers, &mut self.bus),
            0x98 => self.alu_instructions.sbc(opcode, &mut self.registers, &mut self.bus),
            0x99 => self.alu_instructions.sbc(opcode, &mut self.registers, &mut self.bus),
            0x9A => self.alu_instructions.sbc(opcode, &mut self.registers, &mut self.bus),
            0x9B => self.alu_instructions.sbc(opcode, &mut self.registers, &mut self.bus),
            0x9C => self.alu_instructions.sbc(opcode, &mut self.registers, &mut self.bus),
            0x9E => self.alu_instructions.sbc(opcode, &mut self.registers, &mut self.bus),
            0x9D => self.alu_instructions.sbc(opcode, &mut self.registers, &mut self.bus),
            0x9F => self.alu_instructions.sbc(opcode, &mut self.registers, &mut self.bus),
            0xA0 => self.alu_instructions.and(opcode, &mut self.registers, &mut self.bus),
            0xA1 => self.alu_instructions.and(opcode, &mut self.registers, &mut self.bus),
            0xA2 => self.alu_instructions.and(opcode, &mut self.registers, &mut self.bus),
            0xA3 => self.alu_instructions.and(opcode, &mut self.registers, &mut self.bus),
            0xA4 => self.alu_instructions.and(opcode, &mut self.registers, &mut self.bus),
            0xA5 => self.alu_instructions.and(opcode, &mut self.registers, &mut self.bus),
            0xA6 => self.alu_instructions.and(opcode, &mut self.registers, &mut self.bus),
            0xA7 => self.alu_instructions.and(opcode, &mut self.registers, &mut self.bus),
            0xA8 => self.alu_instructions.xor(opcode, &mut self.registers, &mut self.bus),
            0xA9 => self.alu_instructions.xor(opcode, &mut self.registers, &mut self.bus),
            0xAA => self.alu_instructions.xor(opcode, &mut self.registers, &mut self.bus),
            0xAB => self.alu_instructions.xor(opcode, &mut self.registers, &mut self.bus),
            0xAC => self.alu_instructions.xor(opcode, &mut self.registers, &mut self.bus),
            0xAD => self.alu_instructions.xor(opcode, &mut self.registers, &mut self.bus),
            0xAE => self.alu_instructions.xor(opcode, &mut self.registers, &mut self.bus),
            0xAF => self.alu_instructions.xor(opcode, &mut self.registers, &mut self.bus),
            0xB0 => self.alu_instructions.or(opcode, &mut self.registers, &mut self.bus),
            0xB1 => self.alu_instructions.or(opcode, &mut self.registers, &mut self.bus),
            0xB2 => self.alu_instructions.or(opcode, &mut self.registers, &mut self.bus),
            0xB3 => self.alu_instructions.or(opcode, &mut self.registers, &mut self.bus),
            0xB4 => self.alu_instructions.or(opcode, &mut self.registers, &mut self.bus),
            0xB5 => self.alu_instructions.or(opcode, &mut self.registers, &mut self.bus),
            0xB6 => self.alu_instructions.or(opcode, &mut self.registers, &mut self.bus),
            0xB7 => self.alu_instructions.or(opcode, &mut self.registers, &mut self.bus),
            0xB8 => self.alu_instructions.cp(opcode, &mut self.registers, &mut self.bus),
            0xB9 => self.alu_instructions.cp(opcode, &mut self.registers, &mut self.bus),
            0xBA => self.alu_instructions.cp(opcode, &mut self.registers, &mut self.bus),
            0xBB => self.alu_instructions.cp(opcode, &mut self.registers, &mut self.bus),
            0xBC => self.alu_instructions.cp(opcode, &mut self.registers, &mut self.bus),
            0xBD => self.alu_instructions.cp(opcode, &mut self.registers, &mut self.bus),
            0xBE => self.alu_instructions.cp(opcode, &mut self.registers, &mut self.bus),
            0xBF => self.alu_instructions.cp(opcode, &mut self.registers, &mut self.bus),
            0xC0 => self.branch_instructions.ret(opcode, &mut self.registers, &mut self.bus, &mut self.ei),
            0xC1 => self.lsm_instructions.pop(opcode, &mut self.registers, &mut self.bus),
            0xC2 => self.branch_instructions.jp(opcode, &mut self.registers, &mut self.bus),
            0xC3 => self.branch_instructions.jp(opcode, &mut self.registers, &mut self.bus),
            0xC4 => self.branch_instructions.call(opcode, &mut self.registers, &mut self.bus),
            0xC5 => self.lsm_instructions.push(opcode, &mut self.registers, &mut self.bus),
            0xC6 => self.alu_instructions.add_8(opcode, &mut self.registers, &mut self.bus),
            0xC7 => self.branch_instructions.rst(opcode, &mut self.registers, &mut self.bus),
            0xC8 => self.branch_instructions.ret(opcode, &mut self.registers, &mut self.bus, &mut self.ei),
            0xC9 => self.branch_instructions.ret(opcode, &mut self.registers, &mut self.bus, &mut self.ei),
            0xCA => self.branch_instructions.jp(opcode, &mut self.registers, &mut self.bus),
            0xCB => opcode.tcycles.0 + self.execute_cb(), // ?? not sure on this
            0xCC => self.branch_instructions.call(opcode, &mut self.registers, &mut self.bus),
            0xCD => self.branch_instructions.call(opcode, &mut self.registers, &mut self.bus),
            0xCE => self.alu_instructions.adc(opcode, &mut self.registers, &mut self.bus),
            0xCF => self.branch_instructions.rst(opcode, &mut self.registers, &mut self.bus),
            0xD0 => self.branch_instructions.ret(opcode, &mut self.registers, &mut self.bus, &mut self.ei),
            0xD1 => self.lsm_instructions.pop(opcode, &mut self.registers, &mut self.bus),
            0xD2 => self.branch_instructions.jp(opcode, &mut self.registers, &mut self.bus),
            0xD4 => self.branch_instructions.call(opcode, &mut self.registers, &mut self.bus),
            0xD5 => self.lsm_instructions.push(opcode, &mut self.registers, &mut self.bus),
            0xD6 => self.alu_instructions.sub(opcode, &mut self.registers, &mut self.bus),
            0xD7 => self.branch_instructions.rst(opcode, &mut self.registers, &mut self.bus),
            0xD8 => self.branch_instructions.ret(opcode, &mut self.registers, &mut self.bus, &mut self.ei),
            0xD9 => self.branch_instructions.ret(opcode, &mut self.registers, &mut self.bus, &mut self.ei),
            0xDA => self.branch_instructions.jp(opcode, &mut self.registers, &mut self.bus),
            0xDC => self.branch_instructions.call(opcode, &mut self.registers, &mut self.bus),
            0xDE => self.alu_instructions.sbc(opcode, &mut self.registers, &mut self.bus),
            0xDF => self.branch_instructions.rst(opcode, &mut self.registers, &mut self.bus),
            0xE0 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0xE1 => self.lsm_instructions.pop(opcode, &mut self.registers, &mut self.bus),
            0xE2 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0xE5 => self.lsm_instructions.push(opcode, &mut self.registers, &mut self.bus),
            0xE6 => self.alu_instructions.and(opcode, &mut self.registers, &mut self.bus),
            0xE7 => self.branch_instructions.rst(opcode, &mut self.registers, &mut self.bus),
            0xE8 => self.alu_instructions.add_16(opcode, &mut self.registers),
            0xE9 => self.branch_instructions.jp(opcode, &mut self.registers, &mut self.bus),
            0xEA => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0xEE => self.alu_instructions.xor(opcode, &mut self.registers, &mut self.bus),
            0xEF => self.branch_instructions.rst(opcode, &mut self.registers, &mut self.bus),
            0xF0 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0xF1 => self.lsm_instructions.pop(opcode, &mut self.registers, &mut self.bus),
            0xF2 => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0xF3 => self.misc_instructions.di(opcode, &mut self.di),
            0xF5 => self.lsm_instructions.push(opcode, &mut self.registers, &mut self.bus),
            0xF6 => self.alu_instructions.or(opcode, &mut self.registers, &mut self.bus),
            0xF7 => self.branch_instructions.rst(opcode, &mut self.registers, &mut self.bus),
            0xF8 => self.alu_instructions.add_16(opcode, &mut self.registers),
            0xF9 => self.branch_instructions.jp(opcode, &mut self.registers, &mut self.bus),
            0xFA => self.lsm_instructions.ld_8(opcode, &mut self.registers, &mut self.bus),
            0xFB => self.misc_instructions.ei(opcode, &mut self.ei),
            0xFE => self.alu_instructions.xor(opcode, &mut self.registers, &mut self.bus),
            0xFF => self.branch_instructions.rst(opcode, &mut self.registers, &mut self.bus),
            code => panic!("Code {:#04X} not implemented", code),
        }
    }

    fn execute_cb(&mut self) -> u8 {
        let value = self.bus.mem_read(self.registers.pc);
        self.registers.pc += 1;

        let opcode = CB_PREFIXED_OPCODES_MAP.get(&value).unwrap();
        match opcode.value {
            0x00 => self.rsb_instructions.rlc(opcode, &mut self.registers, &mut self.bus),
            0x01 => self.rsb_instructions.rlc(opcode, &mut self.registers, &mut self.bus),
            0x02 => self.rsb_instructions.rlc(opcode, &mut self.registers, &mut self.bus),
            0x03 => self.rsb_instructions.rlc(opcode, &mut self.registers, &mut self.bus),
            0x04 => self.rsb_instructions.rlc(opcode, &mut self.registers, &mut self.bus),
            0x05 => self.rsb_instructions.rlc(opcode, &mut self.registers, &mut self.bus),
            0x06 => self.rsb_instructions.rlc(opcode, &mut self.registers, &mut self.bus),
            0x07 => self.rsb_instructions.rlc(opcode, &mut self.registers, &mut self.bus),
            code => panic!("Code {:#04X} not implemented", code),
        }
    }
}
