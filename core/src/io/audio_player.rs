use std::{
    cmp::min,
    sync::{Arc, Mutex},
};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, Sample, SampleFormat, SampleRate, Stream, StreamConfig,
};

pub trait AudioPlayer: Send {
    fn play(&mut self, left_channel: &[f32], right_channel: &[f32]);
    fn samples_rate(&self) -> u32;
    fn underflowed(&self) -> bool;
}

pub struct NullAudioPlayer {}

impl AudioPlayer for NullAudioPlayer {
    fn play(&mut self, _buf_left: &[f32], _buf_right: &[f32]) {
        // Do nothing
    }

    fn samples_rate(&self) -> u32 {
        44100
    }

    fn underflowed(&self) -> bool {
        false
    }
}

pub struct CpalPlayer {
    buffer: Arc<Mutex<Vec<(f32, f32)>>>,
    sample_rate: u32,
}

impl CpalPlayer {
    pub fn get() -> Option<(CpalPlayer, Stream)> {
        let device = match cpal::default_host().default_output_device() {
            Some(device) => device,
            None => return None,
        };

        let sample_rate = SampleRate(44100);
        let supported_output_configs = match device.supported_output_configs() {
            Ok(configs) => configs,
            Err(_) => return None,
        };

        let mut supported_config = None;
        for config in supported_output_configs {
            if config.channels() == 2 && config.sample_format() == SampleFormat::F32 {
                if config.min_sample_rate() <= sample_rate && sample_rate <= config.max_sample_rate() {
                    supported_config = Some(config.with_sample_rate(sample_rate));
                } else {
                    supported_config = Some(config.with_max_sample_rate());
                }
                break;
            }
        }
        if supported_config.is_none() {
            return None;
        }

        let selected_config = supported_config.unwrap();
        let sample_format = selected_config.sample_format();
        let stream_config: StreamConfig = selected_config.into();

        let err_fn = |err| eprintln!("An error occurred on the output audio stream: {}", err);

        let shared_buffer = Arc::new(Mutex::new(Vec::new()));
        let stream_buffer = shared_buffer.clone();

        let player = CpalPlayer {
            buffer: shared_buffer,
            sample_rate: stream_config.sample_rate.0,
        };

        let stream = match sample_format {
            cpal::SampleFormat::I8 => device.build_output_stream(
                &stream_config,
                move |data: &mut [i8], _callback_info: &cpal::OutputCallbackInfo| cpal_thread(data, &stream_buffer),
                err_fn,
                None,
            ),
            cpal::SampleFormat::I16 => device.build_output_stream(
                &stream_config,
                move |data: &mut [i16], _callback_info: &cpal::OutputCallbackInfo| cpal_thread(data, &stream_buffer),
                err_fn,
                None,
            ),
            cpal::SampleFormat::I32 => device.build_output_stream(
                &stream_config,
                move |data: &mut [i32], _callback_info: &cpal::OutputCallbackInfo| cpal_thread(data, &stream_buffer),
                err_fn,
                None,
            ),
            cpal::SampleFormat::I64 => device.build_output_stream(
                &stream_config,
                move |data: &mut [i64], _callback_info: &cpal::OutputCallbackInfo| cpal_thread(data, &stream_buffer),
                err_fn,
                None,
            ),
            cpal::SampleFormat::U8 => device.build_output_stream(
                &stream_config,
                move |data: &mut [u8], _callback_info: &cpal::OutputCallbackInfo| cpal_thread(data, &stream_buffer),
                err_fn,
                None,
            ),
            cpal::SampleFormat::U16 => device.build_output_stream(
                &stream_config,
                move |data: &mut [u16], _callback_info: &cpal::OutputCallbackInfo| cpal_thread(data, &stream_buffer),
                err_fn,
                None,
            ),
            cpal::SampleFormat::U32 => device.build_output_stream(
                &stream_config,
                move |data: &mut [u32], _callback_info: &cpal::OutputCallbackInfo| cpal_thread(data, &stream_buffer),
                err_fn,
                None,
            ),
            cpal::SampleFormat::U64 => device.build_output_stream(
                &stream_config,
                move |data: &mut [u64], _callback_info: &cpal::OutputCallbackInfo| cpal_thread(data, &stream_buffer),
                err_fn,
                None,
            ),
            cpal::SampleFormat::F32 => device.build_output_stream(
                &stream_config,
                move |data: &mut [f32], _callback_info: &cpal::OutputCallbackInfo| cpal_thread(data, &stream_buffer),
                err_fn,
                None,
            ),
            cpal::SampleFormat::F64 => device.build_output_stream(
                &stream_config,
                move |data: &mut [f64], _callback_info: &cpal::OutputCallbackInfo| cpal_thread(data, &stream_buffer),
                err_fn,
                None,
            ),
            sf => panic!("Unsupported sample format {}", sf),
        }
        .unwrap();

        stream.play().unwrap();
        Some((player, stream))
    }
}

fn cpal_thread<T: Sample + FromSample<f32>>(output_buffer: &mut [T], audio_buffer: &Arc<Mutex<Vec<(f32, f32)>>>) {
    let mut input_buffer = audio_buffer.lock().unwrap();
    let output_length = min(output_buffer.len() / 2, input_buffer.len());
    for (i, (input_left, input_right)) in input_buffer.drain(..output_length).enumerate() {
        output_buffer[i * 2] = T::from_sample(input_left);
        output_buffer[i * 2 + 1] = T::from_sample(input_right);
    }
}

impl AudioPlayer for CpalPlayer {
    fn play(&mut self, left_channel: &[f32], right_channel: &[f32]) {
        debug_assert!(left_channel.len() == right_channel.len());
        let mut buffer = self.buffer.lock().unwrap();
        for (l, r) in left_channel.iter().zip(right_channel) {
            if buffer.len() > self.sample_rate as usize {
                return;
            }
            buffer.push((*l, *r));
        }
    }

    fn samples_rate(&self) -> u32 {
        self.sample_rate
    }

    fn underflowed(&self) -> bool {
        (*self.buffer.lock().unwrap()).len() == 0
    }
}
