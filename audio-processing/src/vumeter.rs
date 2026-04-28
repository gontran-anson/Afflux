pub struct VumeterProcessor {
	pub current_level: f32,
	pub decay_factor: f32,
}

impl VumeterProcessor {
	pub fn new(decay_factor: f32) -> Self {
		Self {
			current_level: 0.0,
			decay_factor,
		}
	}

	pub fn compute_level(&mut self, samples: &[f32]) -> f32 {
		let sum_sq : f32 = samples.iter().map(|&s| s * s).sum();
		let rms = (sum_sq / samples.len() as f32).sqrt();

		// dB conversion
		// Zero eviction by epsilon
		let db = 20.0 * rms.max(1e-5).log10();

		let min_db = -45.0; // -60.0

		// Normalization for UI (Scale : min_db to 0dB -> 0.0 to 1.0)
		let mut normalized = ((db - min_db) / (0.0 - min_db) ).clamp(0.0, 1.0);

		// Non linear curve
		normalized = normalized.powf(1.5);

		// fallof logic 
		if normalized > self.current_level {
			self.current_level = normalized;
		} else {
			self.current_level *= self.decay_factor; // Fluid descending
		}

		self.current_level
	}
}


pub struct VumeterState {
	pub current_level: f32,
	pub decay_factor: f32,
}

impl VumeterState {
	pub fn update(&mut self, new_sample_peak: f32) -> f32 {
		if new_sample_peak > self.current_level {
			self.current_level = new_sample_peak
		} else {

			self.current_level *= self.decay_factor
		}
		self.current_level
	}
}