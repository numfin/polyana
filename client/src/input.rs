use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Host, Stream, StreamConfig};
use ringbuf::Producer;

pub fn get_input_setup(host: &Host) -> anyhow::Result<(Device, StreamConfig)> {
    let device = host.default_input_device().expect("No default device");
    let config = device.default_input_config()?.into();
    Ok((device, config))
}

pub fn create_input_stream(
    (device, config): (&Device, &StreamConfig),
    mut producer: Producer<f32>,
) -> anyhow::Result<Stream> {
    let stream = device.build_input_stream(
        config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            for sample in data {
                producer.push(*sample).unwrap();
            }
        },
        move |err| println!("{}", err),
    )?;
    Ok(stream)
}
