use std::convert::TryFrom;
use std::thread::{self, JoinHandle};

use audiopus::coder::{Decoder, Encoder};
use audiopus::packet::Packet;
use audiopus::*;
use ringbuf::{Consumer, Producer};

use crate::config::{AUDIO_TIME_STEP, FRAME_SIZE_PER_STEP, SAMPLE_RATE};

pub fn create_encoder() -> anyhow::Result<Encoder> {
    let mut encoder = Encoder::new(
        SampleRate::try_from(SAMPLE_RATE)?,
        Channels::Mono,
        Application::Voip,
    )?;
    encoder.set_bitrate(Bitrate::Auto)?;
    Ok(encoder)
}
pub fn create_decoder() -> anyhow::Result<Decoder> {
    let decoder = Decoder::new(SampleRate::try_from(SAMPLE_RATE)?, Channels::Mono)?;
    Ok(decoder)
}

pub fn encode_input(
    mut input_consumer: Consumer<f32>,
    mut frames_producer: Producer<Vec<u8>>,
) -> JoinHandle<anyhow::Result<()>> {
    thread::spawn(move || -> anyhow::Result<()> {
        let encoder = create_encoder()?;

        loop {
            while input_consumer.len() > SAMPLE_RATE as usize {
                let mut input_buffer = vec![];
                for _ in 0..FRAME_SIZE_PER_STEP {
                    match input_consumer.pop() {
                        Some(v) => input_buffer.push(v),
                        None => input_buffer.push(0.0),
                    };
                }
                let mut output_buffer = vec![0; FRAME_SIZE_PER_STEP];
                encoder.encode_float(&input_buffer, &mut output_buffer)?;
                if frames_producer.push(output_buffer).is_err() {
                    eprint!("?");
                }
            }
            thread::sleep(AUDIO_TIME_STEP);
        }
    })
}

pub fn decode_frames(
    mut frames_consumer: Consumer<Vec<u8>>,
    mut output_producer: Producer<f32>,
) -> JoinHandle<anyhow::Result<()>> {
    thread::spawn(move || -> anyhow::Result<()> {
        let mut decoder = create_decoder()?;

        loop {
            while let Some(frame) = frames_consumer.pop() {
                let mut output_buffer = vec![0.0; FRAME_SIZE_PER_STEP];
                let output_signals: MutSignals<f32> = MutSignals::try_from(&mut output_buffer)?;
                let frame = Packet::try_from(&frame)?;
                decoder
                    .decode_float(Some(frame), output_signals, false)
                    .unwrap();
                output_producer.push_slice(&output_buffer);
            }
            thread::sleep(AUDIO_TIME_STEP);
        }
    })
}
