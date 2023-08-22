// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{Connection, Result};
use std::time::{Duration, SystemTime};

struct Command {
  command: String,
  time: SystemTime
}

#[tauri::command]
fn load_database() {
  let conn = Connection::open_in_memory();

  let result = conn.unwrap().execute(
        "CREATE TABLE command (
          command TEXT NOT NULL,
          time TIME
      )",
      (), // empty list of parameters.
  );

  println!("{}", result.unwrap());
}

#[tauri::command]
fn save_command(command: &str) {
  println!("{}", command);
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![save_command, load_database])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

