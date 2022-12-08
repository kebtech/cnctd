
// use crate::sample_formats::{SampleFormat, FromSample};

use chrono::{Utc, DateTime};
use cpal::{Device, StreamConfig, Stream};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use dasp::sample::{Sample, FromSample};
// use dasp::{interpolate::linear::Linear, signal, Signal};

use hound::{WavWriter, WavSpec, WavReader, WavSpecEx};
use serde::{Serialize, Deserialize};
use serde_json::json;
use wav::Header;
use std::fs::File;
use std::io::{BufWriter, BufReader, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

use anyhow::anyhow;
use tauri::{self, AppHandle};
// use crate::transcoder;
// use crate::encoder::encode;
// use lame;


type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

pub struct AudioInput {
    pub name: String,
    pub config: StreamConfig,
    pub device: Device,
}


pub struct RecordHandle {
    stream: Stream,
    /// Option is only taken in "stop".
    pub clip: Arc<Mutex<Option<AudioClip>>>,
    pub writer: WavWriterHandle,
    pub path: String,
    pub dir: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct CnctdHeader {
    pub audio_format: u16,
    pub channel_count: u16,
    pub sampling_rate: u32,
    pub bytes_per_second: u32,
    pub bytes_per_sample: u16,
    pub bits_per_sample: u16,
}

impl CnctdHeader {
    pub fn from_wav_spec(spec: WavSpec) -> Self {
        let audio_format = match spec.sample_format {
            hound::SampleFormat::Int => 0x01,
            hound::SampleFormat::Float => 0x03
        };
        let header = wav::header::Header::new(audio_format, spec.channels, spec.sample_rate, spec.bits_per_sample);
        let cnctd_header = Self { 
            audio_format: header.audio_format,
            channel_count: header.channel_count, 
            sampling_rate: header.sampling_rate, 
            bytes_per_second: header.bytes_per_second, 
            bytes_per_sample: header.bytes_per_sample, 
            bits_per_sample: header.bits_per_sample 
        };
        cnctd_header
    }
}

impl RecordHandle {
    pub fn stop(self) -> Result<(Vec<f32>, Vec<f32>), anyhow::Error> {
        drop(self.stream);
        println!("stream dropped");
        let writer = self.writer.lock().unwrap().take().ok_or(anyhow!("error getting writer"))?;
        let spec = writer.spec();
        let length = writer.len();
        let audio_format = match spec.sample_format {
            hound::SampleFormat::Int => wav::header::WAV_FORMAT_PCM,
            hound::SampleFormat::Float => wav::header::WAV_FORMAT_IEEE_FLOAT
        };
        let header = wav::header::Header::new(audio_format, spec.channels, spec.sample_rate, spec.bits_per_sample);

        writer.finalize()?;
        println!("spec: {:?}", spec);
        println!("header: {:?}", header);
        
        let wav_reader = hound::WavReader::open(&self.path).unwrap();
        
        let samples = wav_reader.into_samples::<f32>();

        let mut left: Vec<f32> = vec![];
        let mut right: Vec<f32> = vec![];
        for (i, sample) in samples.enumerate() {
            let sample = sample.unwrap();
            if i % 2 == 0 { left.push(sample) } else { right.push(sample) }
            if i == length as usize - 1 { break }
        }
       
        Ok((left, right))
    
    }
}

pub trait StreamHandle {
    fn sample_rate(&self) -> u32;
    fn samples(&self) -> usize;
    fn time(&self) -> f64;
}

// impl StreamHandle for RecordHandle {
//     fn sample_rate(&self) -> u32 {
//         let mut state = self.clip.lock().unwrap();
//         let state = state.as_mut().unwrap();

//         state.clip.sample_rate
//     }

//     fn samples(&self) -> usize {
//         let mut state = self.clip.lock().unwrap();
//         let state = state.as_mut().unwrap();

//         state.clip.samples.len()
//     }

//     fn time(&self) -> f64 {
//         let mut state = self.clip.lock().unwrap();
//         let state = state.as_mut().unwrap();

//         (state.clip.samples.len()) as f64 / (state.clip.sample_rate as f64)
//     }
// }

#[derive(Clone)]
pub struct AudioClip {
    pub date: DateTime<Utc>,
    // pub samples: Vec<f32>,
    pub samples_l: Vec<f32>,
    pub samples_r: Vec<f32>,
    pub sample_rate: u32,
    pub sample_format: hound::SampleFormat
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
        

        // let PATH: &str = &format!("{}{}", &filename, "/recorded.wav");
        let dir = app_handle.path_resolver().resource_dir().unwrap().into_os_string().into_string().unwrap();
        let path = format!("{}/recordings/{}", dir, "recorded.wav");
        
        // const PATH: &str = "recordings/recorded.wav";
        // println!("PATH: {}", PATH);
        println!("path: {}", path);
        let config = &input.device.default_input_config()?;
        let spec = wav_spec_from_config(&config);
        println!("spec!: {:?}", spec);
        


        let clip = AudioClip {
            date: Utc::now(),
            samples_l: vec![] as Vec<f32>,
            samples_r: vec![] as Vec<f32>,
            // samples: Vec::new(),
            sample_rate: input.config.sample_rate.0,
            sample_format: spec.sample_format
        };


        let writer = hound::WavWriter::create(std::path::Path::new(&path), spec)?;

        let writer = Arc::new(Mutex::new(Some(writer)));
        
        let clip = Arc::new(Mutex::new(Some(clip)));

    
        println!("start recording");
        let err_fn = move |err| {
            println!("an error occurred on stream: {}", err);
        };
        let clip_2 = clip.clone();
        let writer_2 = writer.clone();
    
        fn write_to_wav<T, U>(input: &[T], writer: &WavWriterHandle) where T: Sample, U: Sample + hound::Sample + FromSample<T> {
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
                sample_format: sample_format(config.sample_format()),
            }
        }
   
        println!("config!: {:?}", config);
        let stream = match config.sample_format() {
            cpal::SampleFormat::I16 => input.device.build_input_stream(
                &input.config.into(),
                move |data, _: &_| write_to_wav::<i16, i16>(data, &writer_2),
                err_fn,
            )?,

            cpal::SampleFormat::F32 => input.device.build_input_stream(
                &input.config.into(),
                move |data, _: &_| write_to_wav::<f32, f32>(data, &writer_2),
                err_fn,
            )?,
            cpal::SampleFormat::U16 => input.device.build_input_stream(
                &input.config.into(),
                move |data, _: &_| write_to_wav::<i16, i16>(data, &writer_2),
                err_fn,
            )?
        };

    
        stream.play()?;
    
        Ok(RecordHandle { stream, clip, writer, path, dir })
    
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

fn sample_format(format: cpal::SampleFormat) -> hound::SampleFormat {
    match format {
        cpal::SampleFormat::I16 | cpal::SampleFormat::U16 => return hound::SampleFormat::Int,
        cpal::SampleFormat::F32 => return hound::SampleFormat::Float
    }
}