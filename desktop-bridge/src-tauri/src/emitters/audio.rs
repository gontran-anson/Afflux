use tauri::{AppHandle, Emitter};
use shared_types::AudioLevel;

pub fn emit_vumeter_levels(app: &AppHandle, levels: AudioLevel) {
  let _ = app.emit("vumeter-update", &levels);
}