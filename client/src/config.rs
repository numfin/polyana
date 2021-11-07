use std::time::Duration;

pub const SAMPLE_RATE: i32 = 48000;
pub const AUDIO_FRAME_RATE: u64 = 50;
pub const AUDIO_TIME_STEP: Duration = Duration::from_millis(1000 / AUDIO_FRAME_RATE);
pub const CHUNKS_PER_SECOND: u64 = 1000 / (1000 / AUDIO_FRAME_RATE);
pub const FRAME_SIZE_PER_STEP: usize = SAMPLE_RATE as usize / AUDIO_FRAME_RATE as usize;
