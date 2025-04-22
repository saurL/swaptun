use backend::user::{LoginRequest, LoginResponse};
use tauri::{Manager, State};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_log::{Target, TargetKind};
mod app;
mod backend;
mod deezer;
mod spotify;
mod validators;
use crate::backend::user::{CreateUserRequest, RegisterResponse};

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
        .invoke_handler(tauri::generate_handler![register, login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn register(
    app: State<'_, App>,
    username: &str,
    password: &str,
    first_name: &str,
    last_name: &str,
    email: &str,
) -> Result<RegisterResponse, String> {
    let request = CreateUserRequest {
        username: username.to_string(),
        password: password.to_string(),
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        email: email.to_string(),
    };
    // ICI on peut accèder aux éléments de l'App
    match app.register(request).await {
        Ok(response) => {
            if response.is_success() {
                return Ok(RegisterResponse { succed: true });
            } else {
                return Err("Failed to register".to_string());
            }
        }
        Err(e) => {
            return Err(format!("Error: {}", e));
        }
    }
}

#[tauri::command]
async fn login(
    app: State<'_, App>,
    username: &str,
    password: &str,
) -> Result<LoginResponse, String> {
    let request = LoginRequest {
        username: username.to_string(),
        password: password.to_string(),
    };
    // ICI on peut accèder aux éléments de l'App
    match app.login(request).await {
        Ok(response) => Ok(response),
        Err(e) => Err(e.to_string()),
    }
}
