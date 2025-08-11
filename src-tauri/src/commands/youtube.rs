use std::sync::Arc;

use swaptun_backend::GetPlaylistResponse;
use tauri::{command, State};

use crate::app::App;

#[command]
pub async fn connect_youtube(app: State<'_, Arc<App>>) -> Result<(), String> {
    match app.connect_youtube().await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub async fn get_playlists_youtubemusic(
    app: State<'_, Arc<App>>,
) -> Result<GetPlaylistResponse, String> {
    match app.get_playlists_youtube().await {
        Ok(response) => Ok(response),
        Err(e) => Err(e.to_string()),
    }
}
