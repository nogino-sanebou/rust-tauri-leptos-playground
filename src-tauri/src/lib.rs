use serde::Serialize;
use tauri::{Emitter, WindowEvent};

mod commands;

#[derive(Serialize, Clone)]
struct WindowSizePayload {
    width: u32,
    height: u32,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::invoke_emit::add_numbers,
        ])
        .on_window_event(|window, event| {
            if let WindowEvent::Resized(size) = event {
                let payload = WindowSizePayload {
                    width: size.width,
                    height: size.height,
                };
                let _ = window.emit("window-resized", payload);
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
