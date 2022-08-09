use std::sync::mpsc::Sender;

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Sample, SampleFormat, Stream, StreamConfig, SupportedStreamConfig};

fn init_device() -> Option<Device> {
    let host = cpal::default_host();
    host.default_input_device()
}

fn get_stream_config(device: &Device) -> Option<SupportedStreamConfig> {
    let config_range = device.supported_input_configs();
    if let Ok(mut configs) = config_range {
        configs.next().map(|config| config.with_max_sample_rate())
    } else {
        None
    }
}

pub struct AudioCapture<T: Sample> {
    device: Device,
    config: StreamConfig,
    tx: Sender<Vec<T>>,
}
impl<T: Sample + Send + 'static> AudioCapture<T> {
    fn new(device: Device, config: StreamConfig, tx: Sender<Vec<T>>) -> Self {
        Self { device, config, tx }
    }
    pub fn listen(self) -> Stream {
        self.device
            .build_input_stream(
                &self.config,
                move |data: &[T], _| {
                    if let Err(err) = self.tx.send(data.to_vec()) {
                        println!("{}", err);
                    };
                },
                |_| {
                    println!("err");
                },
            )
            .expect("Unable to create input stream")
    }
}

pub struct AudioCaptureBuilder {
    device: Device,
}
impl AudioCaptureBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init<T: Sample + Send + 'static>(self, tx: Sender<Vec<T>>) -> AudioCapture<T> {
        let supported_config =
            get_stream_config(&self.device).expect("No supported capture config");
        match supported_config.sample_format() {
            SampleFormat::I16 => AudioCapture::new(self.device, supported_config.config(), tx),
            SampleFormat::U16 => AudioCapture::new(self.device, supported_config.config(), tx),
            SampleFormat::F32 => AudioCapture::new(self.device, supported_config.config(), tx),
        }
    }
}

impl Default for AudioCaptureBuilder {
    fn default() -> Self {
        let device = init_device().expect("No capture device found");
        Self { device }
    }
}
