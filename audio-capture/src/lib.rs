use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait};
use shared_types::AudioDevice;

pub fn get_input_devices() -> Result<Vec<AudioDevice>> {
    let host = cpal::default_host();
    let devices = host.input_devices().context("access devices impossible")?;

    let mut input_devices = Vec::new();

    for device in devices {
        let result = device.description();

        if let Ok(desc) = result {
            input_devices.push(AudioDevice { name: desc.name().to_string() });
        }
    }

    Ok(input_devices)
}

pub fn start_input_stream<F>(device_name: &str, mut on_data: F) -> Result<cpal::Stream>
where
    F: FnMut(&[f32]) + Send + 'static,
{
    let host = cpal::default_host();

    // Find input by name
    let device = host
        .input_devices()?
        .find(|x| x.description().map(|n| n.name() == device_name).unwrap_or(false))
        .context("input not found")?;

    // Get default config (sample rate, Channels, etc.)
    let config = device.default_input_config()?.config();

    // Create stream
    let stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            on_data(data); // send raw samples
        },
        |err| eprintln!("Stream error: {}", err),
        None,
    )?;

    Ok(stream)
}
