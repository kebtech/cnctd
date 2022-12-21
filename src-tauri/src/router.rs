use std::{sync::{Arc, Mutex}, borrow::Borrow};
use serde_json::json;
use state::Storage;
use serde::{Serialize, Deserialize};
use tauri::{AppHandle, Manager};
use crate::{recorder::{self, AudioClip}, tuner};
use wav;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutgoingMessage {
	pub channel: String,
	pub payload: Payload
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Payload {
    pub instruction: String,
	pub data: Option<serde_json::Value>,
}

pub static RECORDING_STATE: Storage<Mutex<bool>> = Storage::new();
pub static TUNER_STATE: Storage<Mutex<bool>> = Storage::new();

// type RecordingState = Arc<Mutex<Option<false>>>;

impl OutgoingMessage {
	pub fn new(channel: String, instruction: String, data: Option<serde_json::Value>, app_handle: &AppHandle) {
		match app_handle.emit_to("main", &channel, Payload { instruction, data }) {
            Ok(_) => {}
            Err(e) => println!("error: {}", e)
        }
	}
}

#[tauri::command]
pub fn start_recording(app_handle: AppHandle, response_channel: String) -> Result<String, String> {
    std::thread::spawn(move || {
        match RECORDING_STATE.try_get() {
            Some(state) => {
                let mut status = state.lock().unwrap();
                *status = true;
                drop(status);
            }
            None => {
                RECORDING_STATE.set(Mutex::new(true));
                ()
            }
        }
        // RECORDING_STATE.get_or_set(|| { Mutex::new(true) });
        match recorder::AudioClip::record(&app_handle) {
            Ok(handle) => {
                loop {
                    println!("checking record status");
                    let state = RECORDING_STATE.get();
                    // *status;
                    let status = *state.lock().unwrap();
                    println!("recording: {}", status);
                    if !status {
                        drop(status);
                        drop(state);
                        println!("recording stopped");
                        break
                    } else {
                        std::thread::sleep(std::time::Duration::from_secs(1));
                    }
                }
                println!("broken out of loop");
                // let filename = &handle.filename.to_string();
                match handle.stop() {
                    Ok(samples) => {

                        let data = json!({"samples": samples });
                        OutgoingMessage::new(response_channel,"new-file".into(),Some(data), &app_handle);
                    }
                    Err(e) => {
                        let msg = OutgoingMessage::new(response_channel,e.to_string(),None, &app_handle);
                    }
                }
                
                ()
            },
            Err(e) => println!("error: {}", e)
        }
    });
    
    Ok("started".into())
}

#[tauri::command]
pub fn stop_recording() -> Result<String, String> {
    let mut status = RECORDING_STATE.get().lock().unwrap();
    *status = false;
    drop(status);
    Ok("stopped".into())
}

#[tauri::command]
pub fn get_recorder_inputs() -> Result<serde_json::Value, String> {
    match recorder::get_devices() {
        Ok(devices) => Ok(devices),
        Err(e) => Err(e.to_string())
    }
}


#[tauri::command]
pub async fn start_tuner(app_handle: AppHandle) -> Result<String, String> {
    std::thread::spawn(move || {
        match TUNER_STATE.try_get() {
            Some(state) => {
                let mut status = state.lock().unwrap();
                *status = true;
                drop(status);
            }
            None => {
                TUNER_STATE.set(Mutex::new(true));
                ()
            }
        }
        match tuner::start(app_handle) {
            Ok(stream) => {
                loop {
                    println!("checking tuner status");
                    let state = TUNER_STATE.get();
                    // *status;
                    let status = *state.lock().unwrap();
                    println!("tuner: {}", status);
                    if !status {
                        drop(status);
                        drop(state);
                        println!("tuner stopped");
                        break
                    } else {
                        std::thread::sleep(std::time::Duration::from_secs(1));
                    }
                }
            }
            Err(e) => print!("error: {}", e)
        }
    });

    Ok("ok".into())
}

#[tauri::command]
pub fn stop_tuner() -> Result<String, String> {
    println!("uhh lets stop this stream");
    let mut status = TUNER_STATE.get().lock().unwrap();
    *status = false;
    drop(status);
    Ok("stopped".into())
}

#[tauri::command]
pub fn test(msg: String) {
    println!("from client: {}", msg);
}

#[tauri::command]
pub fn open_url(url: String) -> Result<(), String> {
    open::that(url).map_err(|e| e.to_string())?;
    Ok(())
}