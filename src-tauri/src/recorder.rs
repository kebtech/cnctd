

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Sample};
use serde_json::json;
use std::fs::File;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};
use anyhow::anyhow;
use tauri;

type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

// pub struct AudioInput {
//     pub name: String,
//     pub 
// }

#[tauri::command]
pub fn start() -> Result<(), String> {
    let host = cpal::default_host();
    let input = host.default_input_device().ok_or("no default input")?;
    let config = input.default_input_config().map_err(|e| { e.to_string() })?;
    


    Ok(())
}

#[tauri::command]
pub fn get_devices() -> Result<serde_json::Value, String> {
    let host = cpal::default_host();
    let input_devices = host.input_devices().map_err(|e| {e.to_string()})?;
    let d2 = host.input_devices().map_err(|e| {e.to_string()})?;
    let len = d2.count();
    let mut inputs: Vec<String> = vec![];

    for (i, input) in input_devices.enumerate() {
        let configs = input.supported_input_configs().map_err(|e| { e.to_string()})?;
        let name = input.name().unwrap_or("no name".to_string());
        println!("name! {}", name);
        for config in configs {
            println!("{:?}", config);
        }
        inputs.push(name);
        if i == len - 1 { break }

    }
    let json = json!({"inputs": inputs});
    Ok(json)

}



//     let host = cpal::default_host();
//     let input = host.default_input_device()?;

//     let config = input.default_input_config()?;

//     const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/recorded.wav");
//     let spec = wav_spec_from_config(&config);
//     let writer = hound::WavWriter::create(PATH, spec)?;
//     let writer = Arc::new(Mutex::new(Some(writer)));

//     // A flag to indicate that recording is in progress.
//     println!("Begin recording...");

//     // Run the input stream on a separate thread.
//     let writer_2 = writer.clone();

//     let err_fn = move |err| {
//         eprintln!("an error occurred on stream: {}", err);
//     };


//     let stream = match config.sample_format() {
//         cpal::SampleFormat::I8 => input.build_input_stream(
//             &config.into(),
//             move |data, _: &_| write_input_data::<i8, i8>(data, &writer_2),
//             err_fn,
//         )?,
//         cpal::SampleFormat::I16 => input.build_input_stream(
//             &config.into(),
//             move |data, _: &_| write_input_data::<i16, i16>(data, &writer_2),
//             err_fn,
//         )?,
//         cpal::SampleFormat::I32 => input.build_input_stream(
//             &config.into(),
//             move |data, _: &_| write_input_data::<i32, i32>(data, &writer_2),
//             err_fn,
//         )?,
//         cpal::SampleFormat::F32 => input.build_input_stream(
//             &config.into(),
//             move |data, _: &_| write_input_data::<f32, f32>(data, &writer_2),
//             err_fn,
//         )?,
//         sample_format => {
//             return Err(anyhow::Error::msg(format!(
//                 "Unsupported sample format '{sample_format}'"
//             )))
//         }
//     };

//     stream.play()?;

//     // Let recording go for roughly three seconds.
//     std::thread::sleep(std::time::Duration::from_secs(10));
//     drop(stream);
//     writer.lock().unwrap().take().unwrap().finalize()?;
//     println!("Recording {} complete!", PATH);

//     Ok(())
// }

// fn sample_format(format: cpal::SampleFormat) -> hound::SampleFormat {
//     if format.is_float() {
//         hound::SampleFormat::Float
//     } else {
//         hound::SampleFormat::Int
//     }
// }

// fn wav_spec_from_config(config: &cpal::SupportedStreamConfig) -> hound::WavSpec {
//     hound::WavSpec {
//         channels: config.channels() as _,
//         sample_rate: config.sample_rate().0 as _,
//         bits_per_sample: (config.sample_format().sample_size() * 8) as _,
//         sample_format: sample_format(config.sample_format()),
//     }
// }

// // fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
// // where
// //     T: Sample,
// //     U: Sample + hound::Sample + cpal::Sample::FromSample<T>,
// // {
// //     if let Ok(mut guard) = writer.try_lock() {
// //         if let Some(writer) = guard.as_mut() {
// //             for &sample in input.iter() {
// //                 let sample: U = U::from_sample(sample);
// //                 writer.write_sample(sample).ok();
// //             }
// //         }
// //     }
// // }

// type ClipHandle = Arc<Mutex<Option<AudioClip>>>;

// fn write_input_data<T>(input: &[T], channels: u16, writer: &ClipHandle)
// where
//     T: cpal::Sample,
// {
//     if let Ok(mut guard) = writer.try_lock() {
//         if let Some(clip) = guard.as_mut() {
//             for frame in input.chunks(channels.into()) {
//                 clip.samples.push(frame[0].to_f32());
//             }
//         }
//     }
// }

