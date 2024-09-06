// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod smtp;
use smtp::method::send_mail;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_mail])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
