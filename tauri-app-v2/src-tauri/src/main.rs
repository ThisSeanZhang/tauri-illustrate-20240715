// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;

use tauri::{async_runtime, Listener, Window};
#[derive(Debug, serde::Serialize)]
enum MyError {
    FooError,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(async)]
async fn greet(window: Window, name: &str) -> Result<String, MyError> {
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(1)).await;
    });
    tokio::time::sleep(Duration::from_secs(1)).await;
    window.listen("test", move |_event| {
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
        });

        // async_runtime::spawn(async move {
        //     tokio::time::sleep(Duration::from_secs(1)).await;
        // });
    });
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}


fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
