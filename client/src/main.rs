use std::thread;
use std::time::Duration;

use config::{CHUNKS_PER_SECOND, SAMPLE_RATE};
use cpal::traits::StreamTrait;
use opus::{decode_frames, encode_input};
use ringbuf::RingBuffer;

use input::create_input_stream;
use input::get_input_setup;
use output::create_output_stream;
use output::get_output_setup;

mod config;
mod input;
mod opus;
mod output;

fn main() -> anyhow::Result<()> {
    let host = cpal::default_host();

    let (input_tx, input_rx) = RingBuffer::new(SAMPLE_RATE as usize * 10).split();
    let (frames_tx, frames_rx) = RingBuffer::new(CHUNKS_PER_SECOND as usize * 5).split();
    let (output_tx, output_rx) = RingBuffer::new(SAMPLE_RATE as usize * 10).split();
    encode_input(input_rx, frames_tx);
    decode_frames(frames_rx, output_tx);

    let (device, config) = get_output_setup(&host)?;
    let output = create_output_stream((&device, &config), output_rx);
    let (device, config) = get_input_setup(&host)?;
    let input = create_input_stream((&device, &config), input_tx)?;

    output.play().unwrap();
    input.play().unwrap();

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
