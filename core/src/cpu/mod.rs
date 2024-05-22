use crate::bus::{Bus, Memory};

use self::{
    instructions::{get_instruction_by_opcode, instruction_name, AddressingMode, Instruction, RegisterType},
    registers::Registers,
};

mod execute;
pub mod instructions;
pub mod registers;

pub struct Cpu {
    bus: Bus,
    registers: Registers,
    fetched_data: u16,
    memory_destination: u16,
    destination_is_memory: bool,
    current_opcode: u8,
    current_instruction: Instruction,
    halted: bool,
    ime: bool,
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
            current_opcode: 0x00,
            current_instruction: Instruction::default(),
            halted: false,
            ime: false,
            stepping: false,
        }
    }

    fn fetch_instruction(&mut self) {
        self.current_opcode = self.bus.mem_read(self.registers.pc);
        self.registers.pc += 1;
        self.current_instruction = *get_instruction_by_opcode(self.current_opcode)
    }

    fn fetch_data(&mut self) {
        match self.current_instruction.addressing_mode {
            AddressingMode::Implied => {}
            AddressingMode::Register => self.fetched_data = self.reg_read(self.current_instruction.register_1),
            AddressingMode::RegisterToRegister => self.fetched_data = self.reg_read(self.current_instruction.register_2),
            AddressingMode::RegisterToRegisterAddress => {
                self.fetched_data = self.reg_read(self.current_instruction.register_2);
                self.memory_destination = self.reg_read(self.current_instruction.register_1);
                self.destination_is_memory = true;
                if self.current_instruction.register_1 == RegisterType::C {
                    self.memory_destination |= 0xFF00;
                }
            }
            AddressingMode::RegisterAddressToRegister => {
                let address = self.reg_read(self.current_instruction.register_2);
                if self.current_instruction.register_1 == RegisterType::C {
                    self.memory_destination |= 0xFF00;
                }
                self.fetched_data = self.bus.mem_read(address) as u16;
            }
            AddressingMode::RegisterToU8Address => {
                self.fetched_data = self.reg_read(self.current_instruction.register_2);
                self.memory_destination = self.fetch_byte() as u16 | 0xFF00;
                self.destination_is_memory = true;
            }
            AddressingMode::RegisterToU16Address => {
                self.fetched_data = self.reg_read(self.current_instruction.register_2);
                self.memory_destination = self.fetch_word();
                self.destination_is_memory = true;
            }
            AddressingMode::RegisterAddress => {
                self.memory_destination = self.reg_read(self.current_instruction.register_1);
                self.destination_is_memory = true;
                self.fetched_data = self.bus.mem_read(self.memory_destination) as u16;
            }
            AddressingMode::RegisterPlusI8ToRegister => self.fetched_data = self.fetch_byte() as i8 as i16 as u16,
            AddressingMode::U8ToRegisterAddress => {
                self.memory_destination = self.reg_read(self.current_instruction.register_1);
                self.destination_is_memory = true;
            }
            AddressingMode::U8 | AddressingMode::U8ToRegister | AddressingMode::U8AddressToRegister => self.fetched_data = self.fetch_byte() as u16,
            AddressingMode::U16 | AddressingMode::U16ToRegister => self.fetched_data = self.fetch_word(),
            AddressingMode::U16AddressToRegister => {
                let address = self.fetch_word();
                self.fetched_data = self.bus.mem_read(address) as u16
            }
            //LD HL, SP + i8
            AddressingMode::I8 => {}
            AddressingMode::I8ToRegister => self.fetched_data = self.fetch_byte() as i8 as i16 as u16,
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

    fn reg_write(&mut self, register: RegisterType, data: u16) {
        match register {
            RegisterType::A => self.registers.a = data as u8,
            RegisterType::B => self.registers.b = data as u8,
            RegisterType::C => self.registers.c = data as u8,
            RegisterType::D => self.registers.d = data as u8,
            RegisterType::E => self.registers.e = data as u8,
            RegisterType::H => self.registers.h = data as u8,
            RegisterType::L => self.registers.l = data as u8,
            RegisterType::AF => self.registers.set_af(data),
            RegisterType::BC => self.registers.set_bc(data),
            RegisterType::DE => self.registers.set_de(data),
            RegisterType::HL => self.registers.set_hl(data),
            RegisterType::SP => self.registers.sp = data,
            RegisterType::PC => self.registers.pc = data,
            RegisterType::None => {}
            _ => panic!("Cannot write to register {:?}", register),
        }
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

    pub fn cycle(&mut self) {
        if !self.halted {
            let pc = self.registers.pc;

            self.fetch_instruction();
            self.fetch_data();

            println!(
                "{:#06X}: {:<7} ({:#04X} {:#04X} {:#04X}) A: {:#04X} BC: {:#06X} DE: {:#06X} HL: {:#06X}\n",
                pc,
                instruction_name(&self.current_instruction.instruction_type),
                self.current_opcode,
                self.bus.mem_read(pc + 1),
                self.bus.mem_read(pc + 2),
                self.registers.a,
                self.registers.bc(),
                self.registers.de(),
                self.registers.hl()
            );

            self.execute()
        }
    }

    pub fn execute(&mut self) {
        self.execute_instructions();
    }
}
