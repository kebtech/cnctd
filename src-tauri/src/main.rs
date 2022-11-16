#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

pub mod recorder;

use cpal::{Stream, traits::{DeviceTrait, HostTrait, StreamTrait}};

fn main() {
  println!("{}", record());
  tauri::Builder::default()
    .on_page_load(|window, _| {
      window.open_devtools();
    })
    .invoke_handler(tauri::generate_handler![recorder::get_devices])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
pub fn record() -> String {
  let host = cpal::default_host();
  let mut input_devices: Vec<cpal::Device> = vec![];
  match host.default_input_device() {
      Some(device) => "got it".into(),
      None => "doh".into()
  }
}
