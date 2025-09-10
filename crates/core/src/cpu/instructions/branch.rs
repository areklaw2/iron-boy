use crate::{MCycleKind, cpu::Cpu, memory::MemoryInterface};

use super::Condition;

pub fn jr_signed_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let signed = cpu.fetch_byte() as i8;
    cpu.registers.set_pc(((cpu.registers.pc() as u32 as i32) + (signed as i32)) as u16);
    cpu.m_cycle(MCycleKind::Idle);
}

pub fn jr_cond_signed_imm8<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let z = cpu.registers.f().zero();
    let c = cpu.registers.f().carry();

    let cond = (cpu.current_opcode & 0b0001_1000) >> 3;
    let jump = match Condition::from(cond) {
        Condition::NC => c == false,
        Condition::C => c == true,
        Condition::NZ => z == false,
        Condition::Z => z == true,
    };

    if jump {
        let signed_byte = cpu.fetch_byte() as i8;
        cpu.registers.set_pc(((cpu.registers.pc() as i16) + (signed_byte as i16)) as u16);
    } else {
        cpu.registers.set_pc(cpu.registers.pc() + 1);
    }
    cpu.m_cycle(MCycleKind::Idle);
}

pub fn jp_cond_imm16<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let z = cpu.registers.f().zero();
    let c = cpu.registers.f().carry();

    let cond = (cpu.current_opcode & 0b0001_1000) >> 3;
    let jump = match Condition::from(cond) {
        Condition::NC => c == false,
        Condition::C => c == true,
        Condition::NZ => z == false,
        Condition::Z => z == true,
    };

    let word = cpu.fetch_word();
    if jump {
        cpu.registers.set_pc(word);
        cpu.m_cycle(MCycleKind::Idle);
    }
}

pub fn jp_imm16<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let word = cpu.fetch_word();
    cpu.registers.set_pc(word);
    cpu.m_cycle(MCycleKind::Idle);
}

pub fn jp_hl<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let hl = cpu.registers.hl();
    cpu.registers.set_pc(hl);
}

pub fn ret_cond<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let z = cpu.registers.f().zero();
    let c = cpu.registers.f().carry();

    let cond = (cpu.current_opcode & 0b0001_1000) >> 3;
    let ret = match Condition::from(cond) {
        Condition::NC => c == false,
        Condition::C => c == true,
        Condition::NZ => z == false,
        Condition::Z => z == true,
    };

    if ret {
        let pop_stack = cpu.pop_stack();
        cpu.registers.set_pc(pop_stack);
        cpu.m_cycle(MCycleKind::Idle);
    }
    cpu.m_cycle(MCycleKind::Idle);
}

pub fn ret<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let pop_stack = cpu.pop_stack();
    cpu.registers.set_pc(pop_stack);
    cpu.m_cycle(MCycleKind::Idle);
}

pub fn reti<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let pop_stack = cpu.pop_stack();
    cpu.registers.set_pc(pop_stack);
    cpu.m_cycle(MCycleKind::Idle);
    cpu.interrupts.set_interrupt_master_enable(true);
}

pub fn call_cond_imm16<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    let z = cpu.registers.f().zero();
    let c = cpu.registers.f().carry();

    let cond = (cpu.current_opcode & 0b0001_1000) >> 3;
    let call = match Condition::from(cond) {
        Condition::NC => c == false,
        Condition::C => c == true,
        Condition::NZ => z == false,
        Condition::Z => z == true,
    };

    let word = cpu.fetch_word();
    if call {
        cpu.push_stack(cpu.registers.pc());
        cpu.registers.set_pc(word);
    }
}

pub fn call_imm16<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    cpu.push_stack(cpu.registers.pc() + 2);
    let word = cpu.fetch_word();
    cpu.registers.set_pc(word);
}

pub fn rst_tgt3<I: MemoryInterface>(cpu: &mut Cpu<I>) {
    cpu.push_stack(cpu.registers.pc());
    let target = cpu.current_opcode & 0b0011_1000;
    cpu.registers.set_pc(target as u16);
}
