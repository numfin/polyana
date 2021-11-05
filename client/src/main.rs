use std::thread;
use std::time::Duration;

use cpal::traits::StreamTrait;
use input::get_input_setup;
use output::get_output_setup;
use ringbuf::RingBuffer;

use crate::input::create_input_stream;
use crate::output::create_output_stream;

mod input;
mod output;

fn main() -> anyhow::Result<()> {
    let host = cpal::default_host();

    let (device, config) = get_output_setup(&host)?;

    let latency = 150.0;
    let latency_frames = (latency / 1_000.0) * config.sample_rate.0 as f32;
    let latency_samples = latency_frames as usize * config.channels as usize;
    let ring: RingBuffer<f32> = ringbuf::RingBuffer::new(latency_samples * 2);

    let (mut producer, consumer) = ring.split();
    for _ in 0..latency_samples {
        producer.push(0.0).unwrap()
    }

    let output = create_output_stream((&device, &config), consumer);
    let (device, config) = get_input_setup(&host)?;
    let input = create_input_stream((&device, &config), producer)?;

    output.play().unwrap();
    input.play().unwrap();

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
