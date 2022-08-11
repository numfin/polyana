use std::thread;
use std::time::Duration;

use audio_capture::AudioCapture;
use audio_playback::AudioPlayback;
use common::Msg;
use cpal::traits::StreamTrait;

fn main() -> Result<(), String> {
    let (tx, rx) = flume::bounded::<Msg>(0);

    // rx will be dropped without variable
    let _playback = AudioPlayback::new()?.play(rx)?;
    let capture = AudioCapture::new()?.listen(tx)?;
    capture.play().unwrap();

    loop {
        thread::sleep(Duration::from_secs(1))
    }
}
