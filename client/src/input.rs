use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Host, SampleFormat, SampleRate, Stream};

pub fn create_input_stream(host: &Host, input_buffer: Arc<Mutex<Vec<f32>>>) -> Stream {
    let device = host.default_input_device().expect("No input device");
    let mut supported_configs = device
        .supported_input_configs()
        .expect("No supported input");
    let config_range = supported_configs
        .find(|c| c.channels() == 1 && c.sample_format() == SampleFormat::F32)
        .expect("Supported config not found");
    let config = config_range.with_max_sample_rate();

    device
        .build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut input_buffer = input_buffer.lock().unwrap();
                *input_buffer = vec![];

                for sample in data {
                    input_buffer.push(*sample);
                }
            },
            move |err| println!("{}", err),
        )
        .expect("Unable to open input stream")
}
