use crate::bus::Memory;

use super::{
    instructions::Instruction,
    registers::{R16, R8},
    Cpu,
};

impl Cpu {
    pub fn execute_instructions(&mut self) {
        match self.current_instruction {
            Instruction::Nop => {}
            Instruction::LdR16Imm16 => self.ld_r16_imm16(),
            Instruction::LdR16MemA => self.ld_r16mem_a(),
            Instruction::LdAR16Mem => self.ld_a_r16mem(),
            Instruction::LdImm16Sp => self.ld_imm16_sp(),
            Instruction::LdR8Imm8 => self.ld_r8_imm8(),
            Instruction::LdR8R8 => self.ld_r8_r8(),
            Instruction::LdhCMemA => {}
            Instruction::LdhImm8MemA => {}
            Instruction::LdImm16MemA => {}
            Instruction::LdhACMem => {}
            Instruction::LdhAImm8Mem => {}
            Instruction::LdAImm16Mem => {}
            Instruction::LdHlSpPlusImm8 => {}
            Instruction::LdSpHl => {}
            Instruction::IncR16 => {}
            Instruction::DecR16 => {}
            Instruction::AddHlR16 => {}
            Instruction::IncR8 => {}
            Instruction::DecR8 => {}
            Instruction::Rlca => {}
            Instruction::Rrca => {}
            Instruction::Rla => {}
            Instruction::Rra => {}
            Instruction::Daa => {}
            Instruction::Cpl => {}
            Instruction::Scf => {}
            Instruction::Ccf => {}
            Instruction::JrImm8 => {}
            Instruction::JrCondImm8 => {}
            Instruction::Stop => {}
            Instruction::Halt => {}
            Instruction::AddAR8 => {}
            Instruction::AdcAR8 => {}
            Instruction::SubAR8 => {}
            Instruction::SbcAR8 => {}
            Instruction::AndAR8 => {}
            Instruction::XorAR8 => {}
            Instruction::OrAR8 => {}
            Instruction::CpAR8 => {}
            Instruction::AddAImm8 => {}
            Instruction::AdcAImm8 => {}
            Instruction::SubAImm8 => {}
            Instruction::SbcAImm8 => {}
            Instruction::AndAImm8 => {}
            Instruction::XorAImm8 => {}
            Instruction::OrAImm8 => {}
            Instruction::CpAImm8 => {}
            Instruction::RetCond => {}
            Instruction::Ret => {}
            Instruction::Reti => {}
            Instruction::JpCondImm16 => {}
            Instruction::JpImm16 => {}
            Instruction::JpHl => {}
            Instruction::CallCondImm16 => {}
            Instruction::CallImm16 => {}
            Instruction::RstTgt3 => {}
            Instruction::PopR16Stk => {}
            Instruction::PushR16Stk => {}
            Instruction::Prefix => {}
            Instruction::AddSpImm8 => {}
            Instruction::Di => {}
            Instruction::Ei => {}
            Instruction::None => panic!("Instruction not implemented"),
        }
    }

    fn ld_r16_imm16(&mut self) {
        let destination = self.current_opcode & 0b0011_0000 >> 4;
        let data = self.fetch_word();
        self.reg_write_16(R16::get_register(destination), data);
    }

    fn ld_r16mem_a(&mut self) {
        let destination = self.current_opcode & 0b0011_0000 >> 4;
        let address = self.reg_read_16(R16::get_register(destination));
        self.bus.mem_write(address, self.registers.a);
    }

    fn ld_a_r16mem(&mut self) {
        let source = self.current_opcode & 0b0011_0000 >> 4;
        let address = self.reg_read_16(R16::get_register(source));
        self.registers.a = self.bus.mem_read(address);
    }

    fn ld_imm16_sp(&mut self) {
        let address = self.fetch_word();
        self.bus.mem_write_16(address, self.registers.sp)
    }

    fn ld_r8_imm8(&mut self) {
        let destination = self.current_opcode & 0b0011_1000 >> 3;
        let data = self.fetch_byte();
        self.reg_write_8(R8::get_register(destination), data)
    }

