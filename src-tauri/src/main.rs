// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::collections::HashMap;

use serde_json::Value;

pub mod epapi;


#[tokio::main]
async fn main() {
  tauri::async_runtime::set(tokio::runtime::Handle::current());
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, get_usage])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
async fn greet(name: String) -> Result<String, ()> {
  Ok(format!("Hello, {}!", name))
}

#[tauri::command]
async fn get_usage(start: String, end: Option<String>) -> Result<HashMap<String, Value>, ()> {
    epapi::get_usage(start, end).await
}