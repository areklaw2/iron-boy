#[cfg(test)]
mod tests {
    use crate::{
        bus::{Bus, Memory},
        cartridge::Cartridge,
        cpu::{
            self,
            registers::{CpuFlag, Registers},
            Cpu,
        },
    };

    const WRAM_START: u16 = 0xC000;

    fn get_cpu() -> Cpu {
        let bus = Bus::new(Cartridge::default());
        let registers = Registers::new(utils::Mode::Monochrome);
        let mut cpu = Cpu::new(bus, registers);
        cpu.registers.pc = WRAM_START;
        cpu
    }

    #[test]
    fn x00_nop() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x00);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
    }

    #[test]
    fn x01_ld_bc_u16() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x01);
        cpu.bus.mem_write_16(cpu.registers.pc + 1, 0x4423);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 12);
        assert_eq!(cpu.registers.bc(), 0x4423)
    }

    #[test]
    fn x02_ld_mem_bc_a() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x02);
        cpu.registers.set_bc(WRAM_START + 1);
        cpu.registers.a = 0x35;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.bus.mem_read(cpu.registers.bc()), cpu.registers.a)
    }

    #[test]
    fn x03_inc_bc() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x03);
        cpu.registers.set_bc(0x2222);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.bc(), 0x2223)
    }

    #[test]
    fn x04_inc_b() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x04);
        cpu.registers.b = 0x0F;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.b, 0x10);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);

        cpu.bus.mem_write(cpu.registers.pc, 0x04);
        cpu.registers.b = 0xFF;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.b, 0x0);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
    }

    #[test]
    fn x05_dec_b() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x05);
        cpu.registers.b = 0x10;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.b, 0x0F);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);

        cpu.bus.mem_write(cpu.registers.pc, 0x05);
        cpu.registers.b = 0x01;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.b, 0x00);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    }

    #[test]
    fn x06_ld_b_u8() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x06);
        cpu.bus.mem_write(cpu.registers.pc + 1, 0x55);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.b, 0x55);
    }

    #[test]
    fn x07_rlca() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x07);
        cpu.registers.a = 0b0100_1010;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.a, 0b1001_0100);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::C), false);

        cpu.bus.mem_write(cpu.registers.pc, 0x07);
        cpu.registers.a = 0b1100_1010;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.a, 0b1001_0101);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::C), true);
    }

    #[test]
    fn x08_ld_u16_mem_sp() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x08);
        cpu.bus.mem_write_16(cpu.registers.pc + 1, WRAM_START);
        cpu.registers.sp = 0x88;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 20);
        assert_eq!(cpu.bus.mem_read(WRAM_START), 0x88);
    }

    #[test]
    fn x09_add_hl_bc() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x09);
        cpu.registers.set_hl(0x2000);
        cpu.registers.set_bc(0x5500);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.hl(), 0x7500);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::C), false);

        cpu.bus.mem_write(cpu.registers.pc, 0x09);
        cpu.registers.set_hl(0xFFF0);
        cpu.registers.set_bc(0x0010);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.hl(), 0x0000);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::C), true);
    }

    #[test]
    fn x0a_ld_a_bc_mem() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x0A);
        cpu.registers.set_bc(WRAM_START + 1);
        cpu.bus.mem_write(WRAM_START + 1, 0x55);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.a, 0x55);
    }

    #[test]
    fn x0b_dec_bc() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x0B);
        cpu.registers.set_bc(0x2222);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.bc(), 0x2221)
    }

    #[test]
    fn x0c_inc_c() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x0C);
        cpu.registers.c = 0x0F;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.c, 0x10);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);

        cpu.bus.mem_write(cpu.registers.pc, 0x0C);
        cpu.registers.c = 0xFF;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.c, 0x0);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
    }

    #[test]
    fn x0d_dec_c() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x0D);
        cpu.registers.c = 0x10;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.c, 0x0F);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);

        cpu.bus.mem_write(cpu.registers.pc, 0x0D);
        cpu.registers.c = 0x01;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.c, 0x00);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    }

    #[test]
    fn x0e_ld_c_u8() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x0E);
        cpu.bus.mem_write(cpu.registers.pc + 1, 0x55);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.c, 0x55);
    }

    #[test]
    fn x0f_rrca() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x0F);
        cpu.registers.a = 0b0100_1010;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.a, 0b0010_0101);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::C), false);

        cpu.bus.mem_write(cpu.registers.pc, 0x0F);
        cpu.registers.a = 0b1001_0101;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.a, 0b1100_1010);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::C), true);
    }

    #[test]
    fn x10_stop() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x10);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
    }

    #[test]
    fn x11_ld_de_u16() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x11);
        cpu.bus.mem_write_16(cpu.registers.pc + 1, 0x4423);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 12);
        assert_eq!(cpu.registers.de(), 0x4423)
    }

    #[test]
    fn x12_ld_mem_de_a() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x12);
        cpu.registers.set_de(WRAM_START + 1);
        cpu.registers.a = 0x35;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.bus.mem_read(cpu.registers.de()), cpu.registers.a)
    }

    #[test]
    fn x13_inc_de() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x13);
        cpu.registers.set_de(0x2222);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.de(), 0x2223)
    }

    #[test]
    fn x14_inc_d() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x14);
        cpu.registers.d = 0x0F;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.d, 0x10);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);

        cpu.bus.mem_write(cpu.registers.pc, 0x14);
        cpu.registers.d = 0xFF;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.d, 0x0);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
    }

    #[test]
    fn x15_dec_d() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x15);
        cpu.registers.d = 0x10;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.d, 0x0F);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);

        cpu.bus.mem_write(cpu.registers.pc, 0x15);
        cpu.registers.d = 0x01;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.d, 0x00);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    }

    #[test]
    fn x16_ld_d_u8() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x16);
        cpu.bus.mem_write(cpu.registers.pc + 1, 0x55);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.d, 0x55);
    }

    #[test]
    fn x17_rla() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x17);
        cpu.registers.set_flag(CpuFlag::C, false);
        cpu.registers.a = 0b0100_1010;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.a, 0b1001_0100);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::C), false);

        cpu.bus.mem_write(cpu.registers.pc, 0x17);
        cpu.registers.set_flag(CpuFlag::C, true);
        cpu.registers.a = 0b1100_1010;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.a, 0b1001_0101);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::C), true);
    }

    #[test]
    fn x18_jr_i8() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x18);
        cpu.bus.mem_write(cpu.registers.pc + 1, 0xF0);
        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 12);
        assert_eq!(cpu.registers.pc, 0xBFF1);
    }

    #[test]
    fn x19_add_hl_de() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x19);
        cpu.registers.set_hl(0x2000);
        cpu.registers.set_de(0x5500);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.hl(), 0x7500);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::C), false);

        cpu.bus.mem_write(cpu.registers.pc, 0x19);
        cpu.registers.set_hl(0xFFF0);
        cpu.registers.set_de(0x0010);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.hl(), 0x0000);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::C), true);
    }

    #[test]
    fn x1a_ld_a_de_mem() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x1A);
        cpu.registers.set_de(WRAM_START + 1);
        cpu.bus.mem_write(WRAM_START + 1, 0x55);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.a, 0x55);
    }

    #[test]
    fn x1b_dec_de() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x1B);
        cpu.registers.set_de(0x2222);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.de(), 0x2221)
    }

    #[test]
    fn x1c_inc_e() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x1C);
        cpu.registers.e = 0x0F;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.e, 0x10);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);

        cpu.bus.mem_write(cpu.registers.pc, 0x1C);
        cpu.registers.e = 0xFF;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.e, 0x0);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
    }

    #[test]
    fn x1d_dec_e() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x1D);
        cpu.registers.e = 0x10;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.e, 0x0F);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);

        cpu.bus.mem_write(cpu.registers.pc, 0x1D);
        cpu.registers.e = 0x01;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.e, 0x00);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    }

    #[test]
    fn x1e_ld_e_u8() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x1E);
        cpu.bus.mem_write(cpu.registers.pc + 1, 0x55);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.e, 0x55);
    }

    #[test]
    fn x1f_rca() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x1F);
        cpu.registers.set_flag(CpuFlag::C, false);
        cpu.registers.a = 0b0100_1010;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.a, 0b0010_0101);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::C), false);

        cpu.bus.mem_write(cpu.registers.pc, 0x1F);
        cpu.registers.set_flag(CpuFlag::C, true);
        cpu.registers.a = 0b1100_1011;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.a, 0b1110_0101);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::C), true);
    }

    #[test]
    fn x20_jr_nz_i8() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x20);
        cpu.bus.mem_write(cpu.registers.pc + 1, 0xF0);
        cpu.registers.set_flag(CpuFlag::Z, false);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 12);
        assert_eq!(cpu.registers.pc, 0xBFF1);

        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x20);
        cpu.bus.mem_write(cpu.registers.pc + 1, 0xF0);
        cpu.registers.set_flag(CpuFlag::Z, true);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.pc, 0xC002);
    }

    #[test]
    fn x21_ld_hl_u16() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x21);
        cpu.bus.mem_write_16(cpu.registers.pc + 1, 0x4423);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 12);
        assert_eq!(cpu.registers.hl(), 0x4423)
    }

    #[test]
    fn x22_ld_mem_hl_inc_a() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x22);
        cpu.registers.set_hl(WRAM_START + 1);
        cpu.registers.a = 0x35;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.bus.mem_read(cpu.registers.increment_hl() - 1), cpu.registers.a)
    }

    #[test]
    fn x23_inc_hl() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x23);
        cpu.registers.set_hl(0x2222);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.hl(), 0x2223)
    }

    #[test]
    fn x24_inc_h() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x24);
        cpu.registers.h = 0x0F;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.h, 0x10);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);

        cpu.bus.mem_write(cpu.registers.pc, 0x24);
        cpu.registers.h = 0xFF;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.h, 0x0);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
    }

    #[test]
    fn x25_dec_h() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x25);
        cpu.registers.h = 0x10;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.h, 0x0F);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);

        cpu.bus.mem_write(cpu.registers.pc, 0x25);
        cpu.registers.h = 0x01;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.h, 0x00);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    }

    #[test]
    fn x26_ld_d_u8() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x26);
        cpu.bus.mem_write(cpu.registers.pc + 1, 0x55);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.h, 0x55);
    }

    #[test]
    fn x27_daa() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x27);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
    }

    #[test]
    fn x28_jr_z_i8() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x28);
        cpu.bus.mem_write(cpu.registers.pc + 1, 0xF0);
        cpu.registers.set_flag(CpuFlag::Z, true);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 12);
        assert_eq!(cpu.registers.pc, 0xBFF1);

        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x28);
        cpu.bus.mem_write(cpu.registers.pc + 1, 0xF0);
        cpu.registers.set_flag(CpuFlag::Z, false);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.pc, 0xC002);
    }

    #[test]
    fn x29_add_hl_hl() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x29);
        cpu.registers.set_hl(0x3A80);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.hl(), 0x7500);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::C), false);

        cpu.bus.mem_write(cpu.registers.pc, 0x29);
        cpu.registers.set_hl(0xFFF0);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.hl(), 0xFFE0);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::C), true);
    }

    #[test]
    fn x2a_ld_a_hli_mem() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x2A);
        cpu.registers.set_hl(WRAM_START + 1);
        cpu.bus.mem_write(WRAM_START + 2, 0x55);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.a, 0x55);
    }

    #[test]
    fn x2b_dec_hl() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x2B);
        cpu.registers.set_hl(0x2222);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.hl(), 0x2221)
    }

    #[test]
    fn x2c_inc_l() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x2C);
        cpu.registers.l = 0x0F;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.l, 0x10);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);

        cpu.bus.mem_write(cpu.registers.pc, 0x2C);
        cpu.registers.l = 0xFF;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.l, 0x0);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
    }

    #[test]
    fn x2d_dec_l() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x2D);
        cpu.registers.l = 0x10;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.l, 0x0F);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);

        cpu.bus.mem_write(cpu.registers.pc, 0x2D);
        cpu.registers.l = 0x01;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.l, 0x00);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    }

    #[test]
    fn x2e_ld_l_u8() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x2E);
        cpu.bus.mem_write(cpu.registers.pc + 1, 0x55);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.l, 0x55);
    }

    #[test]
    fn x2f_cpl() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x2F);
        cpu.registers.a = 0x50;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.a, 0xAF);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
    }

    #[test]
    fn x30_jr_nc_i8() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x30);
        cpu.bus.mem_write(cpu.registers.pc + 1, 0xF0);
        cpu.registers.set_flag(CpuFlag::C, false);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 12);
        assert_eq!(cpu.registers.pc, 0xBFF1);

        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x30);
        cpu.bus.mem_write(cpu.registers.pc + 1, 0xF0);
        cpu.registers.set_flag(CpuFlag::C, true);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.pc, 0xC002);
    }

    #[test]
    fn x31_ld_sp_u16() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x31);
        cpu.bus.mem_write_16(cpu.registers.pc + 1, 0x4423);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 12);
        assert_eq!(cpu.registers.sp, 0x4423)
    }

    #[test]
    fn x32_ld_mem_hl_dec_a() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x32);
        cpu.registers.set_hl(WRAM_START + 1);
        cpu.registers.a = 0x35;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.bus.mem_read(cpu.registers.decrement_hl() + 1), cpu.registers.a)
    }

    #[test]
    fn x33_inc_sp() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x33);
        cpu.registers.sp = 0x3222;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.sp, 0x3223)
    }

    #[test]
    fn x34_inc_hl_mem() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x34);
        cpu.registers.set_hl(WRAM_START + 1);
        cpu.bus.mem_write(cpu.registers.hl(), 0x0F);
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 12);
        assert_eq!(cpu.bus.mem_read(cpu.registers.hl()), 0x10);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);

        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x34);
        cpu.registers.set_hl(WRAM_START + 1);
        cpu.bus.mem_write(cpu.registers.hl(), 0xFF);
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 12);
        assert_eq!(cpu.bus.mem_read(cpu.registers.hl()), 0x0);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
    }

    #[test]
    fn x35_dec_hl_mem() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x35);
        cpu.registers.set_hl(WRAM_START + 1);
        cpu.bus.mem_write(cpu.registers.hl(), 0x10);
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 12);
        assert_eq!(cpu.bus.mem_read(cpu.registers.hl()), 0x0F);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);

        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x35);
        cpu.registers.set_hl(WRAM_START + 1);
        cpu.bus.mem_write(cpu.registers.hl(), 0x01);
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 12);
        assert_eq!(cpu.bus.mem_read(cpu.registers.hl()), 0x00);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    }

    #[test]
    fn x36_ld_hl_u8() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x36);
        cpu.bus.mem_write(cpu.registers.pc + 1, 0x55);
        cpu.registers.set_hl(WRAM_START + 2);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 12);
        assert_eq!(cpu.bus.mem_read(cpu.registers.hl()), 0x55);
    }

    #[test]
    fn x37_scf() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x37);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.f.contains(CpuFlag::C), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
    }

    #[test]
    fn x38_jr_c_i8() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x38);
        cpu.bus.mem_write(cpu.registers.pc + 1, 0xF0);
        cpu.registers.set_flag(CpuFlag::C, true);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 12);
        assert_eq!(cpu.registers.pc, 0xBFF1);

        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x38);
        cpu.bus.mem_write(cpu.registers.pc + 1, 0xF0);
        cpu.registers.set_flag(CpuFlag::C, false);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.pc, 0xC002);
    }

    #[test]
    fn x39_add_hl_sp() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x39);
        cpu.registers.set_hl(0x2000);
        cpu.registers.sp = 0x5500;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.hl(), 0x7500);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::C), false);

        cpu.bus.mem_write(cpu.registers.pc, 0x39);
        cpu.registers.set_hl(0xFFF0);
        cpu.registers.sp = 0x0010;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.hl(), 0x0000);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::C), true);
    }

    #[test]
    fn x3a_ld_a_hld_mem() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x3A);
        cpu.registers.set_hl(WRAM_START + 2);
        cpu.bus.mem_write(WRAM_START + 1, 0x55);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.a, 0x55);
    }

    #[test]
    fn x3b_dec_sp() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x3B);
        cpu.registers.sp = 0x3222;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.sp, 0x3221)
    }

    #[test]
    fn x3c_inc_a() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x3C);
        cpu.registers.a = 0x0F;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);

        cpu.bus.mem_write(cpu.registers.pc, 0x3C);
        cpu.registers.a = 0xFF;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.a, 0x0);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);
    }

    #[test]
    fn x3d_dec_a() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x3D);
        cpu.registers.a = 0x10;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.a, 0x0F);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), true);

        cpu.bus.mem_write(cpu.registers.pc, 0x3D);
        cpu.registers.a = 0x01;
        let cycles = cpu.cpu_cycle();

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.registers.f.contains(CpuFlag::Z), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
    }

    #[test]
    fn x3e_ld_a_u8() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x3E);
        cpu.bus.mem_write(cpu.registers.pc + 1, 0x55);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.a, 0x55);
    }

    #[test]
    fn x3f_ccf() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x3F);
        cpu.registers.set_flag(CpuFlag::C, false);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.f.contains(CpuFlag::C), true);
        assert_eq!(cpu.registers.f.contains(CpuFlag::H), false);
        assert_eq!(cpu.registers.f.contains(CpuFlag::N), false);
    }

    #[test]
    fn x40_ld_b_b() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x40);
        cpu.registers.b = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.b, 34);
    }

    #[test]
    fn x41_ld_b_c() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x41);
        cpu.registers.c = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.b, 34);
    }

    #[test]
    fn x42_ld_b_d() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x42);
        cpu.registers.d = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.b, 34);
    }

    #[test]
    fn x43_ld_b_e() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x43);
        cpu.registers.e = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.b, 34);
    }

    #[test]
    fn x44_ld_b_h() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x44);
        cpu.registers.h = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.b, 34);
    }

    #[test]
    fn x45_ld_b_e() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x45);
        cpu.registers.l = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.b, 34);
    }

    #[test]
    fn x46_ld_b_hl_mem() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x46);
        cpu.registers.set_hl(WRAM_START + 1);
        cpu.bus.mem_write(cpu.registers.hl(), 34);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.b, 34);
    }

    #[test]
    fn x47_ld_b_a() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x47);
        cpu.registers.a = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.b, 34);
    }

    #[test]
    fn x48_ld_c_b() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x48);
        cpu.registers.b = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.c, 34);
    }

    #[test]
    fn x49_ld_c_c() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x49);
        cpu.registers.c = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.c, 34);
    }

    #[test]
    fn x4a_ld_c_d() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x4a);
        cpu.registers.d = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.c, 34);
    }

    #[test]
    fn x4b_ld_c_e() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x4b);
        cpu.registers.e = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.c, 34);
    }

    #[test]
    fn x4c_ld_c_h() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x4c);
        cpu.registers.h = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.c, 34);
    }

    #[test]
    fn x4d_ld_c_e() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x4d);
        cpu.registers.l = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.c, 34);
    }

    #[test]
    fn x4e_ld_c_hl_mem() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x4e);
        cpu.registers.set_hl(WRAM_START + 1);
        cpu.bus.mem_write(cpu.registers.hl(), 34);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.c, 34);
    }

    #[test]
    fn x4f_ld_c_a() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x4f);
        cpu.registers.a = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.c, 34);
    }

    #[test]
    fn x50_ld_d_b() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x50);
        cpu.registers.b = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.d, 34);
    }

    #[test]
    fn x51_ld_d_c() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x51);
        cpu.registers.c = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.d, 34);
    }

    #[test]
    fn x52_ld_d_d() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x52);
        cpu.registers.d = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.d, 34);
    }

    #[test]
    fn x53_ld_d_e() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x53);
        cpu.registers.e = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.d, 34);
    }

    #[test]
    fn x54_ld_d_h() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x54);
        cpu.registers.h = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.d, 34);
    }

    #[test]
    fn x55_ld_d_e() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x55);
        cpu.registers.l = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.d, 34);
    }

    #[test]
    fn x56_ld_d_hl_mem() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x56);
        cpu.registers.set_hl(WRAM_START + 1);
        cpu.bus.mem_write(cpu.registers.hl(), 34);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.d, 34);
    }

    #[test]
    fn x57_ld_d_a() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x57);
        cpu.registers.a = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.d, 34);
    }

    #[test]
    fn x58_ld_e_b() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x58);
        cpu.registers.b = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.e, 34);
    }

    #[test]
    fn x59_ld_e_c() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x59);
        cpu.registers.c = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.e, 34);
    }

    #[test]
    fn x5a_ld_e_d() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x5a);
        cpu.registers.d = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.e, 34);
    }

    #[test]
    fn x5b_ld_e_e() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x5b);
        cpu.registers.e = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.e, 34);
    }

    #[test]
    fn x5c_ld_e_h() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x5c);
        cpu.registers.h = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.e, 34);
    }

    #[test]
    fn x5d_ld_e_e() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x5d);
        cpu.registers.l = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.e, 34);
    }

    #[test]
    fn x5e_ld_c_hl_mem() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x5e);
        cpu.registers.set_hl(WRAM_START + 1);
        cpu.bus.mem_write(cpu.registers.hl(), 34);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 8);
        assert_eq!(cpu.registers.e, 34);
    }

    #[test]
    fn x5f_ld_e_a() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x5f);
        cpu.registers.a = 34;

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.e, 34);
    }
}
