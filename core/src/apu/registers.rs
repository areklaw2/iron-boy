use bitflags::bitflags;

bitflags! {
    #[derive(Debug)]
    pub struct AudioMasterControl: u8 {
        const AudioEnabled = 0b1000_0000;
        const Ch4Enabled = 0b0000_1000;
        const Ch3Enabled = 0b0000_0100;
        const Ch2Enabled = 0b0000_0010;
        const Ch1Enabled = 0b0000_0001;
    }

    #[derive(Debug)]
    pub struct SoundPanning: u8 {
        const Ch4Left = 0b1000_0000;
        const Ch3Left = 0b0100_0000;
        const Ch2Left = 0b0010_0000;
        const Ch1Left = 0b0001_0000;
        const Ch4Right = 0b0000_1000;
        const Ch3Right = 0b0000_0100;
        const Ch2Right = 0b0000_0010;
        const Ch1Right = 0b0000_0001;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Volume {
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
}

impl Volume {
    pub fn get_volume(value: u8) -> Volume {
        match value {
            0b000 => Volume::Level1,
            0b001 => Volume::Level2,
            0b010 => Volume::Level3,
            0b011 => Volume::Level4,
            0b100 => Volume::Level5,
            0b101 => Volume::Level6,
            0b110 => Volume::Level7,
            0b111 => Volume::Level8,
            _ => panic!("Invalid value was passed"),
        }
    }
}

pub struct MasterVolumeAndVinPanning {
    vin_left: bool, // will need to access these methods or pub??
    left_volume: Volume,
    vin_right: bool,
    right_volume: Volume,
}

impl MasterVolumeAndVinPanning {
    pub fn reg_write(data: u8) -> MasterVolumeAndVinPanning {
        MasterVolumeAndVinPanning {
            vin_left: data & 0x80 == 0x80,
            left_volume: Volume::get_volume((data & 0b0111_0000) >> 4),
            vin_right: data & 0x08 == 0x08,
            right_volume: Volume::get_volume(data & 0b0000_0111),
        }
    }

    pub fn reg_read(&self) -> u8 {
        let mut data = 0;

        data |= (self.vin_left as u8) << 7;
        data |= (self.left_volume as u8) << 4;
        data |= (self.right_volume as u8) << 3;
        data |= self.right_volume as u8;

        data
    }
}
