use crate::backend::SpotifyClient;
use crate::backend::UserService;
use crate::deezer::DeezerClient;
use log::info;
use swaptun_backend::{
    CreateUserRequest, GetAuthorizationUrlRequest, LoginEmailRequest, LoginRequest, LoginResponse,
    SpotifyUrlResponse, VerifyTokenRequest, VerifyTokenResponse,
};
use tauri::async_runtime::Mutex;
use tauri::http::StatusCode;
use tauri::AppHandle;
use tauri::Manager;
use tauri::Url;
use tauri_plugin_http::reqwest;
use tauri_plugin_oauth::start;
use tauri_plugin_oauth::{start_with_config, OauthConfig};
pub struct App {
    _app_handle: AppHandle,
    _spotify_client: Mutex<SpotifyClient>,
    _deezer_client: Mutex<DeezerClient>,
    user_service: UserService,
    spotify_url_port: u16,
    ready: Mutex<bool>,
}

impl App {
    pub fn new(app_handle: AppHandle) -> Self {
        let webview_window = app_handle.get_webview_window("main").unwrap();
        let cloned_webview_window = webview_window.clone();
        // A utiliser lors que spotify aura un redirect_uri dynamique qui sera bien testé
        /*
        let spotify_url_port = start(move |url| {
            info!("start_server for spotifyredirect_uri: {}", url);
            let homepage_url = "http://tauri.localhost/homepage";
            cloned_app_handle
                .get_webview_window("main")
                .unwrap()
                .navigate(Url::parse(homepage_url).unwrap())
                .unwrap();
        })
        .unwrap();
        */
        let config = OauthConfig {
            ports: Some(vec![8000]),
            response: None,
        };

        start_with_config(config, move |url| {
            // Because of the unprotected localhost port, you must verify the URL here.
            // Preferebly send back only the token, or nothing at all if you can handle everything else in Rust.
            info!("start_server redirect_uri: {}", url);
            let homepage_url = "http://tauri.localhost/homepage";
            cloned_webview_window
                .navigate(Url::parse(homepage_url).unwrap())
                .unwrap();
        })
        .map_err(|err| err.to_string());

        let spotify_url_port = 8000;
        let deezer_url_port = start(move |url| {
            info!("start_server for deezer redirect_uri: {}", url);
            let homepage_url = "http://tauri.localhost/homepage";
            webview_window
                .navigate(Url::parse(homepage_url).unwrap())
                .unwrap();
        })
        .unwrap();

        info!("spotify_url_port: {}", spotify_url_port);
        info!("deezer_url_port: {}", deezer_url_port);
        Self {
            _app_handle: app_handle.clone(),
            _spotify_client: SpotifyClient::new(app_handle.clone()).into(),
            _deezer_client: DeezerClient::new().into(),
            user_service: UserService::new(app_handle.clone()),
            spotify_url_port,
            ready: Mutex::new(false),
        }
    }
    pub fn _app_handle(&self) -> &AppHandle {
        &self._app_handle
    }

    pub async fn get_autorization_url_spotify(
        &self,
    ) -> Result<SpotifyUrlResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut spotify_client = self._spotify_client.lock().await;
        let req = GetAuthorizationUrlRequest {
            port: self.spotify_url_port,
        };
        spotify_client.get_auth_url(req).await
    }

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

    pub async fn is_app_ready(&self) -> Result<bool, String> {
        let ready = self.ready.lock().await;
        Ok(*ready)
    }

    pub async fn set_app_ready(&self) {
        let mut ready = self.ready.lock().await;
        *ready = true;
    }
}
