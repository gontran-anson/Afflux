use tauri::{AppHandle, State, Manager};

use audio_capture;
use shared_types::AudioLevel;

use crate::state::AudioState;
use crate::emitters::audio::emit_vumeter_levels;

#[tauri::command]
pub async fn start_stream(device_name: String, state: State<'_, AudioState>, app_handler: AppHandle) -> Result<(), String> {
    
    let handle = app_handler.clone();

    let on_data = move |samples: &[f32]| {
        let audio_state = handle.state::<AudioState>();
        let mut vumeter = audio_state.vumeter.lock().unwrap();
        let display_level = vumeter.compute_level(samples);

        let levels = AudioLevel {
            left_peak: display_level,
            right_peak: display_level,
            rms: display_level
        };

        emit_vumeter_levels(&handle, levels);
    };

    let stream = audio_capture::start_input_stream(&device_name, on_data)
        .map_err(|e| {
            println!(">>> ERREUR CAPTURE : {}", e);
            e.to_string()
        }
    )?;

    let mut current_stream = state.current_stream.lock().unwrap();
    *current_stream = Some(stream);

    Ok(())
}


#[tauri::command]
pub async fn stop_stream(state: State<'_, AudioState>) -> Result<(), String> {
    println!("Stop Stream !");

    let mut current_stream = state.current_stream.lock().unwrap();

    *current_stream = None;

    print!("Flux audio arrêté proprement");

    Ok(())
}