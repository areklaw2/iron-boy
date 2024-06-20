#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up = 0,
    Down = 1,
}

#[derive(Clone, Copy)]
pub struct Sweep {
    direction: Direction,
    shift: u8,
    duration: u32,
    counter: u32,
}

impl Sweep {
    pub fn write(val: u8) -> Sweep {
        let direction = match val & 0x08 == 0x80 {
            false => Direction::Up,
            true => Direction::Down,
        };

        let shift = val & 0x07;
        let length = ((val & 0x70) >> 4) as u32;

        Sweep {
            direction,
            shift: shift,
            duration: length * 0x8000,
            counter: 0,
        }
    }

    pub fn read(&self) -> u8 {
        let l = (self.duration / 0x8000) as u8;
        let dir = self.direction as u8;
        (1 << 7) | (l << 4) | (dir << 3) | self.shift
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

                if div > 0x07FF {
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
