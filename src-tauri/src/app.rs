use crate::deezer::DeezerClient;
use crate::spotify::SpotifyClient;
use tauri::async_runtime::Mutex;
use tauri_plugin_http::reqwest;

use tauri::AppHandle;
pub struct App {
    app_handle: AppHandle,
    spotify_client: Mutex<SpotifyClient>,
    deezer_client: Mutex<DeezerClient>,
}

impl App {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            app_handle,
            spotify_client: SpotifyClient::new().into(),
            deezer_client: DeezerClient::new().into(),
        }
    }
    pub fn app_handle(&self) -> &AppHandle {
        &self.app_handle
    }

    pub async fn authenticate_spotify(
        &self,
        client_id: &str,
        client_secret: &str,
        code: &str,
        redirect_uri: &str,
    ) -> Result<(), reqwest::Error> {
        let mut spotify_client = self.spotify_client.lock().await;
        spotify_client
            .authenticate(client_id, client_secret, code, redirect_uri)
            .await
    }
    pub async fn authenticate_deezer(
        &self,
        app_id: &str,
        app_secret: &str,
        code: &str,
    ) -> Result<(), reqwest::Error> {
        let mut deezer_client = self.deezer_client.lock().await;
        deezer_client.authenticate(app_id, app_secret, code).await
    }
}
