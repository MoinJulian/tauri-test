// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use tauri_plugin_http::reqwest;
use dotenv::dotenv;
use std::env;

#[tauri::command]
async fn get_courses() -> Result<serde_json::Value, String> {
    dotenv().ok();
    let api_key: String = env::var("SECRET_API_KEY").expect("SECRET_API_KEY must be set");
    let url: String = format!("https://api.realgolf.games/v1/courses?apiKey={}&limit=10&skip=0", api_key);
    let res: reqwest::Response = reqwest::get(url.as_str()).await.map_err(|e: reqwest::Error| e.to_string())?;
    let json: serde_json::Value = res.json::<serde_json::Value>().await.map_err(|e: reqwest::Error| e.to_string())?;
    Ok(json)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, get_courses])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}