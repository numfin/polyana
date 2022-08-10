use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Sample, SampleFormat, Stream, StreamConfig, SupportedStreamConfig};
use flume::Receiver;

fn init_device() -> Option<Device> {
    let host = cpal::default_host();
    host.default_output_device()
}

fn get_stream_config(device: &Device) -> Option<SupportedStreamConfig> {
    let config_range = device.supported_output_configs();
    if let Ok(mut configs) = config_range {
        configs.next().map(|config| config.with_max_sample_rate())
    } else {
        None
    }
}

pub struct AudioPlayback<T: Sample> {
    device: Device,
    config: StreamConfig,
    rx: Receiver<Vec<T>>,
}
impl<T: Sample + Send + 'static> AudioPlayback<T> {
    fn new(device: Device, config: StreamConfig, rx: Receiver<Vec<T>>) -> Self {
        Self { device, config, rx }
    }
    pub fn play(self) -> Stream {
        self.device
            .build_output_stream(
                &self.config,
                move |output: &mut [T], _| {
                    // self.rx
                    //     .try_iter()
                    //     .zip(output.chunks_mut(10000))
                    //     .for_each(|(input, output)| {
                    //         for (s, o) in output.iter_mut().zip(input) {
                    //             *s = o
                    //         }
                    //     });
                    if let Ok(rx_sample) = self.rx.try_recv() {
                        let mut iter = rx_sample.iter();
                        for o in output {
                            if let Some(i) = iter.next() {
                                *o = i.to_owned();
                            }
                        }
                        // output.iter_mut().zip(rx_sample).for_each(|(s, o)| {
                        //     *s = o;
                        // });
                    }
                },
                |_| {},
            )
            .expect("Unable to create output stream")
    }
}

pub struct AudioPlaybackBuilder {
    device: Device,
}
impl AudioPlaybackBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init<T: Sample + Send + 'static>(self, rx: Receiver<Vec<T>>) -> AudioPlayback<T> {
        let supported_config =
            get_stream_config(&self.device).expect("No supported playback config");
        match supported_config.sample_format() {
            SampleFormat::I16 => AudioPlayback::new(self.device, supported_config.config(), rx),
            SampleFormat::U16 => AudioPlayback::new(self.device, supported_config.config(), rx),
            SampleFormat::F32 => AudioPlayback::new(self.device, supported_config.config(), rx),
        }
    }
}

impl Default for AudioPlaybackBuilder {
    fn default() -> Self {
        let device = init_device().expect("No output device found");
        Self { device }
    }
}
