use std::sync::Arc;

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
