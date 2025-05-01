use crate::backend::UserService;
use crate::deezer::DeezerClient;
// use crate::spotify::SpotifyClient;
use swaptun_backend::{
    CreateUserRequest, LoginEmailRequest, LoginRequest, LoginResponse, VerifyTokenRequest,
    VerifyTokenResponse,
};
use tauri::async_runtime::Mutex;
use tauri::http::StatusCode;
use tauri::AppHandle;
use tauri_plugin_http::reqwest;
pub struct App {
    _app_handle: AppHandle,
    // _spotify_client: Mutex<SpotifyClient>,
    _deezer_client: Mutex<DeezerClient>,
    user_service: UserService,
}

impl App {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            _app_handle: app_handle.clone(),
            // _spotify_client: SpotifyClient::new().into(),
            _deezer_client: DeezerClient::new().into(),
            user_service: UserService::new(app_handle.clone()),
        }
    }
    pub fn _app_handle(&self) -> &AppHandle {
        &self._app_handle
    }
    /*
    pub async fn authenticate_spotify(
        &self,
        client_id: &str,
        client_secret: &str,
    ) -> Result<(), reqwest::Error> {
        let mut spotify_client = self._spotify_client.lock().await;
        spotify_client.authenticate(client_id, client_secret).await
    }
    */
    pub async fn _authenticate_deezer(
        &self,
        app_id: &str,
        app_secret: &str,
        code: &str,
    ) -> Result<(), reqwest::Error> {
        let mut deezer_client = self._deezer_client.lock().await;
        deezer_client._authenticate(app_id, app_secret, code).await
    }

    pub async fn register(
        &self,
        request: CreateUserRequest,
    ) -> Result<StatusCode, Box<dyn std::error::Error + Send + Sync>> {
        self.user_service.register(request).await
    }

    pub async fn login(
        &self,
        request: LoginRequest,
    ) -> Result<LoginResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.user_service.login(request).await
    }

    pub async fn login_email(
        &self,
        request: LoginEmailRequest,
    ) -> Result<LoginResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.user_service.login_email(request).await
    }

    pub async fn verify_token(
        &self,
        request: VerifyTokenRequest,
    ) -> Result<VerifyTokenResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.user_service.verify_token(request).await
    }
}
