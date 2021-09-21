use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Host, Stream};

pub fn create_output_stream(host: &Host, input_buffer: Arc<Mutex<Vec<f32>>>) -> Stream {
    let device = host.default_output_device().expect("No output device");
    let config = device
        .supported_output_configs()
        .expect("No supported output")
        .next()
        .expect("No supported output config")
        .with_max_sample_rate();

    device
        .build_output_stream(
            &config.into(),
            move |output_buffer: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let mut input_buffer = input_buffer.lock().unwrap();
                let input_iter = Rc::new(RefCell::new(input_buffer.iter()));
                for sample in output_buffer.iter_mut() {
                    *sample = match input_iter.borrow_mut().next() {
                        Some(&v) => v,
                        None => *input_buffer.last().unwrap_or(&0.0),
                    }
                }
                *input_buffer = vec![];
            },
            move |err| println!("{}", err),
        )
        .expect("Unable to open output stream")
}
