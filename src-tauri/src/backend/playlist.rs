use crate::backend::backend::BackendClient;
use swaptun_backend::{GetPlaylistResponse, GetPlaylistsParams, SendPlaylistRequest};
use tauri::{http::StatusCode, AppHandle};

pub struct PlaylistService {
    backend_client: BackendClient,
    base_url: String,
}

impl PlaylistService {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            backend_client: BackendClient::new(app_handle),
            base_url: "playlists".into(),
        }
    }

    pub async fn get_playlists(
        &self,
        params: GetPlaylistsParams,
    ) -> Result<GetPlaylistResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.backend_client
            .get_with_body::<GetPlaylistResponse, _>(
                &self.base_url,
                serde_json::to_string(&params).unwrap(),
            )
            .await
    }

    pub async fn send_playlist(
        &self,
        playlist_id: i32,
        req: SendPlaylistRequest,
    ) -> Result<StatusCode, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!("{}/{}/send", self.base_url, playlist_id);
        self.backend_client
            .post(&url, serde_json::to_string(&req).unwrap())
            .await
    }
}
