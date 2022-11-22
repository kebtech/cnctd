use std::{sync::{Arc, Mutex}, borrow::Borrow};
use serde_json::json;
use state::Storage;
use serde::{Serialize, Deserialize};
use tauri::{AppHandle, Manager};
use crate::recorder::{self, AudioClip};
use wav;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutgoingMessage {
	pub channel: String,
	pub instruction: String,
	pub data: Option<serde_json::Value>,
}

pub static RECORDING_STATE: Storage<Mutex<bool>> = Storage::new();

// type RecordingState = Arc<Mutex<Option<false>>>;

impl OutgoingMessage {
	pub fn new(channel: String, instruction: String, data: Option<serde_json::Value>) -> Self {
		let msg = OutgoingMessage {
			channel,
			instruction,
			data,
		};
		msg
	}
    pub fn to_window(&self, app_handle: &AppHandle) {
        match app_handle.emit_to("main", "message", self) {
            Ok(_) => println!("message sent"),
            Err(e) => println!("error: {}", e)
        }
    }
    pub fn as_response(&self, app_handle: &AppHandle, response_channel: &str) {
        match app_handle.emit_to("main", response_channel, self) {
            Ok(_) => println!("message sent"),
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
                    Ok(path) => {

                        let data = json!({"path": path });
                        let msg = OutgoingMessage::new(
                            "recorder".into(),
                            "new-file".into(),
                            Some(data),
                        );
                        msg.as_response(&app_handle, &response_channel);
                    }
                    Err(e) => {
                        let msg = OutgoingMessage::new(
                            "recorder".into(),
                            e.to_string(),
                            None,
                        );
                        msg.as_response(&app_handle, &response_channel);
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

