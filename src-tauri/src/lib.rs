use std::error::Error;

use log::info;
use swaptun_backend::{
    CreateUserRequest, LoginEmailRequest, LoginRequest, LoginResponse, VerifyTokenRequest,
};
use tauri::{Manager, State};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri_plugin_log::{Target, TargetKind};
mod app;
mod backend;
mod deezer;
// mod spotify;

use app::App;
use tauri_plugin_http::reqwest::{
    blocking::{Body, Client, Response},
    StatusCode,
};
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_pinia::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .setup(|app| {
            check_rustls();
            let app_handle = app.handle().clone();
            app.manage(App::new(app_handle));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            register,
            login,
            login_email,
            verify_token,
            //authenticate_spotify,
        ])
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

#[tauri::command]
async fn login_email(
    app: State<'_, App>,
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

#[tauri::command]
async fn verify_token(app: State<'_, App>, token: String) -> Result<bool, String> {
    let request = VerifyTokenRequest {
        token: token.clone(),
    };
    match app.verify_token(request).await {
        Ok(is_valid) => Ok(is_valid.valid),
        Err(e) => Err(e.to_string()),
    }
}
/*
#[tauri::command]
async fn authenticate_spotify(
    app: State<'_, App>,
    client_id: &str,
    client_secret: &str,
) -> Result<bool, String> {
    match app.authenticate_spotify(client_id, client_secret).await {
        Ok(_) => Ok(true),
        Err(e) => Err(e.to_string()),
    }
}
 */

use rustls::{ClientConfig, RootCertStore};
pub fn check_rustls() -> Result<(), Box<dyn Error>> {
    info!("Checking Rustls initialization");
    info!("Rustls version: ");
    // Créer un RootCertStore vide
    info!("Creating empty RootCertStore");
    let mut root_store = RootCertStore::empty();
    info!("RootCertStore created successfully");

    // Charger les certificats racines de webpki-roots
    info!("Loading webpki-roots certificates");
    let trust_anchors = webpki_roots::TLS_SERVER_ROOTS.0.iter().map(|ta| {
        info!("Processing trust anchor: {:?}", ta.subject);
        rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
            ta.subject.to_vec(),
            ta.spki.to_vec(),
            ta.name_constraints.as_ref().map(|nc| nc.to_vec()),
        )
    });
    root_store.add_server_trust_anchors(trust_anchors);
    info!("webpki-roots certificates loaded successfully");

    // Créer la configuration TLS
    info!("Creating ClientConfig");
    let config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    info!("ClientConfig created successfully");

    info!("Rustls configuration created successfully");
    Ok(())
}
