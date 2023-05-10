// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod utils;
mod db;


fn main() {
    tauri::Builder::default()
        .manage(utils::AppState {
            photos_library_path: Default::default() 
        })
        .invoke_handler(tauri::generate_handler![utils::photos_library_dialog])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
