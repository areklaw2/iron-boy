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
            InstructionType::RLA => {}
            InstructionType::RRCA => self.rrca(),
            InstructionType::RRA => {}
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
            InstructionType::JR => {}
            InstructionType::JP => self.jp(),
            InstructionType::RET => {}
            InstructionType::RETI => {}
            InstructionType::CALL => {}
            InstructionType::RST => {}
            InstructionType::NOP => {}
            InstructionType::STOP => self.stop(),
            InstructionType::HALT => {}
            InstructionType::CB => {}
            InstructionType::DI => self.di(),
            InstructionType::EI => {}
            InstructionType::NONE => panic!("Invalid instruction!"),
        }
    }

    fn ld(&mut self) {
        println!("Mem Dest: {:#06X}", self.memory_destination);
        println!("Fetched Data: {:#06X}", self.fetched_data);

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

    fn pop(&mut self) {}

    fn push(&mut self) {}

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
        if self.current_instruction.register_1 == RegisterType::HL {
            self.registers.set_flag(CpuFlag::N, false);
            self.registers.set_flag(CpuFlag::H, (data1 & 0x07FF) + (data2 & 0x07FF) > 0x07FF);
            self.registers.set_flag(CpuFlag::C, data1 as u32 + data2 as u32 > 0xFFFF);
            self.registers.set_hl(result);
        }
    }

    fn adc(&mut self) {}

    fn sub(&mut self) {}

    fn sbc(&mut self) {}

    fn and(&mut self) {}

    fn xor(&mut self) {
        let result = self.registers.a ^ self.fetched_data as u8;
        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, false);
        self.registers.a = result;
    }

    fn or(&mut self) {}

    fn cp(&mut self) {}

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

    fn jp(&mut self) {
        let z = self.registers.f.contains(CpuFlag::Z);
        let c = self.registers.f.contains(CpuFlag::C);

        let jump = match self.current_instruction.condition {
            ConditionType::None => true,
            ConditionType::NC => c == false,
            ConditionType::C => c == true,
            ConditionType::NZ => z == false,
            ConditionType::Z => z == false,
        };

        if jump {
            self.registers.pc = self.fetched_data;
        }

        // add cycles!!
    }

    fn stop(&mut self) {
        todo!("Implement with timer")
    }

    fn di(&mut self) {
        self.ime = false;
    }
}
