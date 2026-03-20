#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod services;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![commands::detect_paths])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
