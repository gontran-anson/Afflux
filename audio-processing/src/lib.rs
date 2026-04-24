use shared_types::{AudioLevel, ProcessingConfig};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool};

pub struct AudioProcessor {
	config: Arc<ProcessingConfig>,
	is_running: Arc<AtomicBool>,
}

impl AudioProcessor {
	pub fn new(config: Arc<ProcessingConfig>) -> Self {
		Self {
			config,
			is_running: Arc::new(AtomicBool::new(true)),
		}
	}

	pub fn process_buffer(&self, buffer: &mut [f32]) -> AudioLevel {
		let mut max_peak = 0.0;
		let mut sum_sq = 0.0;

		for sample in buffer.iter_mut() {

			// Application du gain
			*sample *= self.config.master_gain;

			// Puis les effets (eq & compresseurs)
			// *sample = self.apply_effects(*sample)

			// Prepare vumetre
			let abs_sample = sample.abs();
			if abs_sample > max_peak { max_peak = abs_sample; }
			sum_sq += sample.powi(2);
		}

		let rms = (sum_sq /  buffer.len() as f32).sqrt();

		AudioLevel {
			left_peak: max_peak,
			right_peak: max_peak,
			rms,
		}
	} 
}