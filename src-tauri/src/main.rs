// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::Manager;
use tauri::api::log::{LogTarget, LoggerBuilder};


mod stream;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let logger = LoggerBuilder::new()
        .targets([LogTarget::LogDir, LogTarget::Stdout])
        .build();
      app.manage(logger);
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![stream::stream_fetch])
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn log_message(message: String) {
  tauri::api::log::info!("{}", message);
}