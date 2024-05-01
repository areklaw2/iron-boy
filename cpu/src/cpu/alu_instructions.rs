use crate::{
    bus::{Bus, Memory},
    opcodes::OpCode,
    registers::{CpuFlag, Registers},
};

use super::Instructions;

pub struct AluInstructions {}

impl Instructions for AluInstructions {}

impl AluInstructions {
    pub fn new() -> Self {
        AluInstructions {}
    }

    pub fn inc_16(&mut self, opcode: &OpCode, registers: &mut Registers) -> u8 {
        let operand = self.get_operands(opcode.mnemonic);
        match operand {
            "BC" => registers.set_bc(registers.bc().wrapping_add(1)),
            "DE" => registers.set_de(registers.de().wrapping_add(1)),
            "HL" => registers.set_hl(registers.hl().wrapping_add(1)),
            "SP" => registers.sp = registers.sp.wrapping_add(1),
            op => panic!("Operands not valid: {op}"),
        };
        opcode.tcycles.0
    }

    pub fn inc_8(&mut self, opcode: &OpCode, registers: &mut Registers, bus: &mut Bus) -> u8 {
        let operand = self.get_operands(opcode.mnemonic);
        let data;
        let result;
        match operand {
            "B" => {
                data = registers.b;
                registers.b = data.wrapping_add(1);
                result = registers.b;
            }
            "C" => {
                data = registers.c;
                registers.c = data.wrapping_add(1);
                result = registers.c;
            }
            "D" => {
                data = registers.d;
                registers.d = data.wrapping_add(1);
                result = registers.d;
            }
            "E" => {
                data = registers.e;
                registers.e = data.wrapping_add(1);
                result = registers.e;
            }
            "H" => {
                data = registers.h;
                registers.h = data.wrapping_add(1);
                result = registers.h;
            }
            "L" => {
                data = registers.l;
                registers.l = data.wrapping_add(1);
                result = registers.l;
            }
            "(HL)" => {
                data = bus.mem_read(registers.hl());
                bus.mem_write(registers.hl(), data.wrapping_add(1));
                result = bus.mem_read(registers.hl());
            }
            "A" => {
                data = registers.a;
                registers.a = data.wrapping_add(1);
                result = registers.a;
            }
            op => panic!("Operands not valid: {op}"),
        };

        registers.set_flag(CpuFlag::ZERO, result == 0);
        registers.set_flag(CpuFlag::SUBRACTION, false);
        registers.set_flag(CpuFlag::HALF_CARRY, (data & 0x0F) + 1 > 0x0F);

        opcode.tcycles.0
    }

    pub fn dec_16(&mut self, opcode: &OpCode, registers: &mut Registers) -> u8 {
        let operand = self.get_operands(opcode.mnemonic);
        match operand {
            "BC" => registers.set_bc(registers.bc().wrapping_sub(1)),
            "DE" => registers.set_de(registers.de().wrapping_sub(1)),
            "HL" => registers.set_hl(registers.hl().wrapping_sub(1)),
            "SP" => registers.sp = registers.sp.wrapping_sub(1),
            op => panic!("Operands not valid: {op}"),
        };
        opcode.tcycles.0
    }

