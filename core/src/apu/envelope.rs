use super::{Sample, SOUND_MAX};

#[derive(Debug, Clone, Copy)]
struct Volume(u8);

impl Volume {
    fn write(data: u8) -> Volume {
        if data > SOUND_MAX {
            panic!("Out of range: {}", data);
        }
        Volume(data)
    }

    fn read(self) -> u8 {
        let Volume(data) = self;
        data
    }

    fn read_as_sample(self) -> Sample {
        let Volume(data) = self;
        data as Sample
    }

    fn up(&mut self) {
        let Volume(data) = *self;
        if data < SOUND_MAX {
            *self = Volume(data + 1);
        }
    }

    fn down(&mut self) {
        let Volume(data) = *self;
        if data > 0 {
            *self = Volume(data - 1);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up = 1,
    Down = 0,
}

#[derive(Debug, Clone, Copy)]
pub struct Envelope {
    direction: Direction,
    volume: Volume,
    duration: u32,
    counter: u32,
}

impl Envelope {
    pub fn write(data: u8) -> Envelope {
        let volume = Volume::write(data >> 4);
        let direction = match data & 0x08 == 0x08 {
            true => Direction::Up,
            false => Direction::Down,
        };
        let length = (data & 0x07) as u32;

        Envelope {
            direction: direction,
            volume: volume,
            duration: length * 0x10000,
            counter: 0,
        }
    }

    pub fn read(&self) -> u8 {
        let mut data = 0;
        data |= self.volume.read() << 4;
        data |= (self.direction as u8) << 3;
        data |= (self.duration / 0x10000) as u8;
        data
    }

    pub fn step(&mut self) {
        if self.duration == 0 {
            return;
        }

        self.counter += 1;
        self.counter %= self.duration;
        if self.counter == 0 {
            match self.direction {
                Direction::Up => self.volume.up(),
                Direction::Down => self.volume.down(),
            }
        }
    }

    pub fn read_as_sample(&self) -> Sample {
        self.volume.read_as_sample()
    }

    pub fn dac_enabled(&self) -> bool {
        self.direction != Direction::Down || self.volume.read_as_sample() != 0
    }
}
