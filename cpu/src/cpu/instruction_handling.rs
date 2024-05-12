use crate::bus::Memory;

use super::{
    instructions::{AddressingMode, ConditionType, InstructionType, RegisterType},
    registers::{self, CpuFlag},
    Cpu,
};

impl Cpu {
    pub fn handle_instructions(&mut self) {
        match self.current_instruction.instruction_type {
            InstructionType::NOP => {}
            InstructionType::LD => self.ld(),
            InstructionType::INC => {}
            InstructionType::DEC => {}
            InstructionType::RLCA => {}
            InstructionType::ADD => {}
            InstructionType::RRCA => {}
            InstructionType::STOP => {}
            InstructionType::RLA => {}
            InstructionType::JR => {}
            InstructionType::RRA => {}
            InstructionType::DAA => {}
            InstructionType::CPL => {}
            InstructionType::SCF => {}
            InstructionType::CCF => {}
            InstructionType::HALT => {}
            InstructionType::ADC => {}
            InstructionType::SUB => {}
            InstructionType::SBC => {}
            InstructionType::AND => {}
            InstructionType::XOR => self.xor(),
            InstructionType::OR => {}
            InstructionType::CP => {}
            InstructionType::POP => {}
            InstructionType::JP => self.jp(),
            InstructionType::PUSH => {}
            InstructionType::RET => {}
            InstructionType::CB => {}
            InstructionType::CALL => {}
            InstructionType::RETI => {}
            InstructionType::LDH => {}
            InstructionType::JPHL => {}
            InstructionType::DI => self.di(),
            InstructionType::EI => {}
            InstructionType::RST => {}
            InstructionType::ERR => {}
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
            InstructionType::NONE => panic!("Invalid instruction!"),
        }
    }

    fn ld(&mut self) {
        if self.destination_is_memory {
            self.bus.mem_write(self.memory_destination, self.fetched_data as u8);
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

    fn xor(&mut self) {
        let result = self.registers.a ^ self.fetched_data as u8;
        self.registers.set_flag(CpuFlag::Z, result == 0);
        self.registers.set_flag(CpuFlag::N, false);
        self.registers.set_flag(CpuFlag::H, false);
        self.registers.set_flag(CpuFlag::C, false);
        self.registers.a = result;
    }

    fn di(&mut self) {
        self.ime = false;
    }
}
