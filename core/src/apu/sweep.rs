#[derive(Clone, Copy)]
pub struct Sweep {
    direction: Direction,
    shift: u8,
    duration: u32,
    counter: u32,
}

impl Sweep {
    pub fn write(data: u8) -> Sweep {
        let direction = match data & 0x08 == 0x08 {
            false => Direction::Up,
            true => Direction::Down,
        };
        let shift = data & 7;
        let length = ((data & 0x70) >> 4) as u32;

        Sweep {
            direction,
            shift: shift,
            duration: length * 0x8000,
            counter: 0,
        }
    }

    pub fn read(&self) -> u8 {
        let mut data = 0x80;
        data |= ((self.duration / 0x8000) as u8) << 4;
        data |= (self.direction as u8) << 3;
        data |= self.shift;
        data
    }

    pub fn step(&mut self, div: u16) -> Option<u16> {
        if self.duration == 0 {
            return Some(div);
        }

        self.counter += 1;
        self.counter %= self.duration;
        if self.counter != 0 {
            return Some(div);
        }

        let offset = div >> (self.shift as usize);
        match self.direction {
            Direction::Up => {
                let div = div + offset;
                if div > 0x7ff {
                    None
                } else {
                    Some(div)
                }
            }
            Direction::Down => {
                if self.shift == 0 || offset > div {
                    Some(div)
                } else {
                    Some(div - offset)
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up = 0,
    Down = 1,
}
