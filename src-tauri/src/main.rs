#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// use br_tauri_hcj::{FileNames, RpMessage, Section, Snippet};


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
// #[tauri::command]
// fn search(search_str: &str) -> Result<RpMessage,String> {
//     //check for directory with JSONs
//     //create if it doesn't exist
//     //check for JSONs
//     //create if they dont exist
//     //read elements to Vectors
// }

fn main() {
    tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![greet])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
