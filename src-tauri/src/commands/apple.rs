use crate::App;
use log::error;
use std::sync::Arc;
use swaptun_backend::GetPlaylistResponse;
use tauri::{command, State};
use tauri_plugin_musickit::AuthorizationResponse;
#[command]
pub async fn connect_apple_music(
    app: State<'_, Arc<App>>,
) -> Result<AuthorizationResponse, String> {
    match app.connect_apple_music().await {
        Ok(response) => Ok(response),
        Err(e) => {
            error!("Failed to connect to Apple Music: {}", e);
            Err(e.to_string())
        }
    }
}

#[command]
pub async fn get_apple_music_playlists(
    app: State<'_, Arc<App>>,
) -> Result<GetPlaylistResponse, String> {
    match app.get_apple_music_playlists().await {
        Ok(playlists) => Ok(playlists),
        Err(e) => {
            error!("Failed to get Apple Music playlists: {}", e);
            Err(e.to_string())
        }
    }
}

#[command]
pub async fn disconnect_apple_music(app: State<'_, Arc<App>>) -> Result<(), String> {
    match app.disconnect_apple_music().await {
        Ok(_) => Ok(()),
        Err(e) => {
            error!("Failed to disconnect from Apple Music: {}", e);
            Err(e.to_string())
        }
    }
}
