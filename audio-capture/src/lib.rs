use std::sync::{Arc, Mutex};

use common::{cpal, flume, Msg, SampleSize};
use flume::Sender;

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Sample, SampleFormat, Stream, SupportedStreamConfig};

#[derive(Debug)]
pub enum CaptureError {
    NoInputDevice,
    NoConfigRange,
    NoSupportedStreamConfigs,
}

pub struct AudioCapture {
    device: Device,
    supported_config: SupportedStreamConfig,
}
impl AudioCapture {
    pub fn new() -> Result<Self, CaptureError> {
        let (supported_config, device) = AudioCapture::get_stream_config()?;
        Ok(Self {
            device,
            supported_config,
        })
    }
    fn get_device() -> Result<Device, CaptureError> {
        let host = cpal::default_host();
        host.default_input_device()
            .ok_or(CaptureError::NoInputDevice)
    }

    fn get_stream_config() -> Result<(SupportedStreamConfig, Device), CaptureError> {
        let device = AudioCapture::get_device()?;
        let config_range = device.supported_input_configs();
        let config_range = config_range
            .map_err(|_| CaptureError::NoConfigRange)?
            .next()
            .ok_or(CaptureError::NoSupportedStreamConfigs)?;
        let supported_config = config_range.with_max_sample_rate();

        Ok((supported_config, device))
    }
    pub fn listen(self, tx: Sender<Msg>, is_paused: Arc<Mutex<bool>>) -> Result<Stream, String> {
        match self.supported_config.sample_format() {
            SampleFormat::I16 => self.listen_type::<i16>(tx, is_paused),
            SampleFormat::U16 => self.listen_type::<u16>(tx, is_paused),
            SampleFormat::F32 => self.listen_type::<f32>(tx, is_paused),
        }
        .map_err(|err| err.to_string())
    }
    fn listen_type<T: Sample + Send + Sync + 'static>(
        self,
        tx: Sender<Msg>,
        is_paused: Arc<Mutex<bool>>,
    ) -> Result<Stream, cpal::BuildStreamError> {
        self.device.build_input_stream(
            &self.supported_config.config(),
            move |data: &[T], _| {
                if *is_paused.lock().unwrap() {
                    return;
                }
                let input_samples = data
                    .iter()
                    .map(|sample| match self.supported_config.sample_format() {
                        SampleFormat::I16 => SampleSize::I16(sample.to_i16()),
                        SampleFormat::U16 => SampleSize::U16(sample.to_u16()),
                        SampleFormat::F32 => SampleSize::F32(sample.to_f32()),
                    })
                    .collect();
                if let Err(err) = tx.send(input_samples) {
                    println!("{}", err);
                };
            },
            |_| {
                println!("err");
            },
        )
    }
}
