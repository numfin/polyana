use common::{Msg, SampleSize};
use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, Sample, Stream, SupportedStreamConfig};
use flume::Receiver;

pub struct AudioPlayback {
    device: Device,
    supported_config: SupportedStreamConfig,
}
impl AudioPlayback {
    pub fn new() -> Result<Self, String> {
        let (supported_config, device) = AudioPlayback::get_stream_config()?;

        Ok(Self {
            device,
            supported_config,
        })
    }
    fn get_device() -> Result<Device, String> {
        let host = cpal::default_host();
        host.default_output_device()
            .ok_or_else(|| "No output device".into())
    }
    fn get_stream_config() -> Result<(SupportedStreamConfig, Device), String> {
        let device = AudioPlayback::get_device()?;
        let config_range = device.supported_output_configs();
        let config_range = config_range
            .map_err(|_| "No config range".to_string())?
            .next()
            .ok_or_else(|| "No supported stream configs".to_string())?;
        let supported_config = config_range.with_max_sample_rate();

        Ok((supported_config, device))
    }
    pub fn play(self, rx: Receiver<Msg>) -> Result<Stream, String> {
        match self.supported_config.sample_format() {
            cpal::SampleFormat::I16 => self.play_type::<i16>(rx),
            cpal::SampleFormat::U16 => self.play_type::<u16>(rx),
            cpal::SampleFormat::F32 => self.play_type::<f32>(rx),
        }
        .map_err(|err| err.to_string())
    }
    fn play_type<T: Sample + Send + Sync + 'static>(
        self,
        rx: Receiver<Msg>,
    ) -> Result<Stream, cpal::BuildStreamError> {
        self.device.build_output_stream(
            &self.supported_config.config(),
            move |buffer: &mut [T], _| {
                let samples = match rx.try_recv() {
                    Ok(packets) => packets.into_iter().map(|p| match p {
                        SampleSize::I16(v) => Sample::from(&v),
                        SampleSize::U16(v) => Sample::from(&v),
                        SampleSize::F32(v) => Sample::from(&v),
                    }),
                    _ => return,
                };
                samples.zip(buffer.iter_mut()).for_each(|(sample, buf)| {
                    *buf = sample;
                });
            },
            |_| {},
        )
    }
}