    fn ld_r8_r8(&mut self) {
        let destination = self.current_opcode & 0b0011_1000 >> 3;
        let source = self.current_opcode & 0b0000_0111;
        let data = self.reg_read_8(R8::get_register(source));
        self.reg_write_8(R8::get_register(destination), data)
    }

    //     fn pop(&mut self) {
    //         let data = self.pop_stack();
    //         self.reg_write(self.current_instruction.register_1, data)
    //     }

    //     fn push(&mut self) {
    //         self.push_stack(self.fetched_data);
    //     }

    //     fn inc(&mut self) {
    //         let data = self.fetched_data.wrapping_add(1);

    //         if self.current_instruction.register_1 < R8::AF || self.destination_is_memory {
    //             self.registers.set_flag(CpuFlag::Z, data as u8 == 0);
    //             self.registers.set_flag(CpuFlag::N, false);
    //             self.registers.set_flag(CpuFlag::H, (data as u8 & 0x0F) == 0);

    //             if self.destination_is_memory {
    //                 self.bus.mem_write(self.memory_destination, data as u8);
    //                 return;
    //             }
    //         }

    //         self.reg_write(self.current_instruction.register_1, data);
    //     }

    //     fn dec(&mut self) {
    //         let data = self.fetched_data.wrapping_sub(1);

    //         if self.current_instruction.register_1 < R8::AF || self.destination_is_memory {
    //             self.registers.set_flag(CpuFlag::Z, data as u8 == 0);
    //             self.registers.set_flag(CpuFlag::N, true);
    //             self.registers.set_flag(CpuFlag::H, (data as u8 & 0x0F) == 0x0F);

    //             if self.destination_is_memory {
    //                 self.bus.mem_write(self.memory_destination, data as u8);
    //                 return;
    //             }
    //         }

    //         self.reg_write(self.current_instruction.register_1, data);
    //     }

    //     fn add(&mut self) {
    //         let data1 = self.reg_read(self.current_instruction.register_1);
    //         let data2 = self.fetched_data;
    //         let result = data1.wrapping_add(data2);
    //         match self.current_instruction.register_1 {
    //             R8::HL => {
    //                 self.registers.set_flag(CpuFlag::N, false);
    //                 self.registers.set_flag(CpuFlag::H, (data1 & 0x07FF) + (data2 & 0x07FF) > 0x07FF);
    //                 self.registers.set_flag(CpuFlag::C, data1 as u32 + data2 as u32 > 0xFFFF);
    //                 self.registers.set_hl(result);
    //             }

    //             R8::SP => {
    //                 self.registers.set_flag(CpuFlag::Z, false);
    //                 self.registers.set_flag(CpuFlag::N, false);
    //                 self.registers.set_flag(CpuFlag::H, (data1 & 0x000F) + (data2 & 0x000F) > 0x000F);
    //                 self.registers.set_flag(CpuFlag::C, (data1 & 0x00FF) + (data2 & 0x00FF) > 0x00FF);
    //                 self.registers.sp = result;
    //             }
    //             _ => {
    //                 self.registers.set_flag(CpuFlag::Z, result == 0);
    //                 self.registers.set_flag(CpuFlag::N, false);
    //                 self.registers.set_flag(CpuFlag::H, (data1 as u8 & 0x0F) + (data2 as u8 & 0x0F) > 0x0F);
    //                 self.registers.set_flag(CpuFlag::C, data1 + data2 > 0xFF);
    //                 self.registers.a = result as u8;
    //             }
    //         }
    //     }

    //     fn adc(&mut self) {
    //         let data1 = self.registers.a;
    //         let data2 = self.fetched_data as u8;
    //         let carry = if self.registers.f.contains(CpuFlag::C) {
    //             1
    //         } else {
    //             0
    //         };
    //         let result = data1.wrapping_add(data2).wrapping_add(carry);

    //         self.registers.set_flag(CpuFlag::Z, result == 0);
    //         self.registers.set_flag(CpuFlag::N, false);
    //         self.registers.set_flag(CpuFlag::H, (data1 & 0x0F) + (data2 & 0x0F) + carry > 0x0F);
    //         self.registers.set_flag(CpuFlag::C, data1 as u16 + data2 as u16 + carry as u16 > 0xFF);
    //         self.registers.a = result as u8;
    //     }

