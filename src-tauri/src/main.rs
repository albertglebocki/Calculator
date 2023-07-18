// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn create_new_window(app: tauri::AppHandle) {
    let now: i64 = chrono::offset::Local::now().timestamp();
    let unique_identifier = format!("label{}", now);
    let _window = tauri::WindowBuilder::new(&app, unique_identifier, tauri::WindowUrl::App("index.html".into()))
        .title("Calculator")
        .resizable(false)
        .fullscreen(false)
        .center()
        .transparent(true)
        .decorations(false)
        .inner_size(350.0, 530.0)
        .build()
        .unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_new_window])
        .run(tauri::generate_context!())
        .expect("There is some issue with running the Calculator");
}