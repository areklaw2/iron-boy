use blip_buf::BlipBuf;
use pulse::PulseChannel;

use crate::{bus::Memory, io::audio_player::AudioPlayer};

mod length_timer;
mod pulse;
mod volume_envelope;

pub const WAVE_PATTERN: [[i32; 8]; 4] = [
    [-1, -1, -1, -1, 1, -1, -1, -1],
    [-1, -1, -1, -1, 1, 1, -1, -1],
    [-1, -1, 1, 1, 1, 1, -1, -1],
    [1, 1, 1, 1, -1, -1, 1, 1],
];
const CLOCKS_PER_SECOND: u32 = 4194304;
const CLOCKS_PER_FRAME: u32 = CLOCKS_PER_SECOND / 512;
const OUTPUT_SAMPLE_COUNT: usize = 2000;

pub trait ChannelMemory {
    fn mem_read(&mut self, address: u16) -> u8;

    fn mem_write(&mut self, address: u16, data: u8, frame_step: u8);
}

pub struct Apu {
    enabled: bool,
    time: u32,
    prev_time: u32,
    next_time: u32,
    frame_step: u8,
    output_period: u32,
    channel1: PulseChannel,
    channel2: PulseChannel,
    volume_left: u8,
    volume_right: u8,
    vin_left: u8,
    vin_right: u8,
    sound_panning: u8,
    need_sync: bool,
    audio_player: Box<dyn AudioPlayer>,
}

impl Memory for Apu {
    fn mem_read(&mut self, address: u16) -> u8 {
        self.run();
        match address {
            0xFF10..=0xFF14 => self.channel1.mem_read(address),
            0xFF16..=0xFF19 => self.channel2.mem_read(address),
            0xFF1A..=0xFF1E => 0xFF,
            0xFF20..=0xFF23 => 0xFF,
            0xFF24 => {
                let mut data = 0;
                data |= (self.vin_left) << 7;
                data |= (self.volume_left & 0x07) << 4;
                data |= (self.vin_right) << 3;
                data |= self.volume_right & 0x07;
                data
            }
            0xFF25 => self.sound_panning,
            0xFF26 => {
                let mut data = 0;
                data |= (self.enabled as u8) << 7;
                data |= 0x70;
                //data |= (self.channel4.on() as u8) << 3;
                //data |= (self.channel3.on() as u8) << 2;
                data |= (self.channel2.on() as u8) << 1;
                data |= self.channel1.on() as u8;
                data
            }
            0xFF30..=0xFF3F => 0xFF,
            _ => 0xFF,
        }
    }

    fn mem_write(&mut self, address: u16, data: u8) {
        if !self.enabled {
            match address {
                0xFF11 => self.channel1.mem_write(address, data & 0x3F, self.frame_step),
                0xFF16 => self.channel2.mem_write(address, data & 0x3F, self.frame_step),
                // 0xFF1B => self.channel3.mem_write(address, data, self.frame_step),
                // 0xFF20 => self.channel4.mem_write(address, data & 0x3F, self.frame_step),
                _ => (),
            }

            if address != 0xFF26 {
                return;
            }
        }

        self.run();
        match address {
            0xFF10..=0xFF14 => self.channel1.mem_write(address, data, self.frame_step),
            0xFF16..=0xFF19 => self.channel2.mem_write(address, data, self.frame_step),
            0xFF1A..=0xFF1E => {}
            0xFF20..=0xFF23 => {}
            0xFF24 => {
                self.vin_left = data & 0x80;
                self.volume_left = (data & 0x70) >> 4;
                self.vin_right = data & 0x08;
                self.volume_right = data & 0x07;
            }
            0xFF25 => self.sound_panning = data,
            0xFF26 => {
                let enable = data & 0x80 == 0x80;
                if self.enabled && !enable {
                    for i in 0xFF10..=0xFF25 {
                        self.mem_write(i, 0);
                    }
                }
                if !self.enabled && enable {
                    self.frame_step = 0;
                }
                self.enabled = enable;
            }
            0xFF30..=0xFF3F => {}
            _ => {}
        }
    }
}

impl Apu {
    pub fn new(audio_player: Box<dyn AudioPlayer>) -> Self {
        let blip_buffer1 = create_blip_buffer(audio_player.samples_rate());
        let blip_buffer2 = create_blip_buffer(audio_player.samples_rate());

        let output_period = (OUTPUT_SAMPLE_COUNT as u64 * CLOCKS_PER_SECOND as u64) / audio_player.samples_rate() as u64;

        Apu {
            enabled: false,
            time: 0,
            prev_time: 0,
            next_time: CLOCKS_PER_SECOND,
            frame_step: 0,
            output_period: output_period as u32,
            channel1: PulseChannel::new(blip_buffer1, true),
            channel2: PulseChannel::new(blip_buffer2, false),

            volume_left: 7,
            volume_right: 7,
            vin_left: 0,
            vin_right: 0,
            sound_panning: 0,
            need_sync: false,
            audio_player: audio_player,
        }
    }

    pub fn cycle(&mut self, ticks: u32) {
        if !self.enabled {
            return;
        }

        self.time += ticks;

        if self.time >= self.output_period {
            self.calculate_output();
        }
    }

    pub fn sync(&mut self) {
        self.need_sync = true;
    }

    fn calculate_output(&mut self) {
        self.run();
        debug_assert!(self.time == self.prev_time);
        self.channel1.blip_buffer.end_frame(self.time);
        self.channel2.blip_buffer.end_frame(self.time);
        // self.channel3.blip.end_frame(self.time);
        // self.channel4.blip.end_frame(self.time);
        self.next_time -= self.time;
        self.time = 0;
        self.prev_time = 0;

        if !self.need_sync || self.audio_player.underflowed() {
            self.need_sync = false;
            self.mix_buffers();
        } else {
            self.clear_buffers();
        }
    }

    fn run(&mut self) {
        while self.next_time <= self.time {
            self.channel1.run(self.prev_time, self.next_time);
            self.channel2.run(self.prev_time, self.next_time);
            // self.channel3.run(self.prev_time, self.next_time);
            // self.channel4.run(self.prev_time, self.next_time);

            if self.frame_step % 2 == 0 {
                self.channel1.step_length();
                self.channel2.step_length();
                // self.channel3.step_length();
                // self.channel4.step_length();
            }
            if self.frame_step % 4 == 2 {
                self.channel1.step_sweep();
            }
            if self.frame_step == 7 {
                self.channel1.volume_envelope.step();
                self.channel2.volume_envelope.step();
                // self.channel4.volume_envelope.step();
            }

            self.frame_step = (self.frame_step + 1) % 8;

            self.prev_time = self.next_time;
            self.next_time += CLOCKS_PER_FRAME;
        }

        if self.prev_time != self.time {
            self.channel1.run(self.prev_time, self.time);
            self.channel2.run(self.prev_time, self.time);
            // self.channel3.run(self.prev_time, self.time);
            // self.channel4.run(self.prev_time, self.time);

            self.prev_time = self.time;
        }
    }

    fn mix_buffers(&mut self) {
        let sample_count = self.channel1.blip_buffer.samples_avail() as usize;
        debug_assert!(sample_count == self.channel2.blip_buffer.samples_avail() as usize);
        // debug_assert!(sample_count == self.channel3.blip.samples_avail() as usize);
        // debug_assert!(sample_count == self.channel4.blip.samples_avail() as usize);

        let mut output = 0;

        let left_vol = (self.volume_left as f32 / 7.0) * (1.0 / 15.0) * 0.25;
        let right_vol = (self.volume_right as f32 / 7.0) * (1.0 / 15.0) * 0.25;

        while output < sample_count {
            let buf_left = &mut [0f32; OUTPUT_SAMPLE_COUNT + 10];
            let buf_right = &mut [0f32; OUTPUT_SAMPLE_COUNT + 10];
            let buf = &mut [0i16; OUTPUT_SAMPLE_COUNT + 10];

            let count1 = self.channel1.blip_buffer.read_samples(buf, false);
            for (i, v) in buf[..count1].iter().enumerate() {
                if self.sound_panning & 0x10 == 0x10 {
                    buf_left[i] += *v as f32 * left_vol;
                }
                if self.sound_panning & 0x01 == 0x01 {
                    buf_right[i] += *v as f32 * right_vol;
                }
            }

            let count2 = self.channel2.blip_buffer.read_samples(buf, false);
            for (i, v) in buf[..count2].iter().enumerate() {
                if self.sound_panning & 0x20 == 0x20 {
                    buf_left[i] += *v as f32 * left_vol;
                }
                if self.sound_panning & 0x02 == 0x02 {
                    buf_right[i] += *v as f32 * right_vol;
                }
            }

            // channel3 is the WaveChannel, that outputs samples with a 4x
            // increase in amplitude in order to avoid a loss of precision.
            // let count3 = self.channel3.blip.read_samples(buf, false);
            // for (i, v) in buf[..count3].iter().enumerate() {
            //     if self.sound_panning & 0x40 == 0x40 {
            //         buf_left[i] += ((*v as f32) / 4.0) * left_vol;
            //     }
            //     if self.sound_panning & 0x04 == 0x04 {
            //         buf_right[i] += ((*v as f32) / 4.0) * right_vol;
            //     }
            // }

            // let count4 = self.channel4.blip.read_samples(buf, false);
            // for (i, v) in buf[..count4].iter().enumerate() {
            //     if self.sound_panning & 0x80 == 0x80 {
            //         buf_left[i] += *v as f32 * left_vol;
            //     }
            //     if self.sound_panning & 0x08 == 0x08 {
            //         buf_right[i] += *v as f32 * right_vol;
            //     }
            // }

            debug_assert!(count1 == count2);
            // debug_assert!(count1 == count3);
            // debug_assert!(count1 == count4);

            self.audio_player.play(&buf_left[..count1], &buf_right[..count1]);

            output += count1;
        }
    }

    fn clear_buffers(&mut self) {
        self.channel1.blip_buffer.clear();
        self.channel2.blip_buffer.clear();
        // self.channel3.blip.clear();
        // self.channel4.blip.clear();
    }
}

fn create_blip_buffer(samples_rate: u32) -> BlipBuf {
    let mut blip_buffer = BlipBuf::new(samples_rate);
    blip_buffer.set_rates(CLOCKS_PER_SECOND as f64, samples_rate as f64);
    blip_buffer
}