    //     fn sub(&mut self) {
    //         let data1 = self.registers.a;
    //         let data2 = self.fetched_data as u8;
    //         let result = data1.wrapping_sub(data2);

    //         self.registers.set_flag(CpuFlag::Z, result == 0);
    //         self.registers.set_flag(CpuFlag::N, true);
    //         self.registers.set_flag(CpuFlag::H, (data1 & 0x0F) < (data2 & 0x0F));
    //         self.registers.set_flag(CpuFlag::C, (data1 as u16) < (data2 as u16));
    //         self.registers.a = result as u8;
    //     }

    //     fn sbc(&mut self) {
    //         let data1 = self.registers.a;
    //         let data2 = self.fetched_data as u8;
    //         let carry = if self.registers.f.contains(CpuFlag::C) {
    //             1
    //         } else {
    //             0
    //         };
    //         let result = data1.wrapping_sub(data2).wrapping_sub(carry);

    //         self.registers.set_flag(CpuFlag::Z, result == 0);
    //         self.registers.set_flag(CpuFlag::N, true);
    //         self.registers.set_flag(CpuFlag::H, (data1 & 0x0F) < (data2 & 0x0F) + carry);
    //         self.registers.set_flag(CpuFlag::C, (data1 as u16) < (data2 as u16) + carry as u16);
    //         self.registers.a = result as u8;
    //     }

    //     fn and(&mut self) {
    //         let result = self.registers.a & self.fetched_data as u8;
    //         self.registers.set_flag(CpuFlag::Z, result == 0);
    //         self.registers.set_flag(CpuFlag::N, false);
    //         self.registers.set_flag(CpuFlag::H, true);
    //         self.registers.set_flag(CpuFlag::C, false);
    //         self.registers.a = result;
    //     }

    //     fn xor(&mut self) {
    //         let result = self.registers.a ^ self.fetched_data as u8;
    //         self.registers.set_flag(CpuFlag::Z, result == 0);
    //         self.registers.set_flag(CpuFlag::N, false);
    //         self.registers.set_flag(CpuFlag::H, false);
    //         self.registers.set_flag(CpuFlag::C, false);
    //         self.registers.a = result;
    //     }

    //     fn or(&mut self) {
    //         let result = self.registers.a | self.fetched_data as u8;
    //         self.registers.set_flag(CpuFlag::Z, result == 0);
    //         self.registers.set_flag(CpuFlag::N, false);
    //         self.registers.set_flag(CpuFlag::H, false);
    //         self.registers.set_flag(CpuFlag::C, false);
    //         self.registers.a = result;
    //     }

    //     fn cp(&mut self) {
    //         let data1 = self.registers.a;
    //         let data2 = self.fetched_data as u8;
    //         let result = data1.wrapping_sub(data2);
    //         self.registers.set_flag(CpuFlag::Z, result == 0);
    //         self.registers.set_flag(CpuFlag::N, true);
    //         self.registers.set_flag(CpuFlag::H, (data1 & 0x0F) < (data1 & 0x0F));
    //         self.registers.set_flag(CpuFlag::C, (data1 as u16) < (data2 as u16));
    //     }

    //     fn rlca(&mut self) {
    //         let carry = self.registers.a & 0x80 == 0x80;
    //         let result = (self.registers.a << 1)
    //             | (if carry {
    //                 1
    //             } else {
    //                 0
    //             });

    //         self.registers.set_flag(CpuFlag::Z, false);
    //         self.registers.set_flag(CpuFlag::N, false);
    //         self.registers.set_flag(CpuFlag::H, false);
    //         self.registers.set_flag(CpuFlag::C, carry);

    //         self.registers.a = result;
    //     }

    //     fn rla(&mut self) {
    //         let carry = self.registers.a & 0x80 == 0x80;
    //         let result = (self.registers.a << 1)
    //             | (if self.registers.f.contains(CpuFlag::C) {
    //                 1
    //             } else {
    //                 0
    //             });

    //         self.registers.set_flag(CpuFlag::Z, false);
    //         self.registers.set_flag(CpuFlag::N, false);
    //         self.registers.set_flag(CpuFlag::H, false);
    //         self.registers.set_flag(CpuFlag::C, carry);

