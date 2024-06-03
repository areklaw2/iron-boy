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
    fn x21_ld_de_u16() {
        let mut cpu = get_cpu();
        cpu.bus.mem_write(cpu.registers.pc, 0x21);
        cpu.bus.mem_write_16(cpu.registers.pc + 1, 0x4423);

        let cycles = cpu.cpu_cycle();
        assert_eq!(cycles, 12);
        assert_eq!(cpu.registers.de(), 0x4423)
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
}