    pub fn dec_8(&mut self, opcode: &OpCode, registers: &mut Registers, bus: &mut Bus) -> u8 {
        let operand = self.get_operands(opcode.mnemonic);
        let data;
        let result;
        match operand {
            "B" => {
                data = registers.b;
                registers.b = data.wrapping_sub(1);
                result = registers.b;
            }
            "C" => {
                data = registers.c;
                registers.c = data.wrapping_sub(1);
                result = registers.c;
            }
            "D" => {
                data = registers.d;
                registers.d = data.wrapping_sub(1);
                result = registers.d;
            }
            "E" => {
                data = registers.e;
                registers.e = data.wrapping_sub(1);
                result = registers.e;
            }
            "H" => {
                data = registers.h;
                registers.h = data.wrapping_sub(1);
                result = registers.h;
            }
            "L" => {
                data = registers.l;
                registers.l = data.wrapping_sub(1);
                result = registers.l;
            }
            "(HL)" => {
                data = bus.mem_read(registers.hl());
                bus.mem_write(registers.hl(), data.wrapping_sub(1));
                result = bus.mem_read(registers.hl());
            }
            "A" => {
                data = registers.a;
                registers.a = data.wrapping_sub(1);
                result = registers.a;
            }
            op => panic!("Operands not valid: {op}"),
        };

        registers.set_flag(CpuFlag::ZERO, result == 0);
        registers.set_flag(CpuFlag::SUBRACTION, true);
        registers.set_flag(CpuFlag::HALF_CARRY, (data & 0x0F) == 0);

        opcode.tcycles.0
    }

    pub fn add_16(&mut self, opcode: &OpCode, registers: &mut Registers) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        match operands {
            "HL,BC" => {
                (data1, data2) = (registers.hl(), registers.bc());
                let result = data1.wrapping_add(data2);
                registers.set_hl(result);
            }
            "HL,DE" => {
                (data1, data2) = (registers.hl(), registers.de());
                let result = data1.wrapping_add(data2);
                registers.set_hl(result);
            }
            "HL,HL" => {
                (data1, data2) = (registers.hl(), registers.hl());
                let result = data1.wrapping_add(data2);
                registers.set_hl(result);
            }
            "HL,SP" => {
                (data1, data2) = (registers.hl(), registers.sp);
                let result = data1.wrapping_add(data2);
                registers.set_hl(result);
            }
            op => panic!("Operands not valid: {op}"),
        };

        registers.set_flag(CpuFlag::SUBRACTION, false);
        registers.set_flag(CpuFlag::HALF_CARRY, (data1 & 0x07FF) + (data2 & 0x07FF) > 0x07FF);
        registers.set_flag(CpuFlag::CARRY, data1 as u32 + data2 as u32 > 0xFFFF);
        opcode.tcycles.0
    }

    pub fn add_8(&mut self, opcode: &OpCode, registers: &mut Registers, bus: &mut Bus) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        match operands {
            "A,B" => {
                (data1, data2) = (registers.a, registers.b);
                let result = data1.wrapping_add(data2);
                registers.a = result;
            }
            "A,C" => {
                (data1, data2) = (registers.a, registers.c);
                let result = data1.wrapping_add(data2);
                registers.a = result;
            }
            "A,D" => {
                (data1, data2) = (registers.a, registers.d);
                let result = data1.wrapping_add(data2);
                registers.a = result;
            }
            "A,E" => {
                (data1, data2) = (registers.a, registers.e);
                let result = data1.wrapping_add(data2);
                registers.a = result;
            }
            "A,H" => {
                (data1, data2) = (registers.a, registers.h);
                let result = data1.wrapping_add(data2);
                registers.a = result;
            }
            "A,L" => {
                (data1, data2) = (registers.a, registers.l);
                let result = data1.wrapping_add(data2);
                registers.a = result;
            }
            "A,(HL)" => {
                (data1, data2) = (registers.a, bus.mem_read(registers.hl()));
                let result = data1.wrapping_add(data2);
                registers.a = result;
            }
            "A,A" => {
                (data1, data2) = (registers.a, registers.a);
                let result = data1.wrapping_add(data2);
                registers.a = result;
            }
            "A,u8" => {
                (data1, data2) = (registers.a, Self::fetch_byte(registers, bus));
                let result = data1.wrapping_add(data2);
                registers.a = result;
            }
            op => panic!("Operands not valid: {op}"),
        }

        registers.set_flag(CpuFlag::ZERO, data1 + data2 == 0);
        registers.set_flag(CpuFlag::SUBRACTION, false);
        registers.set_flag(CpuFlag::HALF_CARRY, (data1 & 0x0F) + (data2 & 0x0F) > 0x0F);
        registers.set_flag(CpuFlag::CARRY, data1 as u16 + data2 as u16 > 0xFF);
        opcode.tcycles.0
    }

    pub fn adc(&mut self, opcode: &OpCode, registers: &mut Registers, bus: &mut Bus) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        let carry = if registers.f.contains(CpuFlag::CARRY) { 1 } else { 0 };
        match operands {
            "A,B" => {
                (data1, data2) = (registers.a, registers.b);
                let result = data1.wrapping_add(data2).wrapping_add(carry);
                registers.a = result;
            }
            "A,C" => {
                (data1, data2) = (registers.a, registers.c);
                let result = data1.wrapping_add(data2).wrapping_add(carry);
                registers.a = result;
            }
            "A,D" => {
                (data1, data2) = (registers.a, registers.d);
                let result = data1.wrapping_add(data2).wrapping_add(carry);
                registers.a = result;
            }
            "A,E" => {
                (data1, data2) = (registers.a, registers.e);
                let result = data1.wrapping_add(data2).wrapping_add(carry);
                registers.a = result;
            }
            "A,H" => {
                (data1, data2) = (registers.a, registers.h);
                let result = data1.wrapping_add(data2).wrapping_add(carry);
                registers.a = result;
            }
            "A,L" => {
                (data1, data2) = (registers.a, registers.l);
                let result = data1.wrapping_add(data2).wrapping_add(carry);
                registers.a = result;
            }
            "A,(HL)" => {
                (data1, data2) = (registers.a, bus.mem_read(registers.hl()));
                let result = data1.wrapping_add(data2).wrapping_add(carry);
                registers.a = result;
            }
            "A,A" => {
                (data1, data2) = (registers.a, registers.a);
                let result = data1.wrapping_add(data2).wrapping_add(carry);
                registers.a = result;
            }
            "A,u8" => {
                (data1, data2) = (registers.a, Self::fetch_byte(registers, bus));
                let result = data1.wrapping_add(data2).wrapping_add(carry);
                registers.a = result;
            }
            op => panic!("Operands not valid: {op}"),
        }

        registers.set_flag(CpuFlag::ZERO, data1 + data2 + carry == 0);
        registers.set_flag(CpuFlag::SUBRACTION, false);
        registers.set_flag(CpuFlag::HALF_CARRY, (data1 & 0x0F) + (data2 & 0x0F) + carry > 0x0F);
        registers.set_flag(CpuFlag::CARRY, data1 as u16 + data2 as u16 + carry as u16 > 0xFF);
        opcode.tcycles.0
    }

    pub fn sub(&mut self, opcode: &OpCode, registers: &mut Registers, bus: &mut Bus) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        match operands {
            "A,B" => {
                (data1, data2) = (registers.a, registers.b);
                let result = data1.wrapping_sub(data2);
                registers.a = result;
            }
            "A,C" => {
                (data1, data2) = (registers.a, registers.c);
                let result = data1.wrapping_sub(data2);
                registers.a = result;
            }
            "A,D" => {
                (data1, data2) = (registers.a, registers.d);
                let result = data1.wrapping_sub(data2);
                registers.a = result;
            }
            "A,E" => {
                (data1, data2) = (registers.a, registers.e);
                let result = data1.wrapping_sub(data2);
                registers.a = result;
            }
            "A,H" => {
                (data1, data2) = (registers.a, registers.h);
                let result = data1.wrapping_sub(data2);
                registers.a = result;
            }
            "A,L" => {
                (data1, data2) = (registers.a, registers.l);
                let result = data1.wrapping_sub(data2);
                registers.a = result;
            }
            "A,(HL)" => {
                (data1, data2) = (registers.a, bus.mem_read(registers.hl()));
                let result = data1.wrapping_sub(data2);
                registers.a = result;
            }
            "A,A" => {
                (data1, data2) = (registers.a, registers.a);
                let result = data1.wrapping_sub(data2);
                registers.a = result;
            }
            "A,u8" => {
                (data1, data2) = (registers.a, Self::fetch_byte(registers, bus));
                let result = data1.wrapping_sub(data2);
                registers.a = result;
            }
            op => panic!("Operands not valid: {op}"),
        }

        registers.set_flag(CpuFlag::ZERO, data1 - data2 == 0);
        registers.set_flag(CpuFlag::SUBRACTION, false);
        registers.set_flag(CpuFlag::HALF_CARRY, (data1 & 0x0F) < (data2 & 0x0F));
        registers.set_flag(CpuFlag::CARRY, (data1 as u16) < (data2 as u16));
        opcode.tcycles.0
    }

    pub fn sbc(&mut self, opcode: &OpCode, registers: &mut Registers, bus: &mut Bus) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        let carry = if registers.f.contains(CpuFlag::CARRY) { 1 } else { 0 };
        match operands {
            "A,B" => {
                (data1, data2) = (registers.a, registers.b);
                let result = data1.wrapping_sub(data2).wrapping_sub(carry);
                registers.a = result;
            }
            "A,C" => {
                (data1, data2) = (registers.a, registers.c);
                let result = data1.wrapping_sub(data2).wrapping_sub(carry);
                registers.a = result;
            }
            "A,D" => {
                (data1, data2) = (registers.a, registers.d);
                let result = data1.wrapping_sub(data2).wrapping_sub(carry);
                registers.a = result;
            }
            "A,E" => {
                (data1, data2) = (registers.a, registers.e);
                let result = data1.wrapping_sub(data2).wrapping_sub(carry);
                registers.a = result;
            }
            "A,H" => {
                (data1, data2) = (registers.a, registers.h);
                let result = data1.wrapping_sub(data2).wrapping_sub(carry);
                registers.a = result;
            }
            "A,L" => {
                (data1, data2) = (registers.a, registers.l);
                let result = data1.wrapping_sub(data2).wrapping_sub(carry);
                registers.a = result;
            }
            "A,(HL)" => {
                (data1, data2) = (registers.a, bus.mem_read(registers.hl()));
                let result = data1.wrapping_sub(data2).wrapping_sub(carry);
                registers.a = result;
            }
            "A,A" => {
                (data1, data2) = (registers.a, registers.a);
                let result = data1.wrapping_sub(data2).wrapping_sub(carry);
                registers.a = result;
            }
            "A,u8" => {
                (data1, data2) = (registers.a, Self::fetch_byte(registers, bus));
                let result = data1.wrapping_sub(data2).wrapping_sub(carry);
                registers.a = result;
            }
            op => panic!("Operands not valid: {op}"),
        }

        registers.set_flag(CpuFlag::ZERO, data1 - data2 == 0);
        registers.set_flag(CpuFlag::SUBRACTION, false);
        registers.set_flag(CpuFlag::HALF_CARRY, (data1 & 0x0F) < (data2 & 0x0F) + carry);
        registers.set_flag(CpuFlag::CARRY, (data1 as u16) < (data2 as u16) + (carry as u16));
        opcode.tcycles.0
    }

    pub fn and(&mut self, opcode: &OpCode, registers: &mut Registers, bus: &mut Bus) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        match operands {
            "A,B" => {
                (data1, data2) = (registers.a, registers.b);
                let result = data1 & data2;
                registers.a = result;
            }
            "A,C" => {
                (data1, data2) = (registers.a, registers.c);
                let result = data1 & data2;
                registers.a = result;
            }
            "A,D" => {
                (data1, data2) = (registers.a, registers.d);
                let result = data1 & data2;
                registers.a = result;
            }
            "A,E" => {
                (data1, data2) = (registers.a, registers.e);
                let result = data1 & data2;
                registers.a = result;
            }
            "A,H" => {
                (data1, data2) = (registers.a, registers.h);
                let result = data1 & data2;
                registers.a = result;
            }
            "A,L" => {
                (data1, data2) = (registers.a, registers.l);
                let result = data1 & data2;
                registers.a = result;
            }
            "A,(HL)" => {
                (data1, data2) = (registers.a, bus.mem_read(registers.hl()));
                let result = data1 & data2;
                registers.a = result;
            }
            "A,A" => {
                (data1, data2) = (registers.a, registers.a);
                let result = data1 & data2;
                registers.a = result;
            }
            "A,u8" => {
                (data1, data2) = (registers.a, Self::fetch_byte(registers, bus));
                let result = data1 & data2;
                registers.a = result;
            }
            op => panic!("Operands not valid: {op}"),
        }

        registers.set_flag(CpuFlag::ZERO, data1 & data2 == 0);
        registers.set_flag(CpuFlag::SUBRACTION, false);
        registers.set_flag(CpuFlag::HALF_CARRY, true);
        registers.set_flag(CpuFlag::CARRY, false);
        opcode.tcycles.0
    }

    pub fn xor(&mut self, opcode: &OpCode, registers: &mut Registers, bus: &mut Bus) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        match operands {
            "A,B" => {
                (data1, data2) = (registers.a, registers.b);
                let result = data1 ^ data2;
                registers.a = result;
            }
            "A,C" => {
                (data1, data2) = (registers.a, registers.c);
                let result = data1 ^ data2;
                registers.a = result;
            }
            "A,D" => {
                (data1, data2) = (registers.a, registers.d);
                let result = data1 ^ data2;
                registers.a = result;
            }
            "A,E" => {
                (data1, data2) = (registers.a, registers.e);
                let result = data1 ^ data2;
                registers.a = result;
            }
            "A,H" => {
                (data1, data2) = (registers.a, registers.h);
                let result = data1 ^ data2;
                registers.a = result;
            }
            "A,L" => {
                (data1, data2) = (registers.a, registers.l);
                let result = data1 ^ data2;
                registers.a = result;
            }
            "A,(HL)" => {
                (data1, data2) = (registers.a, bus.mem_read(registers.hl()));
                let result = data1 ^ data2;
                registers.a = result;
            }
            "A,A" => {
                (data1, data2) = (registers.a, registers.a);
                let result = data1 ^ data2;
                registers.a = result;
            }
            "A,u8" => {
                (data1, data2) = (registers.a, Self::fetch_byte(registers, bus));
                let result = data1 ^ data2;
                registers.a = result;
            }
            op => panic!("Operands not valid: {op}"),
        }

        registers.set_flag(CpuFlag::ZERO, data1 ^ data2 == 0);
        registers.set_flag(CpuFlag::SUBRACTION, false);
        registers.set_flag(CpuFlag::HALF_CARRY, false);
        registers.set_flag(CpuFlag::CARRY, false);
        opcode.tcycles.0
    }

    pub fn or(&mut self, opcode: &OpCode, registers: &mut Registers, bus: &mut Bus) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        match operands {
            "A,B" => {
                (data1, data2) = (registers.a, registers.b);
                let result = data1 | data2;
                registers.a = result;
            }
            "A,C" => {
                (data1, data2) = (registers.a, registers.c);
                let result = data1 | data2;
                registers.a = result;
            }
            "A,D" => {
                (data1, data2) = (registers.a, registers.d);
                let result = data1 | data2;
                registers.a = result;
            }
            "A,E" => {
                (data1, data2) = (registers.a, registers.e);
                let result = data1 | data2;
                registers.a = result;
            }
            "A,H" => {
                (data1, data2) = (registers.a, registers.h);
                let result = data1 | data2;
                registers.a = result;
            }
            "A,L" => {
                (data1, data2) = (registers.a, registers.l);
                let result = data1 | data2;
                registers.a = result;
            }
            "A,(HL)" => {
                (data1, data2) = (registers.a, bus.mem_read(registers.hl()));
                let result = data1 | data2;
                registers.a = result;
            }
            "A,A" => {
                (data1, data2) = (registers.a, registers.a);
                let result = data1 | data2;
                registers.a = result;
            }
            "A,u8" => {
                (data1, data2) = (registers.a, Self::fetch_byte(registers, bus));
                let result = data1 | data2;
                registers.a = result;
            }
            op => panic!("Operands not valid: {op}"),
        }

        registers.set_flag(CpuFlag::ZERO, data1 | data2 == 0);
        registers.set_flag(CpuFlag::SUBRACTION, false);
        registers.set_flag(CpuFlag::HALF_CARRY, false);
        registers.set_flag(CpuFlag::CARRY, false);
        opcode.tcycles.0
    }

    pub fn cp(&mut self, opcode: &OpCode, registers: &mut Registers, bus: &mut Bus) -> u8 {
        let operands = self.get_operands(opcode.mnemonic);
        let (data1, data2);
        match operands {
            "A,B" => (data1, data2) = (registers.a, registers.b),
            "A,C" => (data1, data2) = (registers.a, registers.c),
            "A,D" => (data1, data2) = (registers.a, registers.d),
            "A,E" => (data1, data2) = (registers.a, registers.e),
            "A,H" => (data1, data2) = (registers.a, registers.h),
            "A,L" => (data1, data2) = (registers.a, registers.l),
            "A,(HL)" => (data1, data2) = (registers.a, bus.mem_read(registers.hl())),
            "A,A" => (data1, data2) = (registers.a, registers.a),
            "A,u8" => (data1, data2) = (registers.a, Self::fetch_byte(registers, bus)),
            op => panic!("Operands not valid: {op}"),
        }

        registers.set_flag(CpuFlag::ZERO, data1 - data2 == 0);
        registers.set_flag(CpuFlag::SUBRACTION, false);
        registers.set_flag(CpuFlag::HALF_CARRY, (data1 & 0x0F) < (data2 & 0x0F));
        registers.set_flag(CpuFlag::CARRY, (data1 as u16) < (data2 as u16));
        opcode.tcycles.0
    }

    pub fn daa(&mut self, opcode: &OpCode, registers: &mut Registers, bus: &mut Bus) -> u8 {
        let mut a = registers.a;
        let mut correction = if registers.f.contains(CpuFlag::CARRY) { 0x60 } else { 0x00 };

        if registers.f.contains(CpuFlag::HALF_CARRY) {
            correction |= 0x06;
        }

        if !registers.f.contains(CpuFlag::SUBRACTION) {
            if a & 0x0F > 0x09 {
                correction |= 0x06;
            };
            if a > 0x99 {
                correction |= 0x60;
            }
        }
        a = a.wrapping_add(correction);
        registers.set_flag(CpuFlag::ZERO, a == 0);
        registers.set_flag(CpuFlag::HALF_CARRY, false);
        registers.set_flag(CpuFlag::CARRY, correction >= 0x60);
        registers.a = a;
        opcode.tcycles.0
    }

    pub fn cpl(&mut self, opcode: &OpCode, registers: &mut Registers) -> u8 {
        registers.a = !registers.a;

        registers.set_flag(CpuFlag::SUBRACTION, true);
        registers.set_flag(CpuFlag::HALF_CARRY, true);
        opcode.tcycles.0
    }

    pub fn scf(&mut self, opcode: &OpCode, registers: &mut Registers) -> u8 {
        registers.set_flag(CpuFlag::SUBRACTION, false);
        registers.set_flag(CpuFlag::HALF_CARRY, false);
        registers.set_flag(CpuFlag::CARRY, true);
        opcode.tcycles.0
    }

    pub fn ccf(&mut self, opcode: &OpCode, registers: &mut Registers) -> u8 {
        registers.set_flag(CpuFlag::SUBRACTION, false);
        registers.set_flag(CpuFlag::HALF_CARRY, false);

        let complement = !registers.f.contains(CpuFlag::CARRY);
        registers.set_flag(CpuFlag::CARRY, complement);

        opcode.tcycles.0
    }
}
