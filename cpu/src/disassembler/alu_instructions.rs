use crate::{
    bus::{Bus, Memory},
    opcodes::OpCode,
    registers::{CpuFlag, Registers},
};

use super::Instruction;

pub struct AluInstructions<'a> {
    registers: &'a mut Registers,
    bus: &'a mut Bus,
}

impl<'a> Instruction for AluInstructions<'a> {}

impl<'a> AluInstructions<'a> {
    pub fn new(registers: &mut Registers, bus: &mut Bus) -> Self {
        AluInstructions { registers, bus }
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

    pub fn inc_16(&mut self, opcode: &OpCode) -> u8 {
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

    pub fn inc_8(&mut self, opcode: &OpCode) -> u8 {
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
                data = self.bus.mem_read(self.registers.hl());
                self.bus.mem_write(self.registers.hl(), data.wrapping_add(1));
                result = self.bus.mem_read(self.registers.hl());
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
        self.registers.set_flag(CpuFlag::HALF_CARRY, (data & 0x0F) + 1 > 0x0F);

        opcode.tcycles.0
    }

    pub fn dec_16(&mut self, opcode: &OpCode) -> u8 {
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

    pub fn dec_8(&mut self, opcode: &OpCode) -> u8 {
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
                data = self.bus.mem_read(self.registers.hl());
                self.bus.mem_write(self.registers.hl(), data.wrapping_sub(1));
                result = self.bus.mem_read(self.registers.hl());
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
        self.registers.set_flag(CpuFlag::HALF_CARRY, (data & 0x0F) == 0);

        opcode.tcycles.0
    }

    pub fn add_16(&mut self, opcode: &OpCode) -> u8 {
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
        self.registers.set_flag(CpuFlag::HALF_CARRY, (data1 & 0x07FF) + (data2 & 0x07FF) > 0x07FF);
        self.registers.set_flag(CpuFlag::CARRY, data1 as u32 + data2 as u32 > 0xFFFF);
        opcode.tcycles.0
    }

    pub fn add_8(&mut self, opcode: &OpCode) -> u8 {
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
                (data1, data2) = (self.registers.a, self.bus.mem_read(self.registers.hl()));
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
        self.registers.set_flag(CpuFlag::HALF_CARRY, (data1 & 0x0F) + (data2 & 0x0F) > 0x0F);
        self.registers.set_flag(CpuFlag::CARRY, data1 as u16 + data2 as u16 > 0xFF);
        opcode.tcycles.0
    }

    pub fn adc(&mut self, opcode: &OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        let carry = if self.registers.f.contains(CpuFlag::CARRY) { 1 } else { 0 };
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
                (data1, data2) = (self.registers.a, self.bus.mem_read(self.registers.hl()));
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

        self.registers.set_flag(CpuFlag::ZERO, data1 + data2 + carry == 0);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers
            .set_flag(CpuFlag::HALF_CARRY, (data1 & 0x0F) + (data2 & 0x0F) + carry > 0x0F);
        self.registers.set_flag(CpuFlag::CARRY, data1 as u16 + data2 as u16 + carry as u16 > 0xFF);
        opcode.tcycles.0
    }

    pub fn sub(&mut self, opcode: &OpCode) -> u8 {
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
                (data1, data2) = (self.registers.a, self.bus.mem_read(self.registers.hl()));
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
        self.registers.set_flag(CpuFlag::HALF_CARRY, (data1 & 0x0F) < (data2 & 0x0F));
        self.registers.set_flag(CpuFlag::CARRY, (data1 as u16) < (data2 as u16));
        opcode.tcycles.0
    }

    pub fn sbc(&mut self, opcode: &OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        let carry = if self.registers.f.contains(CpuFlag::CARRY) { 1 } else { 0 };
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
                (data1, data2) = (self.registers.a, self.bus.mem_read(self.registers.hl()));
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
        self.registers.set_flag(CpuFlag::HALF_CARRY, (data1 & 0x0F) < (data2 & 0x0F) + carry);
        self.registers.set_flag(CpuFlag::CARRY, (data1 as u16) < (data2 as u16) + (carry as u16));
        opcode.tcycles.0
    }

    pub fn and(&mut self, opcode: &OpCode) -> u8 {
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
                (data1, data2) = (self.registers.a, self.bus.mem_read(self.registers.hl()));
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

    pub fn xor(&mut self, opcode: &OpCode) -> u8 {
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
                (data1, data2) = (self.registers.a, self.bus.mem_read(self.registers.hl()));
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

    pub fn or(&mut self, opcode: &OpCode) -> u8 {
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
                (data1, data2) = (self.registers.a, self.bus.mem_read(self.registers.hl()));
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

    pub fn cp(&mut self, opcode: &OpCode) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        match operands {
            "A,B" => (data1, data2) = (self.registers.a, self.registers.b),
            "A,C" => (data1, data2) = (self.registers.a, self.registers.c),
            "A,D" => (data1, data2) = (self.registers.a, self.registers.d),
            "A,E" => (data1, data2) = (self.registers.a, self.registers.e),
            "A,H" => (data1, data2) = (self.registers.a, self.registers.h),
            "A,L" => (data1, data2) = (self.registers.a, self.registers.l),
            "A,(HL)" => (data1, data2) = (self.registers.a, self.bus.mem_read(self.registers.hl())),
            "A,A" => (data1, data2) = (self.registers.a, self.registers.a),
            "A,u8" => (data1, data2) = (self.registers.a, self.fetch_byte()),
            op => panic!("Operands not valid: {op}"),
        }

        self.registers.set_flag(CpuFlag::ZERO, data1 - data2 == 0);
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(CpuFlag::HALF_CARRY, (data1 & 0x0F) < (data2 & 0x0F));
        self.registers.set_flag(CpuFlag::CARRY, (data1 as u16) < (data2 as u16));
        opcode.tcycles.0
    }

    pub fn daa(&mut self, opcode: &OpCode) -> u8 {
        let mut a = self.registers.a;
        let mut correction = if self.registers.f.contains(CpuFlag::CARRY) { 0x60 } else { 0x00 };

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

    pub fn cpl(&mut self, opcode: &OpCode) -> u8 {
        self.registers.a = !self.registers.a;

        self.registers.set_flag(CpuFlag::SUBRACTION, true);
        self.registers.set_flag(CpuFlag::HALF_CARRY, true);
        opcode.tcycles.0
    }

    pub fn scf(&mut self, opcode: &OpCode) -> u8 {
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(CpuFlag::HALF_CARRY, false);
        self.registers.set_flag(CpuFlag::CARRY, true);
        opcode.tcycles.0
    }

    pub fn ccf(&mut self, opcode: &OpCode) -> u8 {
        self.registers.set_flag(CpuFlag::SUBRACTION, false);
        self.registers.set_flag(CpuFlag::HALF_CARRY, false);

        let complement = !self.registers.f.contains(CpuFlag::CARRY);
        self.registers.set_flag(CpuFlag::CARRY, complement);

        opcode.tcycles.0
    }
}