    //         self.registers.a = result;
    //     }

    //     fn rrca(&mut self) {
    //         let carry = self.registers.a & 0x01 == 0x01;
    //         let result = (self.registers.a >> 1)
    //             | (if carry {
    //                 0x80
    //             } else {
    //                 0
    //             });

    //         self.registers.set_flag(CpuFlag::Z, false);
    //         self.registers.set_flag(CpuFlag::N, false);
    //         self.registers.set_flag(CpuFlag::H, false);
    //         self.registers.set_flag(CpuFlag::C, carry);

    //         self.registers.a = result;
    //     }

    //     fn rra(&mut self) {
    //         let carry = self.registers.a & 0x01 == 0x01;
    //         let result = (self.registers.a >> 1)
    //             | (if self.registers.f.contains(CpuFlag::C) {
    //                 0x80
    //             } else {
    //                 0
    //             });

    //         self.registers.set_flag(CpuFlag::Z, false);
    //         self.registers.set_flag(CpuFlag::N, false);
    //         self.registers.set_flag(CpuFlag::H, false);
    //         self.registers.set_flag(CpuFlag::C, carry);

    //         self.registers.a = result;
    //     }

    //     fn jr(&mut self) {
    //         let z = self.registers.f.contains(CpuFlag::Z);
    //         let c = self.registers.f.contains(CpuFlag::C);

    //         let jump = match self.current_instruction.condition {
    //             ConditionType::None => true,
    //             ConditionType::NC => c == false,
    //             ConditionType::C => c == true,
    //             ConditionType::NZ => z == false,
    //             ConditionType::Z => z == true,
    //         };

    //         if jump {
    //             self.registers.pc = ((self.registers.pc as u32 as i32) + (self.fetched_data as i8 as i32)) as u16;
    //         } else {
    //             self.registers.pc += 1
    //         }
    //     }

    //     fn jp(&mut self) {
    //         let z = self.registers.f.contains(CpuFlag::Z);
    //         let c = self.registers.f.contains(CpuFlag::C);

    //         let jump = match self.current_instruction.condition {
    //             ConditionType::None => true,
    //             ConditionType::NC => c == false,
    //             ConditionType::C => c == true,
    //             ConditionType::NZ => z == false,
    //             ConditionType::Z => z == true,
    //         };

    //         if jump {
    //             self.registers.pc = self.fetched_data;
    //         } else {
    //             self.registers.pc += 2
    //         }

    //         // add cycles!!
    //     }

    //     fn ret(&mut self) {
    //         let z = self.registers.f.contains(CpuFlag::Z);
    //         let c = self.registers.f.contains(CpuFlag::C);

    //         let ret = match self.current_instruction.condition {
    //             ConditionType::C => c == true,
    //             ConditionType::Z => z == true,
    //             _ => true,
    //         };

    //         if ret {
    //             self.registers.pc = self.pop_stack();
    //         }
    //     }

    //     fn reti(&mut self) {}

    //     fn call(&mut self) {
    //         let z = self.registers.f.contains(CpuFlag::Z);
    //         let c = self.registers.f.contains(CpuFlag::C);

    //         let jump = match self.current_instruction.condition {
    //             ConditionType::None => true,
    //             ConditionType::NC => c == false,
    //             ConditionType::C => c == true,
    //             ConditionType::NZ => z == false,
    //             ConditionType::Z => z == true,
    //         };

    //         if jump {
    //             self.push_stack(self.registers.pc + 2);
    //             self.registers.pc = self.fetched_data;
    //         } else {
    //             self.registers.pc += 2
    //         }
    //     }

    //     fn rst(&mut self) {
    //         self.push_stack(self.registers.pc);
    //         self.registers.pc = self.current_instruction.parameter.unwrap() as u16
    //     }

    //     fn stop(&mut self) {
    //         // TODO: add speed
    //         panic!("Stop not used in DMG")
    //     }

    //     fn cb(&mut self) {
    //         let opcode = self.fetched_data; // change this to a fetch byte on refactor
    //     }

    //     fn di(&mut self) {
    //         self.ime = false;
    //     }
}
