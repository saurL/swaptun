use crate::backend::backend::BackendClient;
use swaptun_backend::{GetPlaylistResponse, GetPlaylistsParams};
use tauri::AppHandle;

pub struct PlaylistService {
    backend_client: BackendClient,
}

impl PlaylistService {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            backend_client: BackendClient::new(app_handle),
        }
    }

    pub async fn get_playlists(
        &self,
        params: GetPlaylistsParams,
    ) -> Result<GetPlaylistResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.backend_client
            .get_with_body::<GetPlaylistResponse, _>(
                "playlists",
                serde_json::to_string(&params).unwrap(),
            )
            .await
    }
}
