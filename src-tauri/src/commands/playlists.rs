use std::sync::Arc;

use crate::app::App;

use log::error;
use swaptun_backend::GetPlaylistMusicsResponse;
use swaptun_backend::GetPlaylistResponse;
use swaptun_backend::SendPlaylistRequest;
use swaptun_backend::SendPlaylistResponse;
use swaptun_backend::SharedPlaylistsResponse;
use tauri::{command, State};
#[command]
pub async fn send_playlist(
    app: State<'_, Arc<App>>,
    playlist_id: i32,
    req: SendPlaylistRequest,
) -> Result<SendPlaylistResponse, String> {
    match app.send_playlist(playlist_id, req).await {
        Ok(response) => Ok(response),
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
) -> Result<SharedPlaylistsResponse, String> {
    match app.get_shared_playlists().await {
        Ok(response) => Ok(response),
        Err(e) => {
            error!("Failed to get shared playlists: {}", e);
            Err(e.to_string())
        }
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
                Err(format!(
                    "Failed to mark playlist as viewed, status: {}",
                    status
                ))
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub async fn get_playlist_musics(
    app: State<'_, Arc<App>>,
    playlist_id: i32,
) -> Result<GetPlaylistMusicsResponse, String> {
    match app.get_playlist_musics(playlist_id).await {
        Ok(response) => Ok(response),
        Err(e) => {
            error!("Failed to get playlist musics: {}", e);
            Err(e.to_string())
        }
    }
}
