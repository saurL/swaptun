use std::sync::Arc;

use swaptun_backend::GetPlaylistResponse;
use tauri::{command, State};

use crate::app::App;

#[command]
pub async fn get_playlists_deezer(app: State<'_, Arc<App>>) -> Result<GetPlaylistResponse, String> {
    match app.get_playlists_deezer().await {
        Ok(response) => Ok(response),
        Err(e) => Err(e.to_string()),
    }
}
