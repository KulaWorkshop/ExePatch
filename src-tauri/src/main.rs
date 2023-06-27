#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod music;
mod types;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![commands::open, commands::get_tracks, commands::set_tracks])
        .run(tauri::generate_context!())
        .expect("An unknown error occurred.");
}