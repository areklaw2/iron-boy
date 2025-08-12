#[cfg(test)]
mod tests {
    use std::fs::{self};

    use ironboy_common::{GameBoyMode, memory::MemoryInterface};
    use serde::{Deserialize, Serialize};

    use crate::{Cpu, registers::Registers};

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
        fn load_8(&self, address: u16) -> u8 {
            self.data[address as usize]
        }

        fn store_8(&mut self, address: u16, value: u8) {
            self.data[address as usize] = value
        }

        fn cycle(&mut self, cycles: u32, _cpu_halted: bool) -> u32 {
            cycles
        }

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
    }

    #[test]
    fn single_step_tests() {
        // let mut file_count = 0;
        let directory = fs::read_dir("../external/sm83/v1").unwrap();
        for file in directory {
            // file_count += 1;
            let file = file.unwrap().path();
            // println!("file: {}", file.file_name().unwrap().to_str().unwrap());
            // println!("file_count: {}", file_count);

            let test_json = fs::read_to_string(file).expect("Unable to open file");
            let tests: Vec<Test> = serde_json::from_str(&test_json).unwrap();
            // let mut test_count = 0;
            for test in tests {
                // test_count += 1;
                // println!("test: {}", test.name);
                // println!("test_count: {}", test_count);

                let inital_state = test.initial;
                let final_state = test.r#final;

                let mut cpu = Cpu::new(SimpleBus::new(), Registers::new(GameBoyMode::Color));
                cpu.registers.pc = inital_state.pc;
                cpu.registers.sp = inital_state.sp;
                cpu.registers.a = inital_state.a;
                cpu.registers.b = inital_state.b;
                cpu.registers.c = inital_state.c;
                cpu.registers.d = inital_state.d;
                cpu.registers.e = inital_state.e;
                cpu.registers.f = inital_state.f.into();
                cpu.registers.h = inital_state.h;
                cpu.registers.l = inital_state.l;

                for data in inital_state.ram {
                    cpu.bus.store_8(data[0], data[1] as u8);
                    assert_eq!(cpu.bus.load_8(data[0]), data[1] as u8)
                }

                cpu.fetch_instruction();
                cpu.execute_instruction();

                assert_eq!(cpu.registers.pc, final_state.pc);
                assert_eq!(cpu.registers.sp, final_state.sp);
                assert_eq!(cpu.registers.a, final_state.a);
                assert_eq!(cpu.registers.b, final_state.b);
                assert_eq!(cpu.registers.c, final_state.c);
                assert_eq!(cpu.registers.d, final_state.d);
                assert_eq!(cpu.registers.e, final_state.e);
                assert_eq!(u8::from(&cpu.registers.f), final_state.f);
                assert_eq!(cpu.registers.h, final_state.h);
                assert_eq!(cpu.registers.l, final_state.l);

                for data in final_state.ram {
                    let value = cpu.bus.load_8(data[0]);
                    assert_eq!(value, data[1] as u8)
                }
            }
        }
    }
}
