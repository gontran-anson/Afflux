use std::sync::Mutex;
use cpal::Stream;

pub struct AudioState {
    pub current_stream: Mutex<Option<Stream>>
}

impl Default for AudioState {
    fn default() -> Self {
        Self { current_stream: Mutex::new(None) }
    }
}