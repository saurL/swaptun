use std::sync::Arc;

use log::info;
use swaptun_backend::GetPlaylistResponse;
use tauri::{command, State};
use tauri_plugin_custom_tabs_manager::{CustomTabsManagerExt, OpenCustomTabSimpleRequest};

use crate::app::App;

#[command]
pub async fn get_autorization_url_spotify(app: State<'_, Arc<App>>) -> Result<String, String> {
    info!("get_autorization_url_spotify called");
    match app.get_autorization_url_spotify().await {
        Ok(response) => {
            info!("get_autorization_url_spotify response: {}", response.url);
            app.app_handle()
                .custom_tabs_manager()
                .open_custom_tab_simple(OpenCustomTabSimpleRequest {
                    url: response.url.clone(),
                    try_native_app: true,
                })
                .expect("error while opening custom tab");
            Ok(response.url)
        }
        Err(e) => Err(format!("Error: {}", e)),
    }
}

#[command]
pub async fn test_spotify(app: State<'_, Arc<App>>) -> Result<(), String> {
    match app.import_playlist_backend_request().await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub async fn get_playlists_spotify(
    app: State<'_, Arc<App>>,
) -> Result<GetPlaylistResponse, String> {
    match app.get_playlists_spotify().await {
        Ok(response) => Ok(response),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub async fn disconnect_spotify(app: State<'_, Arc<App>>) -> Result<(), String> {
    match app.disconnect_spotify().await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
