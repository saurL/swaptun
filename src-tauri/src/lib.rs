use std::sync::Arc;

use log::info;
use swaptun_backend::{
    CreateUserRequest, GetPlaylistResponse, LoginEmailRequest, LoginRequest, LoginResponse,
    VerifyTokenRequest,
};
use tauri::{async_runtime::spawn, command, Emitter, Manager, State, Url, Window};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_log::{Target, TargetKind};
mod app;
mod backend;
use app::App;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_pinia::init())
        .plugin(tauri_plugin_oauth::init())
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
            spawn(async move {
                let swaptun_app = App::new(app_handle.clone()).await;
                swaptun_app.set_app_ready().await;
                app_handle.manage(swaptun_app);
                app_handle.emit("app_ready", "").unwrap();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            register,
            login,
            login_email,
            verify_token,
            get_autorization_url_spotify,
            is_app_ready,
            test_spotify,
            get_playlists_spotify,
            get_playlists_deezer,
            set_auth_header,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[command]
async fn register(
    app: State<'_, Arc<App>>,
    username: &str,
    password: &str,
    first_name: &str,
    last_name: &str,
    email: &str,
) -> Result<bool, String> {
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
                return Ok(true);
            } else {
                return Err("Failed to register".to_string());
            }
        }
        Err(e) => {
            return Err(format!("Error: {}", e));
        }
    }
}

#[command]
async fn login(
    app: State<'_, Arc<App>>,
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

#[command]
async fn login_email(
    app: State<'_, Arc<App>>,
    email: &str,
    password: &str,
) -> Result<LoginResponse, String> {
    let request = LoginEmailRequest {
        email: email.to_string(),
        password: password.to_string(),
    };
    // ICI on peut accèder aux éléments de l'App
    match app.login_email(request).await {
        Ok(response) => Ok(response),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
async fn verify_token(app: State<'_, Arc<App>>, token: String) -> Result<bool, String> {
    let request = VerifyTokenRequest {
        token: token.clone(),
    };
    match app.verify_token(request).await {
        Ok(is_valid) => Ok(is_valid.valid),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
async fn get_autorization_url_spotify(app: State<'_, Arc<App>>) -> Result<String, String> {
    info!("get_autorization_url_spotify called");
    match app.get_autorization_url_spotify().await {
        Ok(response) => {
            info!("get_autorization_url_spotify response: {}", response.url);
            Ok(response.url)
        }
        Err(e) => Err(format!("Error: {}", e)),
    }
}

#[command]
async fn is_app_ready(app: State<'_, Arc<App>>) -> Result<bool, String> {
    app.is_app_ready().await
}

#[command]
async fn test_spotify(app: State<'_, Arc<App>>) -> Result<(), String> {
    match app.import_playlist_backend_request().await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
async fn get_playlists_spotify(app: State<'_, Arc<App>>) -> Result<GetPlaylistResponse, String> {
    match app.get_playlists_spotify().await {
        Ok(response) => Ok(response),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
async fn get_playlists_deezer(app: State<'_, Arc<App>>) -> Result<GetPlaylistResponse, String> {
    match app.get_playlists_deezer().await {
        Ok(response) => Ok(response),
        Err(e) => Err(e.to_string()),
    }
}
#[command]
async fn set_auth_header(app: State<'_, Arc<App>>, token: String) -> Result<(), String> {
    info!("setting_auth_header");
    app.set_auth_header(token).await;
    Ok(())
}
