use std::sync::Arc;

use crate::backend::playlist::{GetSharedPlaylistsResponse, SharedPlaylist};
use swaptun_backend::SendPlaylistRequest;
use tauri::{command, State};

use crate::app::App;

#[command]
pub async fn send_playlist(
    app: State<'_, Arc<App>>,
    playlist_id: i32,
    req: SendPlaylistRequest,
) -> Result<bool, String> {
    match app.send_playlist(playlist_id, req).await {
        Ok(status) => {
            if status.is_success() {
                Ok(true)
            } else {
                Err(format!("Failed to send playlist, status: {}", status))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub async fn share_playlist(
    app: State<'_, Arc<App>>,
    playlist_id: i32,
    user_id: i32,
) -> Result<bool, String> {
    match app.share_playlist(playlist_id, user_id).await {
        Ok(status) => {
            if status.is_success() {
                Ok(true)
            } else {
                Err(format!("Failed to share playlist, status: {}", status))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub async fn get_shared_playlists(
    app: State<'_, Arc<App>>,
) -> Result<Vec<SharedPlaylist>, String> {
    match app.get_shared_playlists().await {
        Ok(response) => Ok(response.playlists),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub async fn mark_shared_playlist_viewed(
    app: State<'_, Arc<App>>,
    shared_playlist_id: i32,
) -> Result<bool, String> {
    match app.mark_shared_playlist_viewed(shared_playlist_id).await {
        Ok(status) => {
            if status.is_success() {
                Ok(true)
            } else {
                Err(format!("Failed to mark playlist as viewed, status: {}", status))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
