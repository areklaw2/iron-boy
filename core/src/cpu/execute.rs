use crate::bus::Memory;

use super::{
    instructions::{AddressingMode, ConditionType, InstructionType, RegisterType},
    registers::{self, CpuFlag},
    Cpu,
};

impl Cpu {
    pub fn execute_instructions(&mut self) {
        match self.current_instruction.instruction_type {
            InstructionType::LD => self.ld(),
            InstructionType::POP => self.pop(),
            InstructionType::PUSH => self.push(),
            InstructionType::INC => self.inc(),
            InstructionType::DEC => self.dec(),
            InstructionType::ADD => self.add(),
            InstructionType::ADC => self.adc(),
            InstructionType::SUB => self.sub(),
            InstructionType::SBC => self.sbc(),
            InstructionType::AND => self.and(),
            InstructionType::XOR => self.xor(),
            InstructionType::OR => self.or(),
            InstructionType::CP => self.cp(),
            InstructionType::DAA => {}
            InstructionType::CPL => {}
            InstructionType::SCF => {}
            InstructionType::CCF => {}
            InstructionType::RLCA => self.rlca(),
            InstructionType::RLA => self.rla(),
            InstructionType::RRCA => self.rrca(),
            InstructionType::RRA => self.rra(),
            InstructionType::RLC => {}
            InstructionType::RRC => {}
            InstructionType::RL => {}
            InstructionType::RR => {}
            InstructionType::SLA => {}
            InstructionType::SRA => {}
            InstructionType::SWAP => {}
            InstructionType::SRL => {}
            InstructionType::BIT => {}
            InstructionType::RES => {}
            InstructionType::SET => {}
            InstructionType::JR => self.jr(),
            InstructionType::JP => self.jp(),
            InstructionType::RET => self.ret(),
            InstructionType::RETI => self.reti(),
            InstructionType::CALL => self.call(),
            InstructionType::RST => self.rst(),
            InstructionType::NOP => {}
            InstructionType::STOP => self.stop(),
            InstructionType::HALT => {}
            InstructionType::CB => self.cb(),
            InstructionType::DI => self.di(),
            InstructionType::EI => {}
            InstructionType::NONE => panic!("Invalid instruction!"),
        }
    }

    fn ld(&mut self) {
        // println!("Mem Dest: {:#06X}", self.memory_destination);
        // println!("Fetched Data: {:#06X}", self.fetched_data);

        if self.destination_is_memory {
            if self.current_instruction.register_2 >= RegisterType::AF {
                self.bus.mem_write_16(self.memory_destination, self.fetched_data);
            } else {
                self.bus.mem_write(self.memory_destination, self.fetched_data as u8);
            }
            return;
        }

        if self.current_instruction.addressing_mode == AddressingMode::RegisterPlusI8ToRegister {
            let signed = self.fetched_data;
            let register2 = self.reg_read(self.current_instruction.register_2);

            self.registers.set_flag(CpuFlag::Z, false);
            self.registers.set_flag(CpuFlag::N, false);
            self.registers.set_flag(CpuFlag::H, (register2 & 0x000F) + (signed & 0x000F) > 0x000F);
            self.registers.set_flag(CpuFlag::C, (register2 & 0x00FF) + (signed & 0x00FF) > 0x00FF);

            self.reg_write(self.current_instruction.register_1, register2 + signed)
        }

        self.reg_write(self.current_instruction.register_1, self.fetched_data)
    }

    fn pop(&mut self) {
        let data = self.pop_stack();
        self.reg_write(self.current_instruction.register_1, data)
    }

    fn push(&mut self) {
        self.push_stack(self.fetched_data);
    }

    fn inc(&mut self) {
        let data = self.fetched_data.wrapping_add(1);

        if self.current_instruction.register_1 < RegisterType::AF || self.destination_is_memory {
            self.registers.set_flag(CpuFlag::Z, data as u8 == 0);
            self.registers.set_flag(CpuFlag::N, false);
            self.registers.set_flag(CpuFlag::H, (data as u8 & 0x0F) == 0);

            if self.destination_is_memory {
                self.bus.mem_write(self.memory_destination, data as u8);
                return;
            }
        }

        self.reg_write(self.current_instruction.register_1, data);
    }

