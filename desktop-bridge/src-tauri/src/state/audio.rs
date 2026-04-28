use std::sync::Mutex;
use cpal::Stream;
use audio_processing::VumeterProcessor;

pub struct AudioState {
    pub current_stream: Mutex<Option<Stream>>,
    pub vumeter: Mutex<VumeterProcessor>,
}

impl Default for AudioState {
    fn default() -> Self {
        Self { 
            current_stream: Mutex::new(None),
            vumeter: Mutex::new(VumeterProcessor::new(0.85)) 
        }
    }
}