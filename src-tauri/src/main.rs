#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

#[cfg(debug_assertions)]
#[cfg(target_os = "macos")]
embed_plist::embed_info_plist!("../Info.plist");

pub mod recorder;
pub mod router;
pub mod tuner;
// pub mod transcoder;
use tauri::Manager;
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};


fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![
			router::start_recording,
			router::get_recorder_inputs,
			router::stop_recording,
			router::start_tuner,
			router::stop_tuner,
			router::test,
		])
		.setup(|app| {
			let window = app.get_window("main").unwrap();
			window.open_devtools();
			#[cfg(target_os = "macos")]
			apply_vibrancy(&window, NSVisualEffectMaterial::UnderWindowBackground, None, None)
			  .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
	  
			#[cfg(target_os = "windows")]
			apply_blur(&window, Some((18, 18, 18, 125)))
			  .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
	  
			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

pub fn timestamp() -> i64 {
	let timestamp = chrono::Duration::num_milliseconds(&chrono::Duration::from_std(std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap()).unwrap());
	timestamp
}