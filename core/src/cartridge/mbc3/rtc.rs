use std::time;

pub struct RealTimeClock {
    registers: [u8; 5],
    latch_registers: [u8; 5],
    time: Option<u64>,
}

impl RealTimeClock {
    pub fn new(has_real_time_clock: bool) -> Self {
        let time = if has_real_time_clock { Some(0) } else { None };
        RealTimeClock {
            registers: [0u8; 5],
            latch_registers: [0u8; 5],
            time,
        }
    }

    pub fn set_register(&mut self, address: usize, value: u8) {
        self.registers[address] = value;
    }

    pub fn latch_register(&self, address: usize) -> u8 {
        self.registers[address]
    }

    pub fn set_latch_registers(&mut self) {
        self.set_registers();
        self.latch_registers.clone_from_slice(&self.registers);
    }

    pub fn set_registers(&mut self) {
        if self.registers[4] & 0x40 == 0x40 {
            return;
        }

        let current_time = match self.time {
            Some(t) => time::UNIX_EPOCH + time::Duration::from_secs(t),
            None => return,
        };

        if self.calculate_time() == self.time {
            return;
        }

        let time = match time::SystemTime::now().duration_since(current_time) {
            Ok(n) => n.as_secs(),
            _ => 0,
        };
        self.registers[0] = (time % 60) as u8;
        self.registers[1] = ((time / 60) % 60) as u8;
        self.registers[2] = ((time / 3600) % 24) as u8;
        let days = time / (3600 * 24);
        self.registers[3] = days as u8;
        self.registers[4] = (self.registers[4] & 0xFE) | (((days >> 8) & 0x01) as u8);
        if days >= 512 {
            self.registers[4] |= 0x80;
            self.set_time();
        }
    }

    pub fn set_time(&mut self) {
        self.time = self.calculate_time();
    }

    fn calculate_time(&self) -> Option<u64> {
        if self.time.is_none() {
            return None;
        }
        let mut time = match time::SystemTime::now().duration_since(time::UNIX_EPOCH) {
            Ok(t) => t.as_secs(),
            Err(_) => panic!("System clock is set to a time before the unix epoch (1970-01-01)"),
        };
        time -= self.registers[0] as u64;
        time -= (self.registers[1] as u64) * 60;
        time -= (self.registers[2] as u64) * 3600;
        let days = ((self.registers[4] as u64 & 0x1) << 8) | (self.registers[3] as u64);
        time -= days * 3600 * 24;
        Some(time)
    }

    pub fn time(&self) -> Option<u64> {
        self.time
    }

    pub fn load_time(&mut self, value: Option<u64>) {
        self.time = value;
    }
}
