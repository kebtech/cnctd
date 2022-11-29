
// use crate::sample_formats::{SampleFormat, FromSample};

use chrono::{Utc, DateTime};
use cpal::{Device, StreamConfig, Stream};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use dasp::sample::FromSample;
use dasp::{interpolate::linear::Linear, signal, Signal};

use hound::WavWriter;
use serde_json::json;
use std::fs::File;
use std::io::BufWriter;
use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use tauri::{self, Manager, AppHandle};
use crate::router::OutgoingMessage;
use crate::transcoder;
// use crate::encoder::encode;



type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

pub struct AudioInput {
    pub name: String,
    pub config: StreamConfig,
    pub device: Device,
}

pub struct RecordState {
    clip: AudioClip,
}


pub struct RecordHandle {
    stream: Stream,
    /// Option is only taken in "stop".
    clip: Arc<Mutex<Option<RecordState>>>,
    pub writer: Arc<Mutex<Option<WavWriter<BufWriter<File>>>>>,
    pub path: String,
}


impl RecordHandle {
    pub fn stop(self) -> Result<String, anyhow::Error> {
        drop(self.stream);
        // let clip = self.clip.lock().unwrap().take().unwrap().clip;
        self.writer.lock().unwrap().take().unwrap().finalize().unwrap();
        println!("stream dropped");
        let encoded = AudioClip::encode(self.path)?;
        Ok(encoded)
        // handle.writer.lock().unwrap().take().unwrap().finalize().unwrap();
        // clip.lock().unwrap().take().unwrap().finalize()?;
        
        // println!("Recorded clip has {} samples", writer.samples.len());
        // clip
    }
}

pub trait StreamHandle {
    fn sample_rate(&self) -> u32;
    fn samples(&self) -> usize;
    fn time(&self) -> f64;
}

impl StreamHandle for RecordHandle {
    fn sample_rate(&self) -> u32 {
        let mut state = self.clip.lock().unwrap();
        let state = state.as_mut().unwrap();

        state.clip.sample_rate
    }

    fn samples(&self) -> usize {
        let mut state = self.clip.lock().unwrap();
        let state = state.as_mut().unwrap();

        state.clip.samples.len()
    }

    fn time(&self) -> f64 {
        let mut state = self.clip.lock().unwrap();
        let state = state.as_mut().unwrap();

        (state.clip.samples.len()) as f64 / (state.clip.sample_rate as f64)
    }
}

#[derive(Clone)]
pub struct AudioClip {
    pub date: DateTime<Utc>,
    pub samples: Vec<f32>,
    // pub samples_l: Vec<f32>,
    // pub samples_r: Vec<f32>,
    pub sample_rate: u32,
}

impl AudioInput {
    pub fn new(requested: Option<String>) -> Result<Self, anyhow::Error> {
        let host = cpal::default_host();

        let device = match requested {
            Some(requested_name) => {
                let mut input_devices = host.input_devices()?;
                let requested_device = input_devices.find(|device| { 
                    device.name().unwrap_or("invalid".into()) == requested_name 
                });
                if requested_device.is_some() {
                    requested_device.unwrap()
                } else {
                    let default = host.default_input_device().ok_or(anyhow!("no default device"))?;
                    default
                }
            }
            None => {
                let default = host.default_input_device().ok_or(anyhow!("no default device"))?;
                default
            }
        };
        let name = device.name().unwrap_or("no name".into());
        let config = device.default_input_config()?;
        let config = config.into();
        let input = AudioInput {
            name,
            config,
            device,
        };

        Ok(input)

    }
}

impl AudioClip {
    pub fn record(app_handle: &AppHandle) -> Result<RecordHandle, anyhow::Error> {
        let input = AudioInput::new(None)?;
        let clip = AudioClip {
            date: Utc::now(),
            // samples_l: Vec::new(),
            // samples_r: Vec::new(),
            samples: Vec::new(),
            sample_rate: input.config.sample_rate.0,
        };

        // let PATH: &str = &format!("{}{}", &filename, "/recorded.wav");
        let dir = app_handle.path_resolver().resource_dir().unwrap().into_os_string().into_string().unwrap();
        let path = format!("{}/recordings/{}", dir, "recorded.wav");
        
        // const PATH: &str = "recordings/recorded.wav";
        // println!("PATH: {}", PATH);
        println!("path: {}", path);
        let config = &input.device.default_input_config()?;
        let spec = wav_spec_from_config(&config);
        let writer = hound::WavWriter::create(path.to_string(), spec)?;
        let writer = Arc::new(Mutex::new(Some(writer)));
    
        let clip = Arc::new(Mutex::new(Some(RecordState { clip })));
        
    
        println!("start recording");
        let err_fn = move |err| {
            println!("an error occurred on stream: {}", err);
        };

        let writer_2 = writer.clone();
    
        fn write_to_wav<T, U>(input: &[T], writer: &WavWriterHandle) where T: cpal::Sample, U:cpal::Sample + hound::Sample + FromSample<T> {
            if let Ok(mut guard) = writer.try_lock() {
                if let Some(writer) = guard.as_mut() {
                    for &sample in input.iter() {
                        let sample: U = U::from_sample_(sample);
                        writer.write_sample(sample).ok();
                    }
                }
            }
        }
        fn wav_spec_from_config(config: &cpal::SupportedStreamConfig) -> hound::WavSpec {
            hound::WavSpec {
                channels: config.channels() as _,
                sample_rate: config.sample_rate().0 as _,
                bits_per_sample: (config.sample_format().sample_size() * 8) as _,
                sample_format: hound::SampleFormat::Float,
            }
        }
        let stream = input.device.build_input_stream(
            &input.config.into(), 
            move |data, _: &_| { write_to_wav::<f32, f32>(data, &writer_2) }, 
            err_fn,  
        )?;
    
        stream.play()?;
    
        Ok(RecordHandle { stream, clip, writer, path })
    
    }
    pub fn resample(&self, sample_rate: u32) -> AudioClip {
        if self.sample_rate == sample_rate {
            return self.clone();
        }

        let mut signal = signal::from_iter(self.samples.iter().copied());

        let a = signal.next();
        let b = signal.next();

        let linear = Linear::new(a, b);
        

        AudioClip {
            date: self.date,
            samples: signal.from_hz_to_hz(linear, self.sample_rate as f64, sample_rate as f64)
                .take(self.samples.len() * (sample_rate as usize) / (self.sample_rate as usize))
                .collect(),
            sample_rate,
        }
    }

    pub fn encode(filename: String) -> Result<String,anyhow::Error> {
        let output = transcoder::transcode(filename.to_string())?;
        Ok(output)
    }
}




pub fn get_devices() -> Result<serde_json::Value, anyhow::Error> {
    let host = cpal::default_host();
    let input_devices = host.input_devices()?;
    let d2 = host.input_devices()?;
    let len = d2.count();
    let mut inputs: Vec<String> = vec![];

    for (i, input) in input_devices.enumerate() {
        let configs = input.supported_input_configs()?;
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

