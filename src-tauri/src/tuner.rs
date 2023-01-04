use cpal::{Stream, traits::{DeviceTrait, StreamTrait}, StreamConfig};

use dasp::{Sample, sample::FromSample};
use pitch_detection::detector::{PitchDetector, mcleod::McLeodDetector};
use serde_json::json;
use tauri::{AppHandle, Manager};
use crate::{recorder::{AudioInput, StreamHandle}, router::OutgoingMessage};

// pub struct StreamHandle(Stream);

pub fn start(app_handle: AppHandle) -> Result<Stream, anyhow::Error> {
    println!("starting tuner");
    let input = AudioInput::new(None)?;
    let supported_config = &input.device.default_input_config()?;
    let err_fn = move |err| {
        println!("an error occurred on stream: {}", err);
    };
    println!("supported config: {:?}", supported_config);
    let config = &input.config;
    let config = StreamConfig { 
        channels: config.channels, 
        sample_rate: config.sample_rate, 
        buffer_size: cpal::BufferSize::Fixed(2048) 
    };
    println!("config: {:?}", config);
    let buffer_size_range = &config.buffer_size;
    println!("buffer size: {:?}, ", buffer_size_range);
    println!("input: {}", input.name);
    let sample_rate = config.sample_rate.0 as usize;
    let stream = input.device.build_input_stream(
        &config,
        move |data, _: &_| { process_audio(data, sample_rate, app_handle.clone()) },
        err_fn,
    )?;
    
    println!("sample Rate: {}", sample_rate);
    // const SAMPLE_RATE: usize = 44100;
    const SIZE: usize = 2048;
    const PADDING: usize = SIZE / 2;
    const POWER_THRESHOLD: f32 = 0.01;
    const CLARITY_THRESHOLD: f32 = 0.5;
    
    

    fn process_audio(data: &[f32], sample_rate: usize, app_handle: AppHandle) {
        let mut detector = McLeodDetector::<f32>::new(SIZE, PADDING);
        match detector.get_pitch(&data, sample_rate, POWER_THRESHOLD, CLARITY_THRESHOLD) {
            Some(pitch) => {
                OutgoingMessage::new(
                    "tuner".into(), 
                    "tune".into(), 
                    Some(json!({"frequency": pitch.frequency, "clarity": pitch.clarity})), 
                    &app_handle
                )
            }
            None => {}
        }
            
        // OutgoingMessage::new(
        //     "tuner".into(), 
        //     "buffer".into(), 
        //     Some(json!({ "buffer": data, "sample_rate": SAMPLE_RATE })), 
        //     &app_handle
        // )
        // for &sample in data.iter() {
        //     detect(sample);
        //     // println!("sample!: {}", sample);
        // }
    }
    
   
    Ok(stream)
}

