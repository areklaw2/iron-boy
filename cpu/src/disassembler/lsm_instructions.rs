use crate::{
    bus::{Bus, Memory},
    opcodes::OpCode,
    registers::Registers,
};

use super::Instruction;

pub struct LsmInstructions<'a> {
    registers: &'a mut Registers,
    bus: &'a mut Bus,
}

impl<'a> Instruction for LsmInstructions<'a> {}

impl<'a> LsmInstructions<'a> {
    pub fn new(registers: &mut Registers, bus: &mut Bus) -> Self {
        LsmInstructions { registers, bus }
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

    pub fn ld_16(&self, opcode: &&OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        match operands {
            "BC,u16" => {
                let value = self.fetch_word();
                self.registers.set_bc(value)
            }
            "(u16),SP" => {
                let address = self.fetch_word();
                self.bus.mem_write_16(address, self.registers.sp);
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

    pub fn ld_8(&self, opcode: &OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        match operands {
            "(BC),A" => self.bus.mem_write(self.registers.bc(), self.registers.a),
            "B,u8" => self.registers.b = self.fetch_byte(),
            "A,(BC)" => self.registers.a = self.bus.mem_read(self.registers.bc()),
            "C,u8" => self.registers.c = self.fetch_byte(),
            "(DE),A" => self.bus.mem_write(self.registers.de(), self.registers.a),
            "D,u8" => self.registers.d = self.fetch_byte(),
            "A,(DE)" => self.registers.a = self.bus.mem_read(self.registers.de()),
            "E,u8" => self.registers.e = self.fetch_byte(),
            "(HL+),A" => {
                let address = self.registers.increment_hl();
                self.bus.mem_write(address, self.registers.a);
            }
            "H,u8" => self.registers.h = self.fetch_byte(),
            "A,(HL+)" => {
                let address = self.registers.increment_hl();
                self.registers.a = self.bus.mem_read(address);
            }
            "L,u8" => self.registers.l = self.fetch_byte(),
            "(HL-),A" => {
                let address = self.registers.decrement_hl();
                self.bus.mem_write(address, self.registers.a);
            }
            "(HL),u8" => {
                let value = self.fetch_byte();
                self.bus.mem_write(self.registers.hl(), value);
            }
            "A,(HL-)" => {
                let address = self.registers.decrement_hl();
                self.registers.a = self.bus.mem_read(address);
            }
            "A,u8" => self.registers.a = self.fetch_byte(),
            "B,B" => {}
            "B,C" => self.registers.b = self.registers.c,
            "B,D" => self.registers.b = self.registers.d,
            "B,E" => self.registers.b = self.registers.e,
            "B,H" => self.registers.b = self.registers.h,
            "B,L" => self.registers.b = self.registers.l,
            "B,(HL)" => self.registers.b = self.bus.mem_read(self.registers.hl()),
            "B,A" => self.registers.b = self.registers.a,
            "C,B" => self.registers.c = self.registers.b,
            "C,C" => {}
            "C,D" => self.registers.c = self.registers.d,
            "C,E" => self.registers.c = self.registers.e,
            "C,H" => self.registers.c = self.registers.h,
            "C,L" => self.registers.c = self.registers.l,
            "C,(HL)" => self.registers.c = self.bus.mem_read(self.registers.hl()),
            "C,A" => self.registers.c = self.registers.a,
            "D,B" => self.registers.d = self.registers.b,
            "D,C" => self.registers.d = self.registers.c,
            "D,D" => {}
            "D,E" => self.registers.d = self.registers.e,
            "D,H" => self.registers.d = self.registers.h,
            "D,L" => self.registers.d = self.registers.l,
            "D,(HL)" => self.registers.d = self.bus.mem_read(self.registers.hl()),
            "D,A" => self.registers.d = self.registers.a,
            "E,B" => self.registers.e = self.registers.b,
            "E,C" => self.registers.e = self.registers.c,
            "E,D" => self.registers.e = self.registers.d,
            "E,E" => {}
            "E,H" => self.registers.e = self.registers.h,
            "E,L" => self.registers.e = self.registers.l,
            "E,(HL)" => self.registers.e = self.bus.mem_read(self.registers.hl()),
            "E,A" => self.registers.e = self.registers.a,
            "H,B" => self.registers.h = self.registers.b,
            "H,C" => self.registers.h = self.registers.c,
            "H,D" => self.registers.h = self.registers.d,
            "H,E" => self.registers.h = self.registers.e,
            "H,H" => {}
            "H,L" => self.registers.h = self.registers.l,
            "H,(HL)" => self.registers.h = self.bus.mem_read(self.registers.hl()),
            "H,A" => self.registers.h = self.registers.a,
            "L,B" => self.registers.l = self.registers.b,
            "L,C" => self.registers.l = self.registers.c,
            "L,D" => self.registers.l = self.registers.d,
            "L,E" => self.registers.l = self.registers.e,
            "L,H" => self.registers.l = self.registers.h,
            "L,L" => {}
            "L,(HL)" => self.registers.l = self.bus.mem_read(self.registers.hl()),
            "L,A" => self.registers.l = self.registers.a,
            "(HL),B" => self.bus.mem_write(self.registers.hl(), self.registers.b),
            "(HL),C" => self.bus.mem_write(self.registers.hl(), self.registers.c),
            "(HL),D" => self.bus.mem_write(self.registers.hl(), self.registers.d),
            "(HL),E" => self.bus.mem_write(self.registers.hl(), self.registers.e),
            "(HL),H" => self.bus.mem_write(self.registers.hl(), self.registers.h),
            "(HL),L" => self.bus.mem_write(self.registers.hl(), self.registers.l),
            "(HL),A" => self.bus.mem_write(self.registers.hl(), self.registers.a),
            "A,B" => self.registers.a = self.registers.b,
            "A,C" => self.registers.a = self.registers.c,
            "A,D" => self.registers.a = self.registers.d,
            "A,E" => self.registers.a = self.registers.e,
            "A,H" => self.registers.a = self.registers.h,
            "A,L" => self.registers.a = self.registers.l,
            "A,(HL)" => self.registers.a = self.bus.mem_read(self.registers.hl()),
            "A,A" => {}
            "(FF00 + u8),A" => {
                let address = 0xFF00 | self.fetch_byte() as u16;
                self.bus.mem_write(address, self.registers.a);
            }
            "(FF00 + C),A" => {
                let address = 0xFF00 | self.registers.c as u16;
                self.bus.mem_write(address, self.registers.a);
            }
            "(u16),A" => {
                let address = self.fetch_word();
                self.bus.mem_write(address, self.registers.a)
            }
            "A,(FF00 + u8)" => {
                let address = 0xFF00 | self.fetch_byte() as u16;
                self.registers.a = self.bus.mem_read(address)
            }
            "A,(FF00 + C)" => {
                let address = 0xFF00 | self.registers.c as u16;
                self.registers.a = self.bus.mem_read(address)
            }
            "A,(u16)" => {
                let address = self.fetch_word();
                self.registers.a = self.bus.mem_read(address)
            }
            op => panic!("Operands not valid: {op}"),
        }
        opcode.tcycles.0
    }

    fn pop_stack(&mut self) -> u16 {
        let data = self.bus.mem_read_16(self.registers.sp);
        self.registers.sp = self.registers.sp.wrapping_add(2);
        data
    }

    fn push_stack(&mut self, data: u16) {
        self.registers.sp = self.registers.sp.wrapping_sub(2);
        self.bus.mem_write_16(self.registers.sp, data);
    }

    pub fn pop(&mut self, opcode: &OpCode) -> u8 {
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

    pub fn push(&mut self, opcode: &OpCode) -> u8 {
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
}
