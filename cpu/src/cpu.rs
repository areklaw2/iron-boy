use std::collections::HashMap;

use crate::{
    bus::{Bus, Memory},
    opcodes::{self, OpCode},
    registers::{CpuFlag, Registers},
};

enum ImeState {
    Disable,
    Enable,
    Staged,
    NoChange,
}

pub struct Cpu {
    registers: Registers,
    bus: Bus,
    unprefixed_opcodes: HashMap<u8, OpCode>,
    cb_prefixed_opcodes: HashMap<u8, OpCode>,
    halted: bool,
    ime: bool,
    ei: ImeState,
    di: ImeState,
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
            unprefixed_opcodes: opcodes::get_unprefixed_opcodes_map(),
            cb_prefixed_opcodes: opcodes::get_cb_prefixed_opcodes_map(),
            halted: false,
            ime: false,
            ei: ImeState::NoChange,
            di: ImeState::NoChange,
        }
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.mem_read(self.registers.pc);
        self.registers.pc += 1;
        byte
    }

    fn fetch_word(&mut self) -> u16 {
        let word = self.mem_read_16(self.registers.pc);
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

    fn execute(&mut self) -> u8 {
        let byte = self.fetch_byte();
        let opcode = *self.unprefixed_opcodes.get(&byte).unwrap();
        match opcode.value {
            0x00 => opcode.tcycles.0,
            0x01 => self.ld_16(opcode),
            0x02 => self.ld_8(opcode),
            0x03 => self.inc_16(opcode),
            0x04 => self.inc_8(opcode),
            0x05 => self.dec_8(opcode),
            0x06 => self.ld_8(opcode),
            0x07 => self.rlca(opcode),
            0x08 => self.ld_16(opcode),
            0x09 => self.add_16(opcode),
            0x0A => self.ld_8(opcode),
            0x0B => self.dec_16(opcode),
            0x0C => self.inc_8(opcode),
            0x0D => self.dec_8(opcode),
            0x0E => self.ld_8(opcode),
            0x0F => self.rrca(opcode),
            0x10 => self.stop(opcode),
            0x11 => self.ld_16(opcode),
            0x12 => self.ld_8(opcode),
            0x13 => self.inc_16(opcode),
            0x14 => self.inc_8(opcode),
            0x15 => self.dec_8(opcode),
            0x16 => self.ld_8(opcode),
            0x17 => self.rla(opcode),
            0x18 => self.jr(opcode),
            0x19 => self.add_16(opcode),
            0x1A => self.ld_8(opcode),
            0x1B => self.dec_16(opcode),
            0x1C => self.inc_8(opcode),
            0x1D => self.dec_8(opcode),
            0x1E => self.ld_8(opcode),
            0x1F => self.rra(opcode),
            0x20 => self.jr(opcode),
            0x21 => self.ld_16(opcode),
            0x22 => self.ld_8(opcode),
            0x23 => self.inc_16(opcode),
            0x24 => self.inc_8(opcode),
            0x25 => self.dec_8(opcode),
            0x26 => self.ld_8(opcode),
            0x27 => self.daa(opcode),
            0x28 => self.jr(opcode),
            0x29 => self.add_16(opcode),
            0x2A => self.ld_8(opcode),
            0x2B => self.dec_16(opcode),
            0x2C => self.inc_8(opcode),
            0x2D => self.dec_8(opcode),
            0x2E => self.ld_8(opcode),
            0x2F => self.cpl(opcode),
            0x30 => self.jr(opcode),
            0x31 => self.ld_16(opcode),
            0x32 => self.ld_8(opcode),
            0x33 => self.inc_16(opcode),
            0x34 => self.inc_8(opcode),
            0x35 => self.dec_8(opcode),
            0x36 => self.ld_8(opcode),
            0x37 => self.scf(opcode),
            0x38 => self.jr(opcode),
            0x39 => self.add_16(opcode),
            0x3A => self.ld_8(opcode),
            0x3B => self.dec_16(opcode),
            0x3C => self.inc_8(opcode),
            0x3D => self.dec_8(opcode),
            0x3E => self.ld_8(opcode),
            0x3F => self.ccf(opcode),
            0x40 => self.ld_8(opcode),
            0x41 => self.ld_8(opcode),
            0x42 => self.ld_8(opcode),
            0x43 => self.ld_8(opcode),
            0x44 => self.ld_8(opcode),
            0x45 => self.ld_8(opcode),
            0x46 => self.ld_8(opcode),
            0x47 => self.ld_8(opcode),
            0x48 => self.ld_8(opcode),
            0x49 => self.ld_8(opcode),
            0x4A => self.ld_8(opcode),
            0x4B => self.ld_8(opcode),
            0x4C => self.ld_8(opcode),
            0x4D => self.ld_8(opcode),
            0x4E => self.ld_8(opcode),
            0x4F => self.ld_8(opcode),
            0x50 => self.ld_8(opcode),
            0x51 => self.ld_8(opcode),
            0x52 => self.ld_8(opcode),
            0x53 => self.ld_8(opcode),
            0x54 => self.ld_8(opcode),
            0x55 => self.ld_8(opcode),
            0x56 => self.ld_8(opcode),
            0x57 => self.ld_8(opcode),
            0x58 => self.ld_8(opcode),
            0x59 => self.ld_8(opcode),
            0x5A => self.ld_8(opcode),
            0x5B => self.ld_8(opcode),
            0x5C => self.ld_8(opcode),
            0x5D => self.ld_8(opcode),
            0x5E => self.ld_8(opcode),
            0x5F => self.ld_8(opcode),
            0x60 => self.ld_8(opcode),
            0x61 => self.ld_8(opcode),
            0x62 => self.ld_8(opcode),
            0x63 => self.ld_8(opcode),
            0x64 => self.ld_8(opcode),
            0x65 => self.ld_8(opcode),
            0x66 => self.ld_8(opcode),
            0x67 => self.ld_8(opcode),
            0x68 => self.ld_8(opcode),
            0x69 => self.ld_8(opcode),
            0x6A => self.ld_8(opcode),
            0x6B => self.ld_8(opcode),
            0x6C => self.ld_8(opcode),
            0x6D => self.ld_8(opcode),
            0x6E => self.ld_8(opcode),
            0x6F => self.ld_8(opcode),
            0x70 => self.ld_8(opcode),
            0x71 => self.ld_8(opcode),
            0x72 => self.ld_8(opcode),
            0x73 => self.ld_8(opcode),
            0x74 => self.ld_8(opcode),
            0x75 => self.ld_8(opcode),
            0x76 => self.halt(opcode),
            0x77 => self.ld_8(opcode),
            0x78 => self.ld_8(opcode),
            0x79 => self.ld_8(opcode),
            0x7A => self.ld_8(opcode),
            0x7B => self.ld_8(opcode),
            0x7C => self.ld_8(opcode),
            0x7D => self.ld_8(opcode),
            0x7E => self.ld_8(opcode),
            0x7F => self.ld_8(opcode),
            0x80 => self.add_8(opcode),
            0x81 => self.add_8(opcode),
            0x82 => self.add_8(opcode),
            0x83 => self.add_8(opcode),
            0x84 => self.add_8(opcode),
            0x85 => self.add_8(opcode),
            0x86 => self.add_8(opcode),
            0x87 => self.add_8(opcode),
            0x88 => self.adc(opcode),
            0x89 => self.adc(opcode),
            0x8A => self.adc(opcode),
            0x8B => self.adc(opcode),
            0x8C => self.adc(opcode),
            0x8E => self.adc(opcode),
            0x8D => self.adc(opcode),
            0x8F => self.adc(opcode),
            0x90 => self.sub(opcode),
            0x91 => self.sub(opcode),
            0x92 => self.sub(opcode),
            0x93 => self.sub(opcode),
            0x94 => self.sub(opcode),
            0x95 => self.sub(opcode),
            0x96 => self.sub(opcode),
            0x97 => self.sub(opcode),
            0x98 => self.sbc(opcode),
            0x99 => self.sbc(opcode),
            0x9A => self.sbc(opcode),
            0x9B => self.sbc(opcode),
            0x9C => self.sbc(opcode),
            0x9E => self.sbc(opcode),
            0x9D => self.sbc(opcode),
            0x9F => self.sbc(opcode),
            0xA0 => self.and(opcode),
            0xA1 => self.and(opcode),
            0xA2 => self.and(opcode),
            0xA3 => self.and(opcode),
            0xA4 => self.and(opcode),
            0xA5 => self.and(opcode),
            0xA6 => self.and(opcode),
            0xA7 => self.and(opcode),
            0xA8 => self.xor(opcode),
            0xA9 => self.xor(opcode),
            0xAA => self.xor(opcode),
            0xAB => self.xor(opcode),
            0xAC => self.xor(opcode),
            0xAD => self.xor(opcode),
            0xAE => self.xor(opcode),
            0xAF => self.xor(opcode),
            0xB0 => self.or(opcode),
            0xB1 => self.or(opcode),
            0xB2 => self.or(opcode),
            0xB3 => self.or(opcode),
            0xB4 => self.or(opcode),
            0xB5 => self.or(opcode),
            0xB6 => self.or(opcode),
            0xB7 => self.or(opcode),
            0xB8 => self.cp(opcode),
            0xB9 => self.cp(opcode),
            0xBA => self.cp(opcode),
            0xBB => self.cp(opcode),
            0xBC => self.cp(opcode),
            0xBD => self.cp(opcode),
            0xBE => self.cp(opcode),
            0xBF => self.cp(opcode),
            0xC0 => self.ret(opcode),
            0xC1 => self.pop(opcode),

            code => panic!("Code {:#04X} not implemented", code),
        }
    }

    fn execute_cb(&self, opcode: u8) {
        todo!()
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

    fn get_operands<'a>(&self, mnemonic: &'a str) -> &'a str {
        let operand: &str = mnemonic.split_whitespace().nth(1).unwrap_or_default();
        operand
    }

    fn ld_16(&mut self, opcode: OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        match operands {
            "BC,u16" => {
                let value = self.fetch_word();
                self.registers.set_bc(value)
            }
            "(u16),SP" => {
                let address = self.fetch_word();
                self.mem_write_16(address, self.registers.sp);
            }
            "DE,u16" => {
                let value = self.fetch_word();
                self.registers.set_de(value)
            }
            "HL,u16" => {
                let value = self.fetch_word();
                self.registers.set_hl(value)
            }
            "SP,u16" => self.registers.sp = self.fetch_word(),
            "SP,HL" => self.registers.sp = self.registers.hl(),
            op => panic!("Operands not valid: {op}"),
        }
        opcode.tcycles.0
    }

    fn ld_8(&mut self, opcode: OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        match operands {
            "(BC),A" => self.mem_write(self.registers.bc(), self.registers.a),
            "B,u8" => self.registers.b = self.fetch_byte(),
            "A,(BC)" => self.registers.a = self.mem_read(self.registers.bc()),
            "C,u8" => self.registers.c = self.fetch_byte(),
            "(DE),A" => self.mem_write(self.registers.de(), self.registers.a),
            "D,u8" => self.registers.d = self.fetch_byte(),
            "A,(DE)" => self.registers.a = self.mem_read(self.registers.de()),
            "E,u8" => self.registers.e = self.fetch_byte(),
            "(HL+),A" => {
                let address = self.registers.increment_hl();
                self.mem_write(address, self.registers.a);
            }
            "H,u8" => self.registers.h = self.fetch_byte(),
            "A,(HL+)" => {
                let address = self.registers.increment_hl();
                self.registers.a = self.mem_read(address);
            }
            "L,u8" => self.registers.l = self.fetch_byte(),
            "(HL-),A" => {
                let address = self.registers.decrement_hl();
                self.mem_write(address, self.registers.a);
            }
            "(HL),u8" => {
                let value = self.fetch_byte();
                self.mem_write(self.registers.hl(), value);
            }
            "A,(HL-)" => {
                let address = self.registers.decrement_hl();
                self.registers.a = self.mem_read(address);
            }
            "A,u8" => self.registers.a = self.fetch_byte(),
            "B,B" => {}
            "B,C" => self.registers.b = self.registers.c,
            "B,D" => self.registers.b = self.registers.d,
            "B,E" => self.registers.b = self.registers.e,
            "B,H" => self.registers.b = self.registers.h,
            "B,L" => self.registers.b = self.registers.l,
            "B,(HL)" => self.registers.b = self.mem_read(self.registers.hl()),
            "B,A" => self.registers.b = self.registers.a,
            "C,B" => self.registers.c = self.registers.b,
            "C,C" => {}
            "C,D" => self.registers.c = self.registers.d,
            "C,E" => self.registers.c = self.registers.e,
            "C,H" => self.registers.c = self.registers.h,
            "C,L" => self.registers.c = self.registers.l,
            "C,(HL)" => self.registers.c = self.mem_read(self.registers.hl()),
            "C,A" => self.registers.c = self.registers.a,
            "D,B" => self.registers.d = self.registers.b,
            "D,C" => self.registers.d = self.registers.c,
            "D,D" => {}
            "D,E" => self.registers.d = self.registers.e,
            "D,H" => self.registers.d = self.registers.h,
            "D,L" => self.registers.d = self.registers.l,
            "D,(HL)" => self.registers.d = self.mem_read(self.registers.hl()),
            "D,A" => self.registers.d = self.registers.a,
            "E,B" => self.registers.e = self.registers.b,
            "E,C" => self.registers.e = self.registers.c,
            "E,D" => self.registers.e = self.registers.d,
            "E,E" => {}
            "E,H" => self.registers.e = self.registers.h,
            "E,L" => self.registers.e = self.registers.l,
            "E,(HL)" => self.registers.e = self.mem_read(self.registers.hl()),
            "E,A" => self.registers.e = self.registers.a,
            "H,B" => self.registers.h = self.registers.b,
            "H,C" => self.registers.h = self.registers.c,
            "H,D" => self.registers.h = self.registers.d,
            "H,E" => self.registers.h = self.registers.e,
            "H,H" => {}
            "H,L" => self.registers.h = self.registers.l,
            "H,(HL)" => self.registers.h = self.mem_read(self.registers.hl()),
            "H,A" => self.registers.h = self.registers.a,
            "L,B" => self.registers.l = self.registers.b,
            "L,C" => self.registers.l = self.registers.c,
            "L,D" => self.registers.l = self.registers.d,
            "L,E" => self.registers.l = self.registers.e,
            "L,H" => self.registers.l = self.registers.h,
            "L,L" => {}
            "L,(HL)" => self.registers.l = self.mem_read(self.registers.hl()),
            "L,A" => self.registers.l = self.registers.a,
            "(HL),B" => self.mem_write(self.registers.hl(), self.registers.b),
            "(HL),C" => self.mem_write(self.registers.hl(), self.registers.c),
            "(HL),D" => self.mem_write(self.registers.hl(), self.registers.d),
            "(HL),E" => self.mem_write(self.registers.hl(), self.registers.e),
            "(HL),H" => self.mem_write(self.registers.hl(), self.registers.h),
            "(HL),L" => self.mem_write(self.registers.hl(), self.registers.l),
            "(HL),A" => self.mem_write(self.registers.hl(), self.registers.a),
            "A,B" => self.registers.a = self.registers.b,
            "A,C" => self.registers.a = self.registers.c,
            "A,D" => self.registers.a = self.registers.d,
            "A,E" => self.registers.a = self.registers.e,
            "A,H" => self.registers.a = self.registers.h,
            "A,L" => self.registers.a = self.registers.l,
            "A,(HL)" => self.registers.a = self.mem_read(self.registers.hl()),
            "A,A" => {}
            "(FF00 + u8),A" => {
                let address = 0xFF00 | self.fetch_byte() as u16;
                self.mem_write(address, self.registers.a);
            }
            "(FF00 + C),A" => {
                let address = 0xFF00 | self.registers.c as u16;
                self.mem_write(address, self.registers.a);
            }
            "(u16),A" => {
                let address = self.fetch_word();
                self.mem_write(address, self.registers.a)
            }
            "A,(FF00 + u8)" => {
                let address = 0xFF00 | self.fetch_byte() as u16;
                self.registers.a = self.mem_read(address)
            }
            "A,(FF00 + C)" => {
                let address = 0xFF00 | self.registers.c as u16;
                self.registers.a = self.mem_read(address)
            }
            "A,(u16)" => {
                let address = self.fetch_word();
                self.registers.a = self.mem_read(address)
            }
            op => panic!("Operands not valid: {op}"),
        }
        opcode.tcycles.0
    }

    fn pop(&mut self, opcode: OpCode) -> u8 {
        let operand = self.get_operands(opcode.mnemonic);
        match operand {
            "BC" => {
                let value = self.pop_stack();
                self.registers.set_bc(value);
            }
            "DE" => {
                let value = self.pop_stack();
                self.registers.set_de(value);
            }
            "HL" => {
                let value = self.pop_stack();
                self.registers.set_hl(value);
            }
            "AF" => {
                let value = self.pop_stack() & 0xFFF0;
                self.registers.set_af(value);
            }
            op => panic!("Operand not valid: {op}"),
        }
        opcode.tcycles.0
    }

    fn push(&mut self, opcode: OpCode) -> u8 {
        let operand = self.get_operands(opcode.mnemonic);
        match operand {
            "BC" => self.push_stack(self.registers.bc()),
            "DE" => self.push_stack(self.registers.de()),
            "HL" => self.push_stack(self.registers.hl()),
            "AF" => self.push_stack(self.registers.af()),
            op => panic!("Operand not valid: {op}"),
        }
        opcode.tcycles.0
    }

    fn inc_16(&mut self, opcode: OpCode) -> u8 {
        let operand = self.get_operands(opcode.mnemonic);
        match operand {
            "BC" => self.registers.set_bc(self.registers.bc().wrapping_add(1)),
            "DE" => self.registers.set_de(self.registers.de().wrapping_add(1)),
            "HL" => self.registers.set_hl(self.registers.hl().wrapping_add(1)),
            "SP" => self.registers.sp = self.registers.sp.wrapping_add(1),
            op => panic!("Operands not valid: {op}"),
        };
        opcode.tcycles.0
    }

    fn inc_8(&mut self, opcode: OpCode) -> u8 {
        let operand = self.get_operands(opcode.mnemonic);
        let data;
        let result;
        match operand {
            "B" => {
                data = self.registers.b;
                self.registers.b = data.wrapping_add(1);
                result = self.registers.b;
            }
            "C" => {
                data = self.registers.c;
                self.registers.c = data.wrapping_add(1);
                result = self.registers.c;
            }
            "D" => {
                data = self.registers.d;
                self.registers.d = data.wrapping_add(1);
                result = self.registers.d;
            }
            "E" => {
                data = self.registers.e;
                self.registers.e = data.wrapping_add(1);
                result = self.registers.e;
            }
            "H" => {
                data = self.registers.h;
                self.registers.h = data.wrapping_add(1);
                result = self.registers.h;
            }
            "L" => {
                data = self.registers.l;
                self.registers.l = data.wrapping_add(1);
                result = self.registers.l;
            }
            "(HL)" => {
                data = self.mem_read(self.registers.hl());
                self.mem_write(self.registers.hl(), data.wrapping_add(1));
                result = self.mem_read(self.registers.hl());
            }
            "A" => {
                data = self.registers.a;
                self.registers.a = data.wrapping_add(1);
                result = self.registers.a;
            }
            op => panic!("Operands not valid: {op}"),
        };

        self.registers.set_flag(CpuFlag::ZERO, result == 0);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers
            .set_flag(CpuFlag::HALF_CARRY, (data & 0x0F) + 1 > 0x0F);

        opcode.tcycles.0
    }

    fn dec_16(&mut self, opcode: OpCode) -> u8 {
        let operand = self.get_operands(opcode.mnemonic);
        match operand {
            "BC" => self.registers.set_bc(self.registers.bc().wrapping_sub(1)),
            "DE" => self.registers.set_de(self.registers.de().wrapping_sub(1)),
            "HL" => self.registers.set_hl(self.registers.hl().wrapping_sub(1)),
            "SP" => self.registers.sp = self.registers.sp.wrapping_sub(1),
            op => panic!("Operands not valid: {op}"),
        };
        opcode.tcycles.0
    }

    fn dec_8(&mut self, opcode: OpCode) -> u8 {
        let operand = self.get_operands(opcode.mnemonic);
        let data;
        let result;
        match operand {
            "B" => {
                data = self.registers.b;
                self.registers.b = data.wrapping_sub(1);
                result = self.registers.b;
            }
            "C" => {
                data = self.registers.c;
                self.registers.c = data.wrapping_sub(1);
                result = self.registers.c;
            }
            "D" => {
                data = self.registers.d;
                self.registers.d = data.wrapping_sub(1);
                result = self.registers.d;
            }
            "E" => {
                data = self.registers.e;
                self.registers.e = data.wrapping_sub(1);
                result = self.registers.e;
            }
            "H" => {
                data = self.registers.h;
                self.registers.h = data.wrapping_sub(1);
                result = self.registers.h;
            }
            "L" => {
                data = self.registers.l;
                self.registers.l = data.wrapping_sub(1);
                result = self.registers.l;
            }
            "(HL)" => {
                data = self.mem_read(self.registers.hl());
                self.mem_write(self.registers.hl(), data.wrapping_sub(1));
                result = self.mem_read(self.registers.hl());
            }
            "A" => {
                data = self.registers.a;
                self.registers.a = data.wrapping_sub(1);
                result = self.registers.a;
            }
            op => panic!("Operands not valid: {op}"),
        };

        self.registers.set_flag(CpuFlag::ZERO, result == 0);
        self.registers.set_flag(CpuFlag::SUBRACTION, true);
        self.registers
            .set_flag(CpuFlag::HALF_CARRY, (data & 0x0F) == 0);

        opcode.tcycles.0
    }

    fn add_16(&mut self, opcode: OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        match operands {
            "HL,BC" => {
                (data1, data2) = (self.registers.hl(), self.registers.bc());
                let result = data1.wrapping_add(data2);
                self.registers.set_hl(result);
            }
            "HL,DE" => {
                (data1, data2) = (self.registers.hl(), self.registers.de());
                let result = data1.wrapping_add(data2);
                self.registers.set_hl(result);
            }
            "HL,HL" => {
                (data1, data2) = (self.registers.hl(), self.registers.hl());
                let result = data1.wrapping_add(data2);
                self.registers.set_hl(result);
            }
            "HL,SP" => {
                (data1, data2) = (self.registers.hl(), self.registers.sp);
                let result = data1.wrapping_add(data2);
                self.registers.set_hl(result);
            }
            op => panic!("Operands not valid: {op}"),
        };

        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(
            CpuFlag::HALF_CARRY,
            (data1 & 0x07FF) + (data2 & 0x07FF) > 0x07FF,
        );
        self.registers
            .set_flag(CpuFlag::CARRY, data1 as u32 + data2 as u32 > 0xFFFF);
        opcode.tcycles.0
    }

    fn add_8(&mut self, opcode: OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        match operands {
            "A,B" => {
                (data1, data2) = (self.registers.a, self.registers.b);
                let result = data1.wrapping_add(data2);
                self.registers.a = result;
            }
            "A,C" => {
                (data1, data2) = (self.registers.a, self.registers.c);
                let result = data1.wrapping_add(data2);
                self.registers.a = result;
            }
            "A,D" => {
                (data1, data2) = (self.registers.a, self.registers.d);
                let result = data1.wrapping_add(data2);
                self.registers.a = result;
            }
            "A,E" => {
                (data1, data2) = (self.registers.a, self.registers.e);
                let result = data1.wrapping_add(data2);
                self.registers.a = result;
            }
            "A,H" => {
                (data1, data2) = (self.registers.a, self.registers.h);
                let result = data1.wrapping_add(data2);
                self.registers.a = result;
            }
            "A,L" => {
                (data1, data2) = (self.registers.a, self.registers.l);
                let result = data1.wrapping_add(data2);
                self.registers.a = result;
            }
            "A,(HL)" => {
                (data1, data2) = (self.registers.a, self.mem_read(self.registers.hl()));
                let result = data1.wrapping_add(data2);
                self.registers.a = result;
            }
            "A,A" => {
                (data1, data2) = (self.registers.a, self.registers.a);
                let result = data1.wrapping_add(data2);
                self.registers.a = result;
            }
            "A,u8" => {
                (data1, data2) = (self.registers.a, self.fetch_byte());
                let result = data1.wrapping_add(data2);
                self.registers.a = result;
            }
            op => panic!("Operands not valid: {op}"),
        }

        self.registers.set_flag(CpuFlag::ZERO, data1 + data2 == 0);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers
            .set_flag(CpuFlag::HALF_CARRY, (data1 & 0x0F) + (data2 & 0x0F) > 0x0F);
        self.registers
            .set_flag(CpuFlag::CARRY, data1 as u16 + data2 as u16 > 0xFF);
        opcode.tcycles.0
    }

    fn adc(&mut self, opcode: OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        let carry = if self.registers.f.contains(CpuFlag::CARRY) {
            1
        } else {
            0
        };
        match operands {
            "A,B" => {
                (data1, data2) = (self.registers.a, self.registers.b);
                let result = data1.wrapping_add(data2).wrapping_add(carry);
                self.registers.a = result;
            }
            "A,C" => {
                (data1, data2) = (self.registers.a, self.registers.c);
                let result = data1.wrapping_add(data2).wrapping_add(carry);
                self.registers.a = result;
            }
            "A,D" => {
                (data1, data2) = (self.registers.a, self.registers.d);
                let result = data1.wrapping_add(data2).wrapping_add(carry);
                self.registers.a = result;
            }
            "A,E" => {
                (data1, data2) = (self.registers.a, self.registers.e);
                let result = data1.wrapping_add(data2).wrapping_add(carry);
                self.registers.a = result;
            }
            "A,H" => {
                (data1, data2) = (self.registers.a, self.registers.h);
                let result = data1.wrapping_add(data2).wrapping_add(carry);
                self.registers.a = result;
            }
            "A,L" => {
                (data1, data2) = (self.registers.a, self.registers.l);
                let result = data1.wrapping_add(data2).wrapping_add(carry);
                self.registers.a = result;
            }
            "A,(HL)" => {
                (data1, data2) = (self.registers.a, self.mem_read(self.registers.hl()));
                let result = data1.wrapping_add(data2).wrapping_add(carry);
                self.registers.a = result;
            }
            "A,A" => {
                (data1, data2) = (self.registers.a, self.registers.a);
                let result = data1.wrapping_add(data2).wrapping_add(carry);
                self.registers.a = result;
            }
            "A,u8" => {
                (data1, data2) = (self.registers.a, self.fetch_byte());
                let result = data1.wrapping_add(data2).wrapping_add(carry);
                self.registers.a = result;
            }
            op => panic!("Operands not valid: {op}"),
        }

        self.registers
            .set_flag(CpuFlag::ZERO, data1 + data2 + carry == 0);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(
            CpuFlag::HALF_CARRY,
            (data1 & 0x0F) + (data2 & 0x0F) + carry > 0x0F,
        );
        self.registers.set_flag(
            CpuFlag::CARRY,
            data1 as u16 + data2 as u16 + carry as u16 > 0xFF,
        );
        opcode.tcycles.0
    }

    fn sub(&mut self, opcode: OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        match operands {
            "A,B" => {
                (data1, data2) = (self.registers.a, self.registers.b);
                let result = data1.wrapping_sub(data2);
                self.registers.a = result;
            }
            "A,C" => {
                (data1, data2) = (self.registers.a, self.registers.c);
                let result = data1.wrapping_sub(data2);
                self.registers.a = result;
            }
            "A,D" => {
                (data1, data2) = (self.registers.a, self.registers.d);
                let result = data1.wrapping_sub(data2);
                self.registers.a = result;
            }
            "A,E" => {
                (data1, data2) = (self.registers.a, self.registers.e);
                let result = data1.wrapping_sub(data2);
                self.registers.a = result;
            }
            "A,H" => {
                (data1, data2) = (self.registers.a, self.registers.h);
                let result = data1.wrapping_sub(data2);
                self.registers.a = result;
            }
            "A,L" => {
                (data1, data2) = (self.registers.a, self.registers.l);
                let result = data1.wrapping_sub(data2);
                self.registers.a = result;
            }
            "A,(HL)" => {
                (data1, data2) = (self.registers.a, self.mem_read(self.registers.hl()));
                let result = data1.wrapping_sub(data2);
                self.registers.a = result;
            }
            "A,A" => {
                (data1, data2) = (self.registers.a, self.registers.a);
                let result = data1.wrapping_sub(data2);
                self.registers.a = result;
            }
            "A,u8" => {
                (data1, data2) = (self.registers.a, self.fetch_byte());
                let result = data1.wrapping_sub(data2);
                self.registers.a = result;
            }
            op => panic!("Operands not valid: {op}"),
        }

        self.registers.set_flag(CpuFlag::ZERO, data1 - data2 == 0);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers
            .set_flag(CpuFlag::HALF_CARRY, (data1 & 0x0F) < (data2 & 0x0F));
        self.registers
            .set_flag(CpuFlag::CARRY, (data1 as u16) < (data2 as u16));
        opcode.tcycles.0
    }

    fn sbc(&mut self, opcode: OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        let carry = if self.registers.f.contains(CpuFlag::CARRY) {
            1
        } else {
            0
        };
        match operands {
            "A,B" => {
                (data1, data2) = (self.registers.a, self.registers.b);
                let result = data1.wrapping_sub(data2).wrapping_sub(carry);
                self.registers.a = result;
            }
            "A,C" => {
                (data1, data2) = (self.registers.a, self.registers.c);
                let result = data1.wrapping_sub(data2).wrapping_sub(carry);
                self.registers.a = result;
            }
            "A,D" => {
                (data1, data2) = (self.registers.a, self.registers.d);
                let result = data1.wrapping_sub(data2).wrapping_sub(carry);
                self.registers.a = result;
            }
            "A,E" => {
                (data1, data2) = (self.registers.a, self.registers.e);
                let result = data1.wrapping_sub(data2).wrapping_sub(carry);
                self.registers.a = result;
            }
            "A,H" => {
                (data1, data2) = (self.registers.a, self.registers.h);
                let result = data1.wrapping_sub(data2).wrapping_sub(carry);
                self.registers.a = result;
            }
            "A,L" => {
                (data1, data2) = (self.registers.a, self.registers.l);
                let result = data1.wrapping_sub(data2).wrapping_sub(carry);
                self.registers.a = result;
            }
            "A,(HL)" => {
                (data1, data2) = (self.registers.a, self.mem_read(self.registers.hl()));
                let result = data1.wrapping_sub(data2).wrapping_sub(carry);
                self.registers.a = result;
            }
            "A,A" => {
                (data1, data2) = (self.registers.a, self.registers.a);
                let result = data1.wrapping_sub(data2).wrapping_sub(carry);
                self.registers.a = result;
            }
            "A,u8" => {
                (data1, data2) = (self.registers.a, self.fetch_byte());
                let result = data1.wrapping_sub(data2).wrapping_sub(carry);
                self.registers.a = result;
            }
            op => panic!("Operands not valid: {op}"),
        }

        self.registers.set_flag(CpuFlag::ZERO, data1 - data2 == 0);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers
            .set_flag(CpuFlag::HALF_CARRY, (data1 & 0x0F) < (data2 & 0x0F) + carry);
        self.registers.set_flag(
            CpuFlag::CARRY,
            (data1 as u16) < (data2 as u16) + (carry as u16),
        );
        opcode.tcycles.0
    }

    fn and(&mut self, opcode: OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        match operands {
            "A,B" => {
                (data1, data2) = (self.registers.a, self.registers.b);
                let result = data1 & data2;
                self.registers.a = result;
            }
            "A,C" => {
                (data1, data2) = (self.registers.a, self.registers.c);
                let result = data1 & data2;
                self.registers.a = result;
            }
            "A,D" => {
                (data1, data2) = (self.registers.a, self.registers.d);
                let result = data1 & data2;
                self.registers.a = result;
            }
            "A,E" => {
                (data1, data2) = (self.registers.a, self.registers.e);
                let result = data1 & data2;
                self.registers.a = result;
            }
            "A,H" => {
                (data1, data2) = (self.registers.a, self.registers.h);
                let result = data1 & data2;
                self.registers.a = result;
            }
            "A,L" => {
                (data1, data2) = (self.registers.a, self.registers.l);
                let result = data1 & data2;
                self.registers.a = result;
            }
            "A,(HL)" => {
                (data1, data2) = (self.registers.a, self.mem_read(self.registers.hl()));
                let result = data1 & data2;
                self.registers.a = result;
            }
            "A,A" => {
                (data1, data2) = (self.registers.a, self.registers.a);
                let result = data1 & data2;
                self.registers.a = result;
            }
            "A,u8" => {
                (data1, data2) = (self.registers.a, self.fetch_byte());
                let result = data1 & data2;
                self.registers.a = result;
            }
            op => panic!("Operands not valid: {op}"),
        }

        self.registers.set_flag(CpuFlag::ZERO, data1 & data2 == 0);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(CpuFlag::HALF_CARRY, true);
        self.registers.set_flag(CpuFlag::CARRY, false);
        opcode.tcycles.0
    }

    fn xor(&mut self, opcode: OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        match operands {
            "A,B" => {
                (data1, data2) = (self.registers.a, self.registers.b);
                let result = data1 ^ data2;
                self.registers.a = result;
            }
            "A,C" => {
                (data1, data2) = (self.registers.a, self.registers.c);
                let result = data1 ^ data2;
                self.registers.a = result;
            }
            "A,D" => {
                (data1, data2) = (self.registers.a, self.registers.d);
                let result = data1 ^ data2;
                self.registers.a = result;
            }
            "A,E" => {
                (data1, data2) = (self.registers.a, self.registers.e);
                let result = data1 ^ data2;
                self.registers.a = result;
            }
            "A,H" => {
                (data1, data2) = (self.registers.a, self.registers.h);
                let result = data1 ^ data2;
                self.registers.a = result;
            }
            "A,L" => {
                (data1, data2) = (self.registers.a, self.registers.l);
                let result = data1 ^ data2;
                self.registers.a = result;
            }
            "A,(HL)" => {
                (data1, data2) = (self.registers.a, self.mem_read(self.registers.hl()));
                let result = data1 ^ data2;
                self.registers.a = result;
            }
            "A,A" => {
                (data1, data2) = (self.registers.a, self.registers.a);
                let result = data1 ^ data2;
                self.registers.a = result;
            }
            "A,u8" => {
                (data1, data2) = (self.registers.a, self.fetch_byte());
                let result = data1 ^ data2;
                self.registers.a = result;
            }
            op => panic!("Operands not valid: {op}"),
        }

        self.registers.set_flag(CpuFlag::ZERO, data1 ^ data2 == 0);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(CpuFlag::HALF_CARRY, false);
        self.registers.set_flag(CpuFlag::CARRY, false);
        opcode.tcycles.0
    }

    fn or(&mut self, opcode: OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        match operands {
            "A,B" => {
                (data1, data2) = (self.registers.a, self.registers.b);
                let result = data1 | data2;
                self.registers.a = result;
            }
            "A,C" => {
                (data1, data2) = (self.registers.a, self.registers.c);
                let result = data1 | data2;
                self.registers.a = result;
            }
            "A,D" => {
                (data1, data2) = (self.registers.a, self.registers.d);
                let result = data1 | data2;
                self.registers.a = result;
            }
            "A,E" => {
                (data1, data2) = (self.registers.a, self.registers.e);
                let result = data1 | data2;
                self.registers.a = result;
            }
            "A,H" => {
                (data1, data2) = (self.registers.a, self.registers.h);
                let result = data1 | data2;
                self.registers.a = result;
            }
            "A,L" => {
                (data1, data2) = (self.registers.a, self.registers.l);
                let result = data1 | data2;
                self.registers.a = result;
            }
            "A,(HL)" => {
                (data1, data2) = (self.registers.a, self.mem_read(self.registers.hl()));
                let result = data1 | data2;
                self.registers.a = result;
            }
            "A,A" => {
                (data1, data2) = (self.registers.a, self.registers.a);
                let result = data1 | data2;
                self.registers.a = result;
            }
            "A,u8" => {
                (data1, data2) = (self.registers.a, self.fetch_byte());
                let result = data1 | data2;
                self.registers.a = result;
            }
            op => panic!("Operands not valid: {op}"),
        }

        self.registers.set_flag(CpuFlag::ZERO, data1 | data2 == 0);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(CpuFlag::HALF_CARRY, false);
        self.registers.set_flag(CpuFlag::CARRY, false);
        opcode.tcycles.0
    }

    fn cp(&mut self, opcode: OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        match operands {
            "A,B" => (data1, data2) = (self.registers.a, self.registers.b),
            "A,C" => (data1, data2) = (self.registers.a, self.registers.c),
            "A,D" => (data1, data2) = (self.registers.a, self.registers.d),
            "A,E" => (data1, data2) = (self.registers.a, self.registers.e),
            "A,H" => (data1, data2) = (self.registers.a, self.registers.h),
            "A,L" => (data1, data2) = (self.registers.a, self.registers.l),
            "A,(HL)" => (data1, data2) = (self.registers.a, self.mem_read(self.registers.hl())),
            "A,A" => (data1, data2) = (self.registers.a, self.registers.a),
            "A,u8" => (data1, data2) = (self.registers.a, self.fetch_byte()),
            op => panic!("Operands not valid: {op}"),
        }

        self.registers.set_flag(CpuFlag::ZERO, data1 - data2 == 0);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers
            .set_flag(CpuFlag::HALF_CARRY, (data1 & 0x0F) < (data2 & 0x0F));
        self.registers
            .set_flag(CpuFlag::CARRY, (data1 as u16) < (data2 as u16));
        opcode.tcycles.0
    }

    fn daa(&mut self, opcode: OpCode) -> u8 {
        let mut a = self.registers.a;
        let mut correction = if self.registers.f.contains(CpuFlag::CARRY) {
            0x60
        } else {
            0x00
        };

        if self.registers.f.contains(CpuFlag::HALF_CARRY) {
            correction |= 0x06;
        }

        if !self.registers.f.contains(CpuFlag::SUBRACTION) {
            if a & 0x0F > 0x09 {
                correction |= 0x06;
            };
            if a > 0x99 {
                correction |= 0x60;
            }
        }
        a = a.wrapping_add(correction);
        self.registers.set_flag(CpuFlag::ZERO, a == 0);
        self.registers.set_flag(CpuFlag::HALF_CARRY, false);
        self.registers.set_flag(CpuFlag::CARRY, correction >= 0x60);
        self.registers.a = a;
        opcode.tcycles.0
    }

    fn cpl(&mut self, opcode: OpCode) -> u8 {
        self.registers.a = !self.registers.a;

        self.registers.set_flag(CpuFlag::SUBRACTION, true);
        self.registers.set_flag(CpuFlag::HALF_CARRY, true);
        opcode.tcycles.0
    }

    fn scf(&mut self, opcode: OpCode) -> u8 {
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(CpuFlag::HALF_CARRY, false);
        self.registers.set_flag(CpuFlag::CARRY, true);
        opcode.tcycles.0
    }

    fn ccf(&mut self, opcode: OpCode) -> u8 {
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(CpuFlag::HALF_CARRY, false);

        let complement = !self.registers.f.contains(CpuFlag::CARRY);
        self.registers.set_flag(CpuFlag::CARRY, complement);

        opcode.tcycles.0
    }

    fn rlca(&mut self, opcode: OpCode) -> u8 {
        self.registers.set_flag(CpuFlag::ZERO, false);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(CpuFlag::HALF_CARRY, false);
        self.registers
            .set_flag(CpuFlag::CARRY, self.registers.a & 0x80 == 0x80);

        let last_bit = if self.registers.f.contains(CpuFlag::CARRY) {
            0x01
        } else {
            0
        };

        self.registers.a = self.registers.a << 1 | last_bit;

        opcode.tcycles.0
    }

    fn rla(&mut self, opcode: OpCode) -> u8 {
        self.registers.set_flag(CpuFlag::ZERO, false);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(CpuFlag::HALF_CARRY, false);

        let last_bit = if self.registers.f.contains(CpuFlag::CARRY) {
            0x01
        } else {
            0
        };

        self.registers
            .set_flag(CpuFlag::CARRY, self.registers.a & 0x80 == 0x80);
        self.registers.a = self.registers.a << 1 | last_bit;

        opcode.tcycles.0
    }

    fn rrca(&mut self, opcode: OpCode) -> u8 {
        self.registers.set_flag(CpuFlag::ZERO, false);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(CpuFlag::HALF_CARRY, false);
        self.registers
            .set_flag(CpuFlag::CARRY, self.registers.a & 0x01 == 0x01);

        let first_bit = if self.registers.f.contains(CpuFlag::CARRY) {
            0x80
        } else {
            0
        };

        self.registers.a = first_bit | self.registers.a >> 1;

        opcode.tcycles.0
    }

    fn rra(&mut self, opcode: OpCode) -> u8 {
        self.registers.set_flag(CpuFlag::ZERO, false);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(CpuFlag::HALF_CARRY, false);

        let first_bit = if self.registers.f.contains(CpuFlag::CARRY) {
            0x80
        } else {
            0
        };

        self.registers
            .set_flag(CpuFlag::CARRY, self.registers.a & 0x01 == 0x01);
        self.registers.a = first_bit | self.registers.a >> 1;

        opcode.tcycles.0
    }

    fn jr(&mut self, opcode: OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let jump = ((self.registers.pc as i32) + (self.fetch_byte() as i32)) as u16;
        match operands {
            "i8" => {
                self.registers.pc = jump;
                return opcode.tcycles.0;
            }
            "NZ,i8" => {
                if !self.registers.f.contains(CpuFlag::ZERO) {
                    self.registers.pc = jump;
                    return opcode.tcycles.1;
                }
            }
            "Z,i8" => {
                if self.registers.f.contains(CpuFlag::ZERO) {
                    self.registers.pc = jump;
                    return opcode.tcycles.1;
                }
            }
            "NC,i8" => {
                if !self.registers.f.contains(CpuFlag::CARRY) {
                    self.registers.pc = jump;
                    return opcode.tcycles.1;
                }
            }
            "C,i8" => {
                if self.registers.f.contains(CpuFlag::CARRY) {
                    self.registers.pc = jump;
                    return opcode.tcycles.1;
                }
            }
            op => panic!("Operands not valid: {op}"),
        }

        self.registers.pc += 1;
        opcode.tcycles.0
    }

    fn ret(&mut self, opcode: OpCode) -> u8 {
        let mut operands = match self.get_operands(opcode.mnemonic) {
            operand if operand == "" && opcode.mnemonic == "RET" => "RET",
            operand if operand == "" && opcode.mnemonic == "RETI" => "RETI",
            operand => operand,
        };

        match operands {
            "NZ" => {
                if !self.registers.f.contains(CpuFlag::ZERO) {
                    self.registers.pc = self.pop_stack();
                    return opcode.tcycles.1;
                }
            }
            "Z" => {
                if self.registers.f.contains(CpuFlag::ZERO) {
                    self.registers.pc = self.pop_stack();
                    return opcode.tcycles.0;
                }
            }
            "RET" => self.registers.pc = self.pop_stack(),
            "NC" => {
                if !self.registers.f.contains(CpuFlag::CARRY) {
                    self.registers.pc = self.pop_stack();
                    return opcode.tcycles.1;
                }
            }
            "C" => {
                if self.registers.f.contains(CpuFlag::CARRY) {
                    self.registers.pc = self.pop_stack();
                    return opcode.tcycles.0;
                }
            }
            "RETI" => {
                self.registers.pc = self.pop_stack();
                self.ei = ImeState::Staged;
            }
            op => panic!("Operands not valid: {op}"),
        }

        opcode.tcycles.0
    }

    fn stop(&mut self, opcode: OpCode) -> u8 {
        todo!("finish this")
    }

    fn halt(&mut self, opcode: OpCode) -> u8 {
        self.halted = true;
        opcode.tcycles.0
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::opcodes;
    use utils::Mode;

    fn get_cpu() -> Cpu {
        let registers = Registers::new(Mode::Monochrome);
        let bus = Bus::new();
        let cpu = Cpu::new(registers, bus);
        cpu
    }

    #[test]
    fn execute_ld8() {
        let mut cpu = get_cpu();
    }

    #[test]
    fn execute_nop() {
        let mut cpu = get_cpu();
        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 4)
    }

    #[test]
    fn execute_ld_bc_with_u16() {
        let mut cpu = get_cpu();
        cpu.mem_write_16(cpu.registers.pc, 0x4423);

        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 12);
        assert_eq!(cpu.registers.bc(), 0x4423);
    }

    #[test]
    fn execute_ld_value_at_bc_with_a() {
        let mut cpu = get_cpu();
        cpu.registers.a = 0x44;

        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.mem_read(cpu.registers.bc()), 0x44);
    }

    #[test]
    fn execute_inc_bc() {
        let mut cpu = get_cpu();
        cpu.registers.set_bc(0x4544);

        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.bc(), 0x4545);
    }

    #[test]
    fn execute_inc_b() {
        let mut cpu = get_cpu();

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.b = 0x45;
        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.b, 0x46);
        assert_eq!(cpu.registers.f.bits(), 0b0000_0000);

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.b = 0b0001_1111;
        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.b, 0x20);
        assert_eq!(cpu.registers.f.bits(), 0b0010_0000);

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.b = 0xFF;
        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.b, 0);
        assert_eq!(cpu.registers.f.bits(), 0b1010_0000);
    }

    #[test]
    fn execute_dec_b() {
        let mut cpu = get_cpu();

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.b = 0x31;
        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.b, 0x30);
        assert_eq!(cpu.registers.f.bits(), 0b0100_0000);

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.b = 0x01;
        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.b, 0);
        assert_eq!(cpu.registers.f.bits(), 0b1100_0000);

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.b = 0;
        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.b, 0xFF);
        assert_eq!(cpu.registers.f.bits(), 0b0110_0000);
    }

    #[test]
    fn execute_ld_b_with_u8() {
        let mut cpu = get_cpu();
        cpu.mem_write_16(cpu.registers.pc, 0x4423);

        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.b, 0x23);
    }

    #[test]
    fn execute_rlca() {
        let mut cpu = get_cpu();

        cpu.registers.a = 0x44;
        let tcylcles = cpu.execute();
        assert_eq!(cpu.registers.a, 0x88);
        assert_eq!(cpu.registers.f.bits(), 0b0000_0000);
        assert_eq!(tcylcles, 4);

        cpu.registers.a = 0x88;
        let tcylcles = cpu.execute();
        assert_eq!(cpu.registers.a, 0x11);
        assert_eq!(cpu.registers.f.bits(), 0b0001_0000);
        assert_eq!(tcylcles, 4);
    }

    #[test]
    fn execute_ld_u16_with_sp() {
        let mut cpu = get_cpu();
        cpu.mem_write_16(cpu.registers.pc, 0x4423);
        cpu.registers.sp = 0x5555;

        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 20);
        assert_eq!(cpu.mem_read_16(0x4423), 0x5555);
    }

    #[test]
    fn execute_add_hl_with_bc() {
        let mut cpu = get_cpu();

        cpu.registers.set_hl(0x00FF);
        cpu.registers.set_bc(0x7C00);
        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.hl(), 0x7CFF);
        assert_eq!(cpu.registers.f.bits(), 0b0000_0000);

        cpu.registers.set_hl(0x07FF);
        cpu.registers.set_bc(0x7C00);
        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.hl(), 0x83FF);
        assert_eq!(cpu.registers.f.bits(), 0b0010_0000);

        cpu.registers.set_hl(0x00FF);
        cpu.registers.set_bc(0xFF01);
        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.hl(), 0x0000);
        assert_eq!(cpu.registers.f.bits(), 0b0011_0000);
    }

    #[test]
    fn execute_ld_a_with_value_at_bc() {
        let mut cpu = get_cpu();
        cpu.mem_write(cpu.registers.bc(), 0x44);

        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.a, 0x44);
    }

    #[test]
    fn execute_dec_bc() {
        let mut cpu = get_cpu();
        cpu.registers.set_bc(0x4544);

        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.bc(), 0x4543);
    }

    #[test]
    fn execute_inc_c() {
        let mut cpu = get_cpu();

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.c = 0x45;
        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.c, 0x46);
        assert_eq!(cpu.registers.f.bits(), 0b0000_0000);

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.c = 0b0001_1111;
        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.c, 0x20);
        assert_eq!(cpu.registers.f.bits(), 0b0010_0000);

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.c = 0xFF;
        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.c, 0);
        assert_eq!(cpu.registers.f.bits(), 0b1010_0000);
    }

    #[test]
    fn execute_dec_c() {
        let mut cpu = get_cpu();

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.c = 0x31;
        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.c, 0x30);
        assert_eq!(cpu.registers.f.bits(), 0b0100_0000);

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.c = 0x01;
        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.c, 0);
        assert_eq!(cpu.registers.f.bits(), 0b1100_0000);

        cpu.registers.f = CpuFlag::from_bits_truncate(0);
        cpu.registers.c = 0;
        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 4);
        assert_eq!(cpu.registers.c, 0xFF);
        assert_eq!(cpu.registers.f.bits(), 0b0110_0000);
    }

    #[test]
    fn execute_ld_c_with_u8() {
        let mut cpu = get_cpu();
        cpu.mem_write_16(cpu.registers.pc, 0x4423);

        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.c, 0x23);
    }

    #[test]
    fn execute_rrca() {
        let mut cpu = get_cpu();

        cpu.registers.a = 0x44;
        let tcylcles = cpu.execute();
        assert_eq!(cpu.registers.a, 0x22);
        assert_eq!(cpu.registers.f.bits(), 0b0000_0000);
        assert_eq!(tcylcles, 4);

        cpu.registers.a = 0x89;
        let tcylcles = cpu.execute();
        assert_eq!(cpu.registers.a, 0xC4);
        assert_eq!(cpu.registers.f.bits(), 0b0001_0000);
        assert_eq!(tcylcles, 4);
    }

    #[test]
    fn execute_ld_de_with_u16() {
        let mut cpu = get_cpu();
        cpu.mem_write_16(cpu.registers.pc, 0x4423);

        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 12);
        assert_eq!(cpu.registers.de(), 0x4423);
    }

    #[test]
    fn execute_ld_hl_with_u16() {
        let mut cpu = get_cpu();
        cpu.mem_write_16(cpu.registers.pc, 0x4423);

        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 12);
        assert_eq!(cpu.registers.hl(), 0x4423);
    }

    #[test]
    fn execute_ld_sp_with_u16() {
        let mut cpu = get_cpu();
        cpu.mem_write_16(cpu.registers.pc, 0x4423);

        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 12);
        assert_eq!(cpu.registers.sp, 0x4423);
    }

    #[test]
    fn execute_ld_sp_with_hl() {
        let mut cpu = get_cpu();
        cpu.registers.set_hl(0x4423);

        let tcylcles = cpu.execute();
        assert_eq!(tcylcles, 8);
        assert_eq!(cpu.registers.sp, 0x4423);
    }
}
