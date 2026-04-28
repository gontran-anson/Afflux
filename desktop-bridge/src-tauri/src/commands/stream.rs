use audio_capture;
use shared_types::AudioLevel;
use tauri::{AppHandle, State};

use crate::state::AudioState;
use crate::emitters::audio::emit_vumeter_levels;

#[tauri::command]
pub async fn start_stream(device_name: String, state: State<'_, AudioState>, app_handler: AppHandle) -> Result<(), String> {
    let handle = app_handler.clone();

    let on_data = move |samples: &[f32]| {
        let sum_sq: f32 = samples.iter().map(|&s| s * s).sum();
        let rms = (sum_sq / samples.len() as f32).sqrt();

        let levels = AudioLevel {
            left_peak: rms,
            right_peak: rms,
            rms
        };

        emit_vumeter_levels(&handle, levels);
    };

    let stream = audio_capture::start_input_stream(&device_name, on_data)
        .map_err(|e| e.to_string())?;

    let mut current_stream = state.current_stream.lock().unwrap();
    *current_stream = Some(stream);

    println!("Flux démarré sur : {}", device_name);

    Ok(())
}


#[tauri::command]
pub async fn stop_stream(state: State<'_, AudioState>) -> Result<(), String> {
    let mut current_stream = state.current_stream.lock().unwrap();

    *current_stream = None;

    print!("Flux audio arrêté proprement");
    
    Ok(())
}