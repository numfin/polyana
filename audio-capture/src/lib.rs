use common::{Msg, SampleSize};
use flume::Sender;

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Sample, SampleFormat, Stream, SupportedStreamConfig};

pub struct AudioCapture {
    device: Device,
    supported_config: SupportedStreamConfig,
}
impl AudioCapture {
    pub fn new() -> Result<Self, String> {
        let (supported_config, device) = AudioCapture::get_stream_config()?;
        Ok(Self {
            device,
            supported_config,
        })
    }
    fn get_device() -> Result<Device, String> {
        let host = cpal::default_host();
        host.default_input_device()
            .ok_or_else(|| "No output device".into())
    }

    fn get_stream_config() -> Result<(SupportedStreamConfig, Device), String> {
        let device = AudioCapture::get_device()?;
        let config_range = device.supported_input_configs();
        let config_range = config_range
            .map_err(|_| "No config range".to_string())?
            .next()
            .ok_or_else(|| "No supported stream configs".to_string())?;
        let supported_config = config_range.with_max_sample_rate();

        Ok((supported_config, device))
    }
    pub fn listen(self, tx: Sender<Msg>) -> Result<Stream, String> {
        match self.supported_config.sample_format() {
            cpal::SampleFormat::I16 => self.listen_type::<i16>(tx),
            cpal::SampleFormat::U16 => self.listen_type::<u16>(tx),
            cpal::SampleFormat::F32 => self.listen_type::<f32>(tx),
        }
        .map_err(|err| err.to_string())
    }
    fn listen_type<T: Sample + Send + Sync + 'static>(
        self,
        tx: Sender<Msg>,
    ) -> Result<Stream, cpal::BuildStreamError> {
        self.device.build_input_stream(
            &self.supported_config.config(),
            move |data: &[T], _| {
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
