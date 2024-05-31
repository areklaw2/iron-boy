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
    fn x05_inc_b() {
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
}
