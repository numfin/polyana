use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

use cpal::traits::StreamTrait;

use crate::input::create_input_stream;
use crate::output::create_output_stream;

mod input;
mod output;

fn main() {
    // let (tx, rx) = mpsc::channel();
    let input_buffer = Arc::new(Mutex::new(vec![]));
    let host = cpal::default_host();
    let output = create_output_stream(&host, input_buffer.clone());
    let input = create_input_stream(&host, input_buffer.clone());

    output.play().unwrap();
    input.play().unwrap();

    loop {
        thread::sleep(Duration::from_secs(1));
    }
}
