use tauri::{Manager, State};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use tauri_plugin_log::{Target, TargetKind};
mod app;
mod deezer;
mod spotify;
use app::App;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .setup(|app| {
            let app_handle = app.handle().clone();
            app.manage(App::new(app_handle));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![function_exemple])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
#[tauri::command]
async fn function_exemple(app: State<'_, App>) -> Result<(), String> {
    // ICI on peut accèder aux éléments de l'App
    app.authenticate_spotify("client_id", "client_secret", "code", "redirect_uri")
        .await
        .map_err(|e| e.to_string())
}
