use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Host, SampleRate, Stream, StreamConfig};
use ringbuf::Producer;

use crate::config::SAMPLE_RATE;

pub fn get_input_setup(host: &Host) -> anyhow::Result<(Device, StreamConfig)> {
    let device = host.default_input_device().expect("No default device");
    let config = device
        .supported_input_configs()
        .unwrap()
        .next()
        .unwrap()
        .with_sample_rate(SampleRate(SAMPLE_RATE as u32))
        .into();
    Ok((device, config))
}

pub fn create_input_stream(
    (device, config): (&Device, &StreamConfig),
    mut producer: Producer<f32>,
) -> anyhow::Result<Stream> {
    let stream = device.build_input_stream(
        config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut is_overflow = false;
            for sample in data {
                if producer.push(*sample).is_err() {
                    is_overflow = true
                }
            }
            if is_overflow {
                eprint!("!")
            }
        },
        move |err| println!("{}", err),
    )?;
    Ok(stream)
}
