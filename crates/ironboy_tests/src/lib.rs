use ironboy_core::cpu::MemoryInterface;

use serde::{Deserialize, Serialize};

pub struct SimpleBus {
    data: Vec<u8>,
    cycles: u8,
}

impl SimpleBus {
    #[allow(dead_code)]
    pub fn new() -> SimpleBus {
        SimpleBus {
            data: vec![0; 0x10000],
            cycles: 0,
        }
    }
}

impl MemoryInterface for SimpleBus {
    fn load_8(&mut self, address: u16, with_cycles: bool) -> u8 {
        if with_cycles {
            self.m_cycle();
        }
        self.data[address as usize]
    }

    fn store_8(&mut self, address: u16, value: u8, with_cycles: bool) {
        if with_cycles {
            self.m_cycle();
        }
        self.data[address as usize] = value
    }

    fn m_cycle(&mut self) {
        self.cycles += 1
    }

    fn total_m_cycles(&self) -> u64 {
        self.cycles as u64
    }

    fn pending_interrupt(&self) -> u8 {
        0
    }

    fn clear_interrupt(&mut self, _sinterrupt_bit: u8) {}

    fn change_speed(&mut self) {}
}

#[derive(Debug, Serialize, Deserialize)]
struct State {
    pc: u16,
    sp: u16,
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
    ram: Vec<[u16; 2]>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Test {
    name: String,
    initial: State,
    r#final: State,
    cycles: Vec<(u16, u8, String)>,
}

#[cfg(test)]
mod tests {
    use ironboy_core::{
        GbMode,
        cpu::{Cpu, instructions::Instruction},
    };
    use std::{cell::RefCell, fs, rc::Rc};

    use super::*;

    #[test]
    fn single_step_tests() {
        let directory = fs::read_dir("../../external/sm83/v1").expect("Unable to read directory");

        let mut files: Vec<_> = directory.collect::<Result<Vec<_>, _>>().expect("Unable to collect files");
        files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

        for file in files {
            let file = file.path();
            let test_json = fs::read_to_string(file).expect("Unable to read file");
            let tests: Vec<Test> = serde_json::from_str(&test_json).expect("Unable to serilize test");
            for test in tests {
                let inital_state = test.initial;
                let final_state = test.r#final;

                let mut cpu = Cpu::new(SimpleBus::new(), GbMode::Monochrome, Rc::new(RefCell::new(false)));

                cpu.registers_mut().set_pc(inital_state.pc);
                cpu.registers_mut().set_sp(inital_state.sp);
                cpu.registers_mut().set_a(inital_state.a);
                cpu.registers_mut().set_b(inital_state.b);
                cpu.registers_mut().set_c(inital_state.c);
                cpu.registers_mut().set_d(inital_state.d);
                cpu.registers_mut().set_e(inital_state.e);
                cpu.registers_mut().set_f(inital_state.f.into());
                cpu.registers_mut().set_h(inital_state.h);
                cpu.registers_mut().set_l(inital_state.l);

                for data in inital_state.ram {
                    cpu.bus_mut().store_8(data[0], data[1] as u8, false);
                    assert_eq!(
                        cpu.bus_mut().load_8(data[0], false),
                        data[1] as u8,
                        "Initial test data load mismatch for test {}",
                        test.name
                    )
                }

                cpu.fetch_instruction();
                cpu.execute_instruction();

                if !matches!(cpu.instruction(), Instruction::Stop | Instruction::Halt) {
                    assert_eq!(
                        cpu.bus().total_m_cycles() as usize,
                        test.cycles.len(),
                        "Cycle count mismatch for test {}",
                        test.name
                    );
                }

                assert_eq!(cpu.registers().pc(), final_state.pc, "PC mismatch for test {}", test.name);
                assert_eq!(cpu.registers().sp(), final_state.sp, "SP mismatch for test {}", test.name);
                assert_eq!(cpu.registers().a(), final_state.a, "A mismatch for test {}", test.name);
                assert_eq!(cpu.registers().b(), final_state.b, "B mismatch for test {}", test.name);
                assert_eq!(cpu.registers().c(), final_state.c, "C mismatch for test {}", test.name);
                assert_eq!(cpu.registers().d(), final_state.d, "D mismatch for test {}", test.name);
                assert_eq!(cpu.registers().e(), final_state.e, "E mismatch for test {}", test.name);
                assert_eq!(cpu.registers().f().into_bits(), final_state.f, "F mismatch for test {}", test.name);
                assert_eq!(cpu.registers().h(), final_state.h, "H mismatch for test {}", test.name);
                assert_eq!(cpu.registers().l(), final_state.l, "L mismatch for test {}", test.name);

                for data in final_state.ram {
                    let value = cpu.bus_mut().load_8(data[0], false);
                    assert_eq!(value, data[1] as u8, "Final test data load mismatch for test {}", test.name)
                }
            }
        }
    }
}
