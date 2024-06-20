#[derive(Clone, Copy)]
pub struct Lfsr {
    register: u16,
    width: Width,
    duration: u32,
    counter: u32,
    data: u8,
}

#[derive(Clone, Copy)]
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

        let mut length = match data & 0x07 {
            0 => 8 / 2,
            n => 8 * n as u32,
        };

        length *= 1 << ((data >> 4) + 1) as usize;
        Lfsr {
            register,
            width,
            duration: length,
            counter: 0,
            data,
        }
    }

    pub fn read(&self) -> u8 {
        self.data
    }

    pub fn step(&mut self) {
        self.counter += 1;
        self.counter %= self.duration;
        if self.counter == 0 {
            self.shift();
        }
    }

    pub fn high(&self) -> bool {
        self.register & 1 != 0
    }

    fn shift(&mut self) {
        let shifted = self.register >> 1;
        let carry = (self.register ^ shifted) & 1;
        self.register = match self.width {
            Width::Lfsr7bit => shifted | (carry << 6),
            Width::Lfsr15bit => shifted | (carry << 14),
        };
    }
}