    fn dec(&mut self) {
        let data = self.fetched_data.wrapping_sub(1);

        if self.current_instruction.register_1 < RegisterType::AF || self.destination_is_memory {
            self.registers.set_flag(CpuFlag::Z, data as u8 == 0);
            self.registers.set_flag(CpuFlag::N, true);
            self.registers.set_flag(CpuFlag::H, (data as u8 & 0x0F) == 0x0F);

            if self.destination_is_memory {
                self.bus.mem_write(self.memory_destination, data as u8);
                return;
            }
        }

        self.reg_write(self.current_instruction.register_1, data);
    }

    fn add(&mut self) {
        let data1 = self.reg_read(self.current_instruction.register_1);
        let data2 = self.fetched_data;
        let result = data1.wrapping_add(data2);
        match self.current_instruction.register_1 {
            RegisterType::HL => {
                self.registers.set_flag(CpuFlag::N, false);
                self.registers.set_flag(CpuFlag::H, (data1 & 0x07FF) + (data2 & 0x07FF) > 0x07FF);
                self.registers.set_flag(CpuFlag::C, data1 as u32 + data2 as u32 > 0xFFFF);
                self.registers.set_hl(result);
            }

            RegisterType::SP => {
                self.registers.set_flag(CpuFlag::Z, false);
                self.registers.set_flag(CpuFlag::N, false);
                self.registers.set_flag(CpuFlag::H, (data1 & 0x000F) + (data2 & 0x000F) > 0x000F);
                self.registers.set_flag(CpuFlag::C, (data1 & 0x00FF) + (data2 & 0x00FF) > 0x00FF);
                self.registers.sp = result;
            }
            _ => {
                self.registers.set_flag(CpuFlag::Z, result == 0);
                self.registers.set_flag(CpuFlag::N, false);
                self.registers.set_flag(CpuFlag::H, (data1 as u8 & 0x0F) + (data2 as u8 & 0x0F) > 0x0F);
                self.registers.set_flag(CpuFlag::C, data1 + data2 > 0xFF);
                self.registers.a = result as u8;
            }
        }
    }

