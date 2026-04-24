use audio_capture;
use shared_types::AudioDevice;

#[tauri::command]
pub async fn get_input_devices() -> Result<Vec<AudioDevice>, String> {
    audio_capture::get_input_devices()
        .map_err(|e| e.to_string())
}