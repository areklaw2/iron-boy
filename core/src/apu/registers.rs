use bitflags::bitflags;

bitflags! {
    #[derive(Debug)]
    pub struct AudioMasterControl: u8 {
        const AudioOn = 0b1000_0000;
        const Ch4On = 0b0000_1000;
        const Ch3On = 0b0000_0100;
        const Ch2On = 0b0000_0010;
        const Ch1On = 0b0000_0001;
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

pub struct MasterVolumeAndVinPanning {
    vin_left: bool, // will need to access these methods or pub??
    left_volume: u8,
    vin_right: bool,
    right_volume: u8,
}

impl MasterVolumeAndVinPanning {
    pub fn reg_write(data: u8) -> MasterVolumeAndVinPanning {
        MasterVolumeAndVinPanning {
            vin_left: data & 0x80 == 0x80,
            left_volume: (data & 0b0111_0000) >> 4,
            vin_right: data & 0x08 == 0x08,
            right_volume: data & 0b0000_0111,
        }
    }

    pub fn reg_read(&self) -> u8 {
        let mut data = 0;

        data |= (self.vin_left as u8) << 7;
        data |= (self.left_volume as u8) << 4;
        data |= (self.vin_right as u8) << 3;
        data |= self.right_volume as u8;

        data
    }
}
