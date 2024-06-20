use super::SOUND_MAX;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up = 1,
    Down = 0,
}

#[derive(Clone, Copy)]
struct Volume(u8);

impl Volume {
    fn write(volume: u8) -> Volume {
        if volume > SOUND_MAX {
            panic!("Volume out of range: {}", volume);
        }
        Volume(volume)
    }

    fn read(self) -> u8 {
        let Volume(data) = self;
        data
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

#[derive(Clone, Copy)]
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
            direction,
            volume,
            duration: length * 0x10000,
            counter: 0,
        }
    }

    pub fn read(&self) -> u8 {
        let volume = self.volume.read();
        let direction = self.direction as u8;
        let l = (self.duration / 0x10000) as u8;
        (volume << 4) | (direction << 3) | l
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

    pub fn read_volume(&self) -> u8 {
        self.volume.read()
    }

    pub fn dac_enabled(&self) -> bool {
        self.direction != Direction::Down || self.volume.read() != 0
    }
}
