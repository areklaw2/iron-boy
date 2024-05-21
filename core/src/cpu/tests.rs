use tests::registers::CpuFlag;

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
fn x00_nop() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x00);
    cpu.cycle();

    assert_eq!(cpu.registers.a, 0x01)
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
fn x03_inc_bc() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x03);
    cpu.registers.set_bc(0x20);
    cpu.cycle();

    assert_eq!(cpu.registers.bc(), 0x21);
}

#[test]
fn x04_inc_b() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x04);
    cpu.registers.b = 0x10;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    assert_eq!(cpu.registers.b, 0x11);

    cpu.bus.mem_write(cpu.registers.pc, 0x04);
    cpu.registers.b = 0xFF;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
    assert_eq!(cpu.registers.b, 0x00);
}

#[test]
fn x05_dec_b() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x05);
    cpu.registers.b = 0x10;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
    assert_eq!(cpu.registers.b, 0x0F);

    cpu.bus.mem_write(cpu.registers.pc, 0x05);
    cpu.registers.b = 0x01;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    assert_eq!(cpu.registers.b, 0x00);
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
fn x07_rlca() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x07);
    cpu.registers.a = 0x90;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::C), true);
    assert_eq!(cpu.registers.a, 0x21);

    cpu.bus.mem_write(cpu.registers.pc, 0x07);
    cpu.registers.a = 0x21;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::C), false);
    assert_eq!(cpu.registers.a, 0x42);
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
fn x09_add_hl_bc() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x09);
    cpu.registers.set_hl(0x1120);
    cpu.registers.set_bc(0x1120);
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::C), false);
    assert_eq!(cpu.registers.hl(), 0x2240);

    cpu.bus.mem_write(cpu.registers.pc, 0x09);
    cpu.registers.set_hl(0x8C88);
    cpu.registers.set_bc(0x8C88);
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
    assert_eq!(cpu.registers.f.contains(CpuFlag::C), true);
    assert_eq!(cpu.registers.hl(), 0x1910);
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
fn x0b_dec_bc() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x0B);
    cpu.registers.set_bc(0x20);
    cpu.cycle();

    assert_eq!(cpu.registers.bc(), 0x1F);
}

#[test]
fn x0c_inc_c() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x0C);
    cpu.registers.c = 0x10;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    assert_eq!(cpu.registers.c, 0x11);

    cpu.bus.mem_write(cpu.registers.pc, 0x0C);
    cpu.registers.c = 0xFF;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
    assert_eq!(cpu.registers.c, 0x00);
}

#[test]
fn x0d_dec_c() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x0D);
    cpu.registers.c = 0x10;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
    assert_eq!(cpu.registers.c, 0x0F);

    cpu.bus.mem_write(cpu.registers.pc, 0x0D);
    cpu.registers.c = 0x01;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    assert_eq!(cpu.registers.c, 0x00);
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
fn x0f_rrca() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x0F);
    cpu.registers.a = 0x21;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::C), true);
    assert_eq!(cpu.registers.a, 0x90);

    cpu.bus.mem_write(cpu.registers.pc, 0x0F);
    cpu.registers.a = 0x42;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::C), false);
    assert_eq!(cpu.registers.a, 0x21);
}

#[test]
#[should_panic]
fn x10_stop() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x10);
    cpu.cycle();
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
fn x13_inc_de() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x13);
    cpu.registers.set_de(0x20);
    cpu.cycle();

    assert_eq!(cpu.registers.de(), 0x21);
}

#[test]
fn x14_inc_d() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x14);
    cpu.registers.d = 0x10;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    assert_eq!(cpu.registers.d, 0x11);

    cpu.bus.mem_write(cpu.registers.pc, 0x14);
    cpu.registers.d = 0xFF;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
    assert_eq!(cpu.registers.d, 0x00);
}

#[test]
fn x15_dec_d() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x15);
    cpu.registers.d = 0x10;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
    assert_eq!(cpu.registers.d, 0x0F);

    cpu.bus.mem_write(cpu.registers.pc, 0x15);
    cpu.registers.d = 0x01;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    assert_eq!(cpu.registers.d, 0x00);
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
fn x17_rla() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x17);
    cpu.registers.a = 0x90;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::C), true);
    assert_eq!(cpu.registers.a, 0x21);

    cpu.bus.mem_write(cpu.registers.pc, 0x17);
    cpu.registers.a = 0x21;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::C), false);
    assert_eq!(cpu.registers.a, 0x42);
}

#[test]
fn x18_jr_i8() {
    let mut cpu = _get_cpu();
    panic!();
}

#[test]
fn x19_add_hl_de() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x19);
    cpu.registers.set_hl(0x1120);
    cpu.registers.set_de(0x1120);
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::C), false);
    assert_eq!(cpu.registers.hl(), 0x2240);

    cpu.bus.mem_write(cpu.registers.pc, 0x19);
    cpu.registers.set_hl(0x8C88);
    cpu.registers.set_de(0x8C88);
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
    assert_eq!(cpu.registers.f.contains(CpuFlag::C), true);
    assert_eq!(cpu.registers.hl(), 0x1910);
}

#[test]
fn x1a_ld_a_de() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x1A);
    cpu.registers.set_de(_WORKING_MEMORY);
    cpu.bus.mem_write(cpu.registers.de(), 0x35);
    cpu.cycle();

    assert_eq!(cpu.registers.a, 0x35);
}

#[test]
fn x1b_dec_de() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x1B);
    cpu.registers.set_de(0x20);
    cpu.cycle();

    assert_eq!(cpu.registers.de(), 0x1F);
}

#[test]
fn x1c_inc_e() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x1C);
    cpu.registers.e = 0x10;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    assert_eq!(cpu.registers.e, 0x11);

    cpu.bus.mem_write(cpu.registers.pc, 0x1C);
    cpu.registers.e = 0xFF;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
    assert_eq!(cpu.registers.e, 0x00);
}

#[test]
fn x1d_dec_e() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x1D);
    cpu.registers.e = 0x10;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
    assert_eq!(cpu.registers.e, 0x0F);

    cpu.bus.mem_write(cpu.registers.pc, 0x1D);
    cpu.registers.e = 0x01;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    assert_eq!(cpu.registers.e, 0x00);
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
fn x1f_rca() {
    let mut cpu = _get_cpu();
    cpu.bus.mem_write(cpu.registers.pc, 0x1F);
    cpu.registers.a = 0x21;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::C), true);
    assert_eq!(cpu.registers.a, 0x90);

    cpu.bus.mem_write(cpu.registers.pc, 0x1F);
    cpu.registers.a = 0x42;
    cpu.cycle();

    assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    assert_eq!(cpu.registers.f.contains(CpuFlag::C), false);
    assert_eq!(cpu.registers.a, 0x21);
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
