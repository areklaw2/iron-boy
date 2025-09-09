use core::{GbSpeed, memory::MemoryInterface};

use serde::{Deserialize, Serialize};

pub struct SimpleBus {
    data: Vec<u8>,
}

impl SimpleBus {
    #[allow(dead_code)]
    pub fn new() -> SimpleBus {
        SimpleBus { data: vec![0; 0x10000] }
    }
}

impl MemoryInterface for SimpleBus {
    fn load_8(&mut self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn store_8(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value
    }

    fn cycle(&mut self) {}

    fn change_speed(&mut self) {}

    fn speed(&self) -> GbSpeed {
        GbSpeed::Normal
    }
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
    use core::{GbMode, cpu::Cpu};
    use std::fs;

    use super::*;

    #[test]
    fn single_step_tests() {
        let mut file_count = 0;
        let directory = fs::read_dir("../../external/sm83/v1").unwrap();

        // This is temporary remove when tests are passing
        let mut files: Vec<_> = directory.collect::<Result<Vec<_>, _>>().unwrap();
        files.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

        for file in files {
            file_count += 1;
            let file = file.path();
            println!("file: {}", file.file_name().unwrap().to_str().unwrap());
            println!("file_count: {}", file_count);

            let test_json = fs::read_to_string(file).expect("Unable to open file");
            let tests: Vec<Test> = serde_json::from_str(&test_json).unwrap();
            let mut test_count = 0;
            for test in tests {
                test_count += 1;
                println!("test: {}", test.name);
                println!("test_count: {}", test_count);

                let inital_state = test.initial;
                let final_state = test.r#final;

                let mut cpu = Cpu::new(SimpleBus::new(), GbMode::Color);
                cpu.registers().set_pc(inital_state.pc);
                cpu.registers().set_sp(inital_state.sp);
                cpu.registers().set_a(inital_state.a);
                cpu.registers().set_b(inital_state.b);
                cpu.registers().set_c(inital_state.c);
                cpu.registers().set_d(inital_state.d);
                cpu.registers().set_e(inital_state.e);
                cpu.registers().set_f(inital_state.f.into());
                cpu.registers().set_h(inital_state.h);
                cpu.registers().set_l(inital_state.l);

                for data in inital_state.ram {
                    cpu.bus_mut().store_8(data[0], data[1] as u8);
                    assert_eq!(cpu.bus_mut().load_8(data[0]), data[1] as u8)
                }

                cpu.fetch_next_instruction();
                cpu.execute_instruction();

                assert_eq!(cpu.registers().pc(), final_state.pc);
                assert_eq!(cpu.registers().sp(), final_state.sp);
                assert_eq!(cpu.registers().a(), final_state.a);
                assert_eq!(cpu.registers().b(), final_state.b);
                assert_eq!(cpu.registers().c(), final_state.c);
                assert_eq!(cpu.registers().d(), final_state.d);
                assert_eq!(cpu.registers().e(), final_state.e);
                assert_eq!(u8::from(&cpu.registers().f().clone()), final_state.f);
                assert_eq!(cpu.registers().h(), final_state.h);
                assert_eq!(cpu.registers().l(), final_state.l);

                for data in final_state.ram {
                    let value = cpu.bus_mut().load_8(data[0]);
                    assert_eq!(value, data[1] as u8)
                }
            }
        }
    }
}
