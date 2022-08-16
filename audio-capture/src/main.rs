use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use audio_capture::AudioCapture;
use audio_playback::AudioPlayback;
use common::Msg;
use cpal::traits::StreamTrait;

fn main() -> Result<(), String> {
    let (tx, rx) = flume::bounded::<Msg>(1);
    let is_paused = Arc::new(Mutex::new(false));

    // rx will be dropped without variable
    let _playback = AudioPlayback::new()?.play(rx)?;
    let capture = AudioCapture::new()?.listen(tx, is_paused.clone())?;
    capture.play().unwrap();

    loop {
        // *is_paused.lock().unwrap() = true;
        // thread::sleep(Duration::from_secs(1));
        *is_paused.lock().unwrap() = false;
        thread::sleep(Duration::from_secs(5));
    }
}
