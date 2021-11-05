use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Host, Stream, StreamConfig};
use ringbuf::Consumer;

pub fn get_output_setup(host: &Host) -> anyhow::Result<(Device, StreamConfig)> {
    let device = host.default_output_device().expect("No output device");
    let config = device.default_output_config()?.into();
    Ok((device, config))
}

pub fn create_output_stream(
    (device, config): (&Device, &StreamConfig),
    mut consumer: Consumer<f32>,
) -> Stream {
    return device
        .build_output_stream(
            config,
            move |output_buffer: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for sample in output_buffer.iter_mut() {
                    *sample = match consumer.pop() {
                        Some(v) => v,
                        None => 0.0,
                    };
                }
            },
            move |err| println!("{}", err),
        )
        .expect("Unable to open output stream");
}