    fn adc(&mut self) {
        let data1 = self.registers.a;
        let data2 = self.fetched_data as u8;
        let carry = if self.registers.f.contains(CpuFlag::C) {
            1
        } else {
            0
        };
        let result = data1.wrapping_add(data2).wrapping_add(carry);

        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, (data1 & 0x0F) + (data2 & 0x0F) + carry > 0x0F);
        self.registers.set_flag(CpuFlag::C, data1 as u16 + data2 as u16 + carry as u16 > 0xFF);
        self.registers.a = result as u8;
    }

    fn sub(&mut self) {
        let data1 = self.registers.a;
        let data2 = self.fetched_data as u8;
        let result = data1.wrapping_sub(data2);

        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, true);
        self.registers.set_flag(CpuFlag::H, (data1 & 0x0F) < (data2 & 0x0F));
        self.registers.set_flag(CpuFlag::C, (data1 as u16) < (data2 as u16));
        self.registers.a = result as u8;
    }

    fn sbc(&mut self) {
        let data1 = self.registers.a;
        let data2 = self.fetched_data as u8;
        let carry = if self.registers.f.contains(CpuFlag::C) {
            1
        } else {
            0
        };
        let result = data1.wrapping_sub(data2).wrapping_sub(carry);

        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, true);
        self.registers.set_flag(CpuFlag::H, (data1 & 0x0F) < (data2 & 0x0F) + carry);
        self.registers.set_flag(CpuFlag::C, (data1 as u16) < (data2 as u16) + carry as u16);
        self.registers.a = result as u8;
    }

    fn and(&mut self) {
        let result = self.registers.a & self.fetched_data as u8;
        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, true);
        self.registers.set_flag(CpuFlag::C, false);
        self.registers.a = result;
    }

    fn xor(&mut self) {
        let result = self.registers.a ^ self.fetched_data as u8;
        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, false);
        self.registers.a = result;
    }

    fn or(&mut self) {
        let result = self.registers.a | self.fetched_data as u8;
        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, false);
        self.registers.a = result;
    }

    fn cp(&mut self) {
        let data1 = self.registers.a;
        let data2 = self.fetched_data as u8;
        let result = data1.wrapping_sub(data2);
        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, true);
        self.registers.set_flag(CpuFlag::H, (data1 & 0x0F) < (data1 & 0x0F));
        self.registers.set_flag(CpuFlag::C, (data1 as u16) < (data2 as u16));
    }

    fn rlca(&mut self) {
        let carry = self.registers.a & 0x80 == 0x80;
        let result = (self.registers.a << 1)
            | (if carry {
                1
            } else {
                0
            });

        self.registers.set_flag(CpuFlag::Z, false);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, carry);

        self.registers.a = result;
    }

    fn rla(&mut self) {
        let carry = self.registers.a & 0x80 == 0x80;
        let result = (self.registers.a << 1)
            | (if self.registers.f.contains(CpuFlag::C) {
                1
            } else {
                0
            });

        self.registers.set_flag(CpuFlag::Z, false);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, carry);

        self.registers.a = result;
    }

    fn rrca(&mut self) {
        let carry = self.registers.a & 0x01 == 0x01;
        let result = (self.registers.a >> 1)
            | (if carry {
                0x80
            } else {
                0
            });

        self.registers.set_flag(CpuFlag::Z, false);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, carry);

        self.registers.a = result;
    }

    fn rra(&mut self) {
        let carry = self.registers.a & 0x01 == 0x01;
        let result = (self.registers.a >> 1)
            | (if self.registers.f.contains(CpuFlag::C) {
                0x80
            } else {
                0
            });

        self.registers.set_flag(CpuFlag::Z, false);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, carry);

        self.registers.a = result;
    }

    fn jr(&mut self) {
        let z = self.registers.f.contains(CpuFlag::Z);
        let c = self.registers.f.contains(CpuFlag::C);

        let jump = match self.current_instruction.condition {
            ConditionType::None => true,
            ConditionType::NC => c == false,
            ConditionType::C => c == true,
            ConditionType::NZ => z == false,
            ConditionType::Z => z == true,
        };

        if jump {
            self.registers.pc = ((self.registers.pc as u32 as i32) + (self.fetched_data as i8 as i32)) as u16;
        } else {
            self.registers.pc += 1
        }
    }

    fn jp(&mut self) {
        let z = self.registers.f.contains(CpuFlag::Z);
        let c = self.registers.f.contains(CpuFlag::C);

        let jump = match self.current_instruction.condition {
            ConditionType::None => true,
            ConditionType::NC => c == false,
            ConditionType::C => c == true,
            ConditionType::NZ => z == false,
            ConditionType::Z => z == true,
        };

        if jump {
            self.registers.pc = self.fetched_data;
        } else {
            self.registers.pc += 2
        }

        // add cycles!!
    }

    fn ret(&mut self) {
        let z = self.registers.f.contains(CpuFlag::Z);
        let c = self.registers.f.contains(CpuFlag::C);

        let ret = match self.current_instruction.condition {
            ConditionType::C => c == true,
            ConditionType::Z => z == true,
            _ => true,
        };

        if ret {
            self.registers.pc = self.pop_stack();
        }
    }

    fn reti(&mut self) {}

    fn call(&mut self) {
        let z = self.registers.f.contains(CpuFlag::Z);
        let c = self.registers.f.contains(CpuFlag::C);

        let jump = match self.current_instruction.condition {
            ConditionType::None => true,
            ConditionType::NC => c == false,
            ConditionType::C => c == true,
            ConditionType::NZ => z == false,
            ConditionType::Z => z == true,
        };

        if jump {
            self.push_stack(self.registers.pc + 2);
            self.registers.pc = self.fetched_data;
        } else {
            self.registers.pc += 2
        }
    }

    fn rst(&mut self) {
        self.push_stack(self.registers.pc);
        self.registers.pc = self.current_instruction.parameter.unwrap() as u16
    }

    fn stop(&mut self) {
        // TODO: add speed
        panic!("Stop not used in DMG")
    }

    fn cb(&mut self) {}

    fn di(&mut self) {
        self.ime = false;
    }
}
