#[derive(Debug, Clone, Copy)]
pub struct Lfsr {
    register: u16,
    width: Width,
    step_duration: u32,
    counter: u32,
    data: u8,
}

#[derive(Debug, Clone, Copy)]
enum Width {
    Lfsr15bit = 0,
    Lfsr7bit = 1,
}

impl Lfsr {
    pub fn write(data: u8) -> Lfsr {
        let (register, width) = match (data & 0x08) == 0x08 {
            true => ((1 << 7) - 1, Width::Lfsr7bit),
            false => ((1 << 15) - 1, Width::Lfsr15bit),
        };

        let mut clock_divder = match data & 0x07 {
            0 => 8 / 2,
            n => 8 * n as u32,
        };

        clock_divder *= 1 << ((data >> 4) + 1) as usize;

        Lfsr {
            register,
            width,
            step_duration: clock_divder,
            counter: 0,
            data,
        }
    }

    pub fn read(&self) -> u8 {
        self.data
    }

    pub fn step(&mut self) {
        self.counter += 1;
        self.counter %= self.step_duration;
        if self.counter == 0 {
            self.shift();
        }
    }

    pub fn high(&self) -> bool {
        self.register & 0x01 == 0x01
    }

    fn shift(&mut self) {
        let shifted = self.register >> 0x01;
        let carry = (self.register ^ shifted) & 0x01;
        self.register = match self.width {
            Width::Lfsr7bit => shifted | (carry << 6),
            Width::Lfsr15bit => shifted | (carry << 14),
        };
    }
}
