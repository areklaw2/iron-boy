use bitflags::Flags;

use crate::bus::{Bus, Memory};

use self::{
    instructions::{get_instruction_by_opcode, AddressingMode, Instruction, RegisterType},
    registers::Registers,
};

pub mod instructions;
mod registers;

pub struct Cpu {
    bus: Bus,
    registers: Registers,
    fetched_data: u16,
    memory_destination: u16,
    destination_is_memory: bool,
    current_instruction: Instruction,
    halted: bool,
    stepping: bool,
}

impl Cpu {
    pub fn new(bus: Bus, registers: Registers) -> Self {
        Cpu {
            bus,
            registers,
            fetched_data: 0,
            memory_destination: 0,
            destination_is_memory: false,
            current_instruction: Instruction::default(),
            halted: false,
            stepping: false,
        }
    }

    fn fetch_instruction(&mut self) {
        let opcode = self.bus.mem_read(self.registers.pc);
        self.registers.pc += 1;
        self.current_instruction = *get_instruction_by_opcode(opcode)
    }

    fn fetch_data(&mut self) {
        match self.current_instruction.addressing_mode {
            AddressingMode::Implied => {}
            AddressingMode::Register => {
                self.fetched_data = self.reg_read(self.current_instruction.register_1) as u16;
            }
            AddressingMode::RegisterAddress => {}
            AddressingMode::RegisterToRegister => {}
            AddressingMode::RegisterToRegisterAddress => {}
            AddressingMode::RegisterToU8Address => {}
            AddressingMode::RegisterToU16Address => {}
            AddressingMode::RegisterAddressToRegister => {}
            AddressingMode::RegisterPlusI8ToRegister => {}
            AddressingMode::I8 => {}
            AddressingMode::U8 => {}
            AddressingMode::U8ToRegister => {
                self.fetched_data = self.fetch_byte() as u16;
            }
            AddressingMode::U8ToRegisterAddress => {}
            AddressingMode::U8AddressToRegister => {}
            AddressingMode::U16 => {
                self.fetched_data = self.fetch_word();
            }
            AddressingMode::U16ToRegister => {}
            AddressingMode::U16AddressToRegister => {}
            AddressingMode::None => {
                // fix this
                panic!("Addressing mode missing")
            }
        }
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

    fn reg_read(&mut self, register: RegisterType) -> u16 {
        match register {
            RegisterType::A => self.registers.a as u16,
            RegisterType::F => self.registers.f.bits() as u16,
            RegisterType::B => self.registers.b as u16,
            RegisterType::C => self.registers.c as u16,
            RegisterType::D => self.registers.d as u16,
            RegisterType::E => self.registers.e as u16,
            RegisterType::H => self.registers.h as u16,
            RegisterType::L => self.registers.l as u16,
            RegisterType::AF => self.registers.af(),
            RegisterType::BC => self.registers.bc(),
            RegisterType::DE => self.registers.de(),
            RegisterType::HL => self.registers.hl(),
            RegisterType::HLI => self.registers.increment_hl(),
            RegisterType::HLD => self.registers.decrement_hl(),
            RegisterType::SP => self.registers.sp,
            RegisterType::PC => self.registers.pc,
            RegisterType::None => 0,
        }
    }

    pub fn cpu_step() {}

    pub fn execute() {
        println!("Not executing")
    }
}
