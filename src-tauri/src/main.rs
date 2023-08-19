// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn save_command(command: &str) {
  println!("{}", command);
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![save_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

