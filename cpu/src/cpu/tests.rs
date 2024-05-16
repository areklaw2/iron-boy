use crate::cartridge::Cartridge;

use super::*;

const _WORKING_MEMORY: u16 = 0xC001;

fn _get_cpu() -> Cpu {
    let cartridge = Cartridge::default();
    let bus = Bus::new(cartridge);
    let mut registers = Registers::new(utils::Mode::Monochrome);
    registers.pc = 0xC000;
    return Cpu::new(bus, registers);
}

#[test]
fn x01_ld_bc_u16() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x01);
    cpu.bus.mem_write_16(cpu.registers.pc + 1, 0x1234);
    cpu.cycle();

    assert_eq!(cpu.registers.bc(), 0x1234);
}

#[test]
fn x02_ld_bc_a() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x02);
    cpu.registers.set_bc(_WORKING_MEMORY);
    cpu.registers.a = 0x03;
    cpu.cycle();

    assert_eq!(cpu.bus.mem_read(cpu.registers.bc()), cpu.registers.a);
}

#[test]
fn x06_ld_b_u8() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x06);
    cpu.bus.mem_write(cpu.registers.pc + 1, 0x12);
    cpu.cycle();

    assert_eq!(cpu.registers.b, 0x12);
}

#[test]
fn x08_ld_u16_sp() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x08);
    cpu.bus.mem_write_16(cpu.registers.pc + 1, _WORKING_MEMORY);
    cpu.registers.sp = 0x1555;
    cpu.cycle();

    assert_eq!(cpu.bus.mem_read_16(_WORKING_MEMORY), 0x1555);
}

#[test]
fn x0a_ld_a_bc() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x0A);
    cpu.registers.set_bc(_WORKING_MEMORY);
    cpu.bus.mem_write(cpu.registers.bc(), 0x35);
    cpu.cycle();

    assert_eq!(cpu.registers.a, 0x35);
}

#[test]
fn x0e_ld_c_u8() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x0e);
    cpu.bus.mem_write(cpu.registers.pc + 1, 0x12);
    cpu.cycle();

    assert_eq!(cpu.registers.c, 0x12);
}

#[test]
fn x11_ld_de_u16() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x11);
    cpu.bus.mem_write_16(cpu.registers.pc + 1, 0x1234);
    cpu.cycle();

    assert_eq!(cpu.registers.de(), 0x1234);
}

#[test]
fn x12_ld_de_a() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x12);
    cpu.registers.set_de(_WORKING_MEMORY);
    cpu.registers.a = 0x03;
    cpu.cycle();

    assert_eq!(cpu.bus.mem_read(cpu.registers.de()), cpu.registers.a);
}

#[test]
fn x16_ld_d_u8() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x16);
    cpu.bus.mem_write(cpu.registers.pc + 1, 0x12);
    cpu.cycle();

    assert_eq!(cpu.registers.d, 0x12);
}

#[test]
fn x0a_ld_a_de() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x1A);
    cpu.registers.set_de(_WORKING_MEMORY);
    cpu.bus.mem_write(cpu.registers.de(), 0x35);
    cpu.cycle();

    assert_eq!(cpu.registers.a, 0x35);
}

#[test]
fn x1e_ld_e_u8() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x1e);
    cpu.bus.mem_write(cpu.registers.pc + 1, 0x12);
    cpu.cycle();

    assert_eq!(cpu.registers.e, 0x12);
}

#[test]
fn xe0_ff00_u8_a() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0xE0);
    cpu.bus.mem_write(cpu.registers.pc + 1, 0x80);
    cpu.registers.a = 0x22;
    cpu.cycle();

    assert_eq!(cpu.bus.mem_read(0xFF80), 0x22);
}

#[test]
fn xe2_ff00_c_a() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0xE2);
    cpu.registers.c = 0x80;
    cpu.registers.a = 0x22;
    cpu.cycle();

    assert_eq!(cpu.bus.mem_read(0xFF80), 0x22);
}
