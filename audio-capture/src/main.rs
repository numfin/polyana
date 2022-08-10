use std::thread;
use std::time::Duration;

use audio_capture::AudioCaptureBuilder;
use audio_playback::AudioPlaybackBuilder;
use cpal::traits::StreamTrait;

fn main() {
    let (tx, rx) = flume::bounded(0);

    // If variable not used - rx will be dropped
    let _playback = AudioPlaybackBuilder::new().init(rx).play();
    let capture = AudioCaptureBuilder::new().init::<i16>(tx).listen();
    capture.play().unwrap();

    loop {
        thread::sleep(Duration::from_secs(1))
    }
}
