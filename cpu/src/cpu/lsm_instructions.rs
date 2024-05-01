use crate::{
    bus::{Bus, Memory},
    opcodes::OpCode,
    registers::Registers,
};

use super::Instructions;

pub struct LsmInstructions {}

impl Instructions for LsmInstructions {}

impl LsmInstructions {
    pub fn new() -> Self {
        LsmInstructions {}
    }

    pub fn ld_16(&self, opcode: &OpCode, registers: &mut Registers, bus: &mut Bus) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        match operands {
            "BC,u16" => {
                let value = Self::fetch_word(registers, bus);
                registers.set_bc(value)
            }
            "(u16),SP" => {
                let address = Self::fetch_word(registers, bus);
                bus.mem_write_16(address, registers.sp);
            }
            "DE,u16" => {
                let value = Self::fetch_word(registers, bus);
                registers.set_de(value)
            }
            "HL,u16" => {
                let value = Self::fetch_word(registers, bus);
                registers.set_hl(value)
            }
            "SP,u16" => registers.sp = Self::fetch_word(registers, bus),
            "SP,HL" => registers.sp = registers.hl(),
            op => panic!("Operands not valid: {op}"),
        }
        opcode.tcycles.0
    }

    pub fn ld_8(&self, opcode: &OpCode, registers: &mut Registers, bus: &mut Bus) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        match operands {
            "(BC),A" => bus.mem_write(registers.bc(), registers.a),
            "B,u8" => registers.b = Self::fetch_byte(registers, bus),
            "A,(BC)" => registers.a = bus.mem_read(registers.bc()),
            "C,u8" => registers.c = Self::fetch_byte(registers, bus),
            "(DE),A" => bus.mem_write(registers.de(), registers.a),
            "D,u8" => registers.d = Self::fetch_byte(registers, bus),
            "A,(DE)" => registers.a = bus.mem_read(registers.de()),
            "E,u8" => registers.e = Self::fetch_byte(registers, bus),
            "(HL+),A" => {
                let address = registers.increment_hl();
                bus.mem_write(address, registers.a);
            }
            "H,u8" => registers.h = Self::fetch_byte(registers, bus),
            "A,(HL+)" => {
                let address = registers.increment_hl();
                registers.a = bus.mem_read(address);
            }
            "L,u8" => registers.l = Self::fetch_byte(registers, bus),
            "(HL-),A" => {
                let address = registers.decrement_hl();
                bus.mem_write(address, registers.a);
            }
            "(HL),u8" => {
                let value = Self::fetch_byte(registers, bus);
                bus.mem_write(registers.hl(), value);
            }
            "A,(HL-)" => {
                let address = registers.decrement_hl();
                registers.a = bus.mem_read(address);
            }
            "A,u8" => registers.a = Self::fetch_byte(registers, bus),
            "B,B" => {}
            "B,C" => registers.b = registers.c,
            "B,D" => registers.b = registers.d,
            "B,E" => registers.b = registers.e,
            "B,H" => registers.b = registers.h,
            "B,L" => registers.b = registers.l,
            "B,(HL)" => registers.b = bus.mem_read(registers.hl()),
            "B,A" => registers.b = registers.a,
            "C,B" => registers.c = registers.b,
            "C,C" => {}
            "C,D" => registers.c = registers.d,
            "C,E" => registers.c = registers.e,
            "C,H" => registers.c = registers.h,
            "C,L" => registers.c = registers.l,
            "C,(HL)" => registers.c = bus.mem_read(registers.hl()),
            "C,A" => registers.c = registers.a,
            "D,B" => registers.d = registers.b,
            "D,C" => registers.d = registers.c,
            "D,D" => {}
            "D,E" => registers.d = registers.e,
            "D,H" => registers.d = registers.h,
            "D,L" => registers.d = registers.l,
            "D,(HL)" => registers.d = bus.mem_read(registers.hl()),
            "D,A" => registers.d = registers.a,
            "E,B" => registers.e = registers.b,
            "E,C" => registers.e = registers.c,
            "E,D" => registers.e = registers.d,
            "E,E" => {}
            "E,H" => registers.e = registers.h,
            "E,L" => registers.e = registers.l,
            "E,(HL)" => registers.e = bus.mem_read(registers.hl()),
            "E,A" => registers.e = registers.a,
            "H,B" => registers.h = registers.b,
            "H,C" => registers.h = registers.c,
            "H,D" => registers.h = registers.d,
            "H,E" => registers.h = registers.e,
            "H,H" => {}
            "H,L" => registers.h = registers.l,
            "H,(HL)" => registers.h = bus.mem_read(registers.hl()),
            "H,A" => registers.h = registers.a,
            "L,B" => registers.l = registers.b,
            "L,C" => registers.l = registers.c,
            "L,D" => registers.l = registers.d,
            "L,E" => registers.l = registers.e,
            "L,H" => registers.l = registers.h,
            "L,L" => {}
            "L,(HL)" => registers.l = bus.mem_read(registers.hl()),
            "L,A" => registers.l = registers.a,
            "(HL),B" => bus.mem_write(registers.hl(), registers.b),
            "(HL),C" => bus.mem_write(registers.hl(), registers.c),
            "(HL),D" => bus.mem_write(registers.hl(), registers.d),
            "(HL),E" => bus.mem_write(registers.hl(), registers.e),
            "(HL),H" => bus.mem_write(registers.hl(), registers.h),
            "(HL),L" => bus.mem_write(registers.hl(), registers.l),
            "(HL),A" => bus.mem_write(registers.hl(), registers.a),
            "A,B" => registers.a = registers.b,
            "A,C" => registers.a = registers.c,
            "A,D" => registers.a = registers.d,
            "A,E" => registers.a = registers.e,
            "A,H" => registers.a = registers.h,
            "A,L" => registers.a = registers.l,
            "A,(HL)" => registers.a = bus.mem_read(registers.hl()),
            "A,A" => {}
            "(FF00 + u8),A" => {
                let address = 0xFF00 | Self::fetch_byte(registers, bus) as u16;
                bus.mem_write(address, registers.a);
            }
            "(FF00 + C),A" => {
                let address = 0xFF00 | registers.c as u16;
                bus.mem_write(address, registers.a);
            }
            "(u16),A" => {
                let address = Self::fetch_word(registers, bus);
                bus.mem_write(address, registers.a)
            }
            "A,(FF00 + u8)" => {
                let address = 0xFF00 | Self::fetch_byte(registers, bus) as u16;
                registers.a = bus.mem_read(address)
            }
            "A,(FF00 + C)" => {
                let address = 0xFF00 | registers.c as u16;
                registers.a = bus.mem_read(address)
            }
            "A,(u16)" => {
                let address = Self::fetch_word(registers, bus);
                registers.a = bus.mem_read(address)
            }
            op => panic!("Operands not valid: {op}"),
        }
        opcode.tcycles.0
    }

    pub fn pop(&mut self, opcode: &OpCode, registers: &mut Registers, bus: &mut Bus) -> u8 {
        let operand = self.get_operands(opcode.mnemonic);
        match operand {
            "BC" => {
                let value = Self::pop_stack(registers, bus);
                registers.set_bc(value);
            }
            "DE" => {
                let value = Self::pop_stack(registers, bus);
                registers.set_de(value);
            }
            "HL" => {
                let value = Self::pop_stack(registers, bus);
                registers.set_hl(value);
            }
            "AF" => {
                let value = Self::pop_stack(registers, bus) & 0xFFF0;
                registers.set_af(value);
            }
            op => panic!("Operand not valid: {op}"),
        }
        opcode.tcycles.0
    }

    pub fn push(&mut self, opcode: &OpCode, registers: &mut Registers, bus: &mut Bus) -> u8 {
        let operand = self.get_operands(opcode.mnemonic);
        match operand {
            "BC" => Self::push_stack(registers.bc(), registers, bus),
            "DE" => Self::push_stack(registers.de(), registers, bus),
            "HL" => Self::push_stack(registers.hl(), registers, bus),
            "AF" => Self::push_stack(registers.af(), registers, bus),
            op => panic!("Operand not valid: {op}"),
        }
        opcode.tcycles.0
    }
}
