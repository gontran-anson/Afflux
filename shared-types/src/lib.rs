use serde::{Serialize, Deserialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioFrame {
	pub samples: Vec<f32>,
	pub channels: u16,
	pub sample_rate: u32, 
}

#[derive(TS, Serialize, Deserialize)]
#[ts(export)]
pub struct AudioLevel {
	pub left_peak: f32,
	pub right_peak: f32,
	pub rms: f32,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingConfig {
	pub equalizer: EQConfig,
	pub compressor: ConpressorConfig,
	pub limiter_treshold: f32,
	pub master_gain: f32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub  struct EQConfig {
	pub low_gain: f32,
	pub mid_gain: f32,
	pub hight_gain: f32,
	pub enable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConpressorConfig {
	pub treshold: f32,
	pub ratio: f32,
	pub attack_ms: f32,
	pub release_ms: f32,
	pub enable: bool,
}
