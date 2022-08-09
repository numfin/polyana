use std::thread;
use std::time::Duration;

// use audio_playback::init_output_stream;
// use cpal::traits::StreamTrait;

fn main() {
    // let stream = init_output_stream().expect("Unable to create output stream");
    // stream.play().expect("Unable to start output stream");

    loop {
        thread::sleep(Duration::from_secs(1))
    }
}
