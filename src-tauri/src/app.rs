use std::sync::Arc;

use crate::backend::DeezerClient;
use crate::backend::PlaylistService;
use crate::backend::SpotifyClient;
use crate::backend::UserService;

use log::error;
use log::info;
use swaptun_backend::AddTokenRequest;
use swaptun_backend::GetPlaylistResponse;
use swaptun_backend::PlaylistOrigin;
use swaptun_backend::{
    CreateUserRequest, GetAuthorizationUrlRequest, GetPlaylistsParams, LoginEmailRequest,
    LoginRequest, LoginResponse, SpotifyUrlResponse, VerifyTokenRequest, VerifyTokenResponse,
};
use tauri::async_runtime::spawn;
use tauri::async_runtime::Mutex;
use tauri::http::StatusCode;
use tauri::AppHandle;
use tauri::Emitter;
use tauri::Url;
use tauri_plugin_oauth::start_with_config;
// use tauri_plugin_oauth::start;
use tauri_plugin_oauth::{start, OauthConfig};
pub struct App {
    app_handle: AppHandle,
    spotify_client: SpotifyClient,
    _deezer_client: DeezerClient,
    user_service: UserService,
    playlist_service: PlaylistService,
    spotify_url_port: Mutex<Option<u16>>,
    deezer_url_port: Mutex<Option<u16>>,
    ready: Mutex<bool>,
}

impl App {
    pub async fn new(app_handle: AppHandle) -> Arc<Self> {
        let instance = Self {
            app_handle: app_handle.clone(),
            spotify_client: SpotifyClient::new(app_handle.clone()),
            _deezer_client: DeezerClient::new(app_handle.clone()),
            user_service: UserService::new(app_handle.clone()),
            playlist_service: PlaylistService::new(app_handle.clone()),
            spotify_url_port: Mutex::new(None),
            deezer_url_port: Mutex::new(None),
            ready: Mutex::new(false),
        };
        let instance = Arc::new(instance);
        instance.start_spotify_oauth_server().await;
        instance.start_deezer_oauth_server().await;
        instance
    }
    pub fn app_handle(&self) -> &AppHandle {
        &self.app_handle
    }

    pub async fn get_autorization_url_spotify(
        &self,
    ) -> Result<SpotifyUrlResponse, Box<dyn std::error::Error + Send + Sync>> {
        match *self.spotify_url_port.lock().await {
            Some(port) => {
                let req = GetAuthorizationUrlRequest { port };
                self.spotify_client.get_auth_url(req).await
            }
            None => Err("No port found for spotify oauth server".into()),
        }
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

    pub async fn start_spotify_oauth_server(self: &Arc<Self>) {
        let config = OauthConfig {
            ports: Some(vec![8000]),
            redirect_uri: Some("http://tauri.localhost/homepage".into()),
            response: None,
        };
        let instance = self.clone();

        match start_with_config(config, move |url: String| {
            info!("start_server spotify modifié redirect_uri: {}", url);

            let parsed_url = Url::parse(&url).expect("URL invalide");
            let params = parsed_url.query_pairs();
            let mut code = None;
            for (key, value) in params {
                match key.as_ref() {
                    "code" => code = Some(value.to_string()),
                    _ => {}
                }
            }
            let cloned_instance = instance.clone();
            if let Some(code) = code {
                spawn(async move {
                    cloned_instance.send_spotify_token(code.to_string()).await;
                    info!("token send");
                    cloned_instance.import_playlist_backend_request().await;
                    info!("importing playlist");
                    match cloned_instance.get_playlists_spotify().await {
                        Ok(playlists) => {
                            info!("playlist imported");
                            cloned_instance
                                .app_handle
                                .emit("spotify_playlists", playlists);
                        }
                        Err(e) => {
                            error!("Error getting playlists: {}", e);
                        }
                    }
                });
            }

            // Affichage des résultats
        }) {
            Ok(port) => {
                *self.spotify_url_port.lock().await = Some(port);
                info!("spotify_url_port: {}", port);
            }
            Err(e) => {
                error!("Error starting spotify oauth server: {}", e);
            }
        };
    }

    pub async fn start_deezer_oauth_server(self: &Arc<Self>) {
        let config = OauthConfig {
            ports: Some(vec![8001]),
            redirect_uri: Some("http://tauri.localhost/homepage".into()),
            response: None,
        };
        match start_with_config(config, move |url| {
            info!("start_server deezer redirect_uri: {}", url);
        }) {
            Ok(port) => {
                *self.deezer_url_port.lock().await = Some(port);
                info!("deezer_url_port: {}", port);
            }
            Err(e) => {
                error!("Error starting deezer oauth server: {}", e);
            }
        };
    }

    pub async fn send_spotify_token(self: &Self, token: String) {
        let req = AddTokenRequest {
            token: token.to_string(),
        };
        self.spotify_client.add_token(req).await;
    }

    pub async fn send_token_and_get_playlists(self: &Self, token: String) {
        self.send_spotify_token(token).await;
        match self.get_playlists_spotify().await {
            Ok(playlists) => {
                self.app_handle
                    .emit("spotify_playlists", playlists)
                    .unwrap();
            }
            Err(e) => {
                error!("Error getting playlists: {}", e);
            }
        }
    }

    pub async fn import_playlist_backend_request(
        &self,
    ) -> Result<StatusCode, Box<dyn std::error::Error + Send + Sync>> {
        self.spotify_client.import_playlist_backend_request().await
    }

    pub async fn get_playlists_spotify(
        &self,
    ) -> Result<GetPlaylistResponse, Box<dyn std::error::Error + Send + Sync>> {
        let params = GetPlaylistsParams {
            origin: Some(PlaylistOrigin::Spotify),
        };
        self.playlist_service.get_playlists(params).await
    }

    pub async fn get_playlists_deezer(
        &self,
    ) -> Result<GetPlaylistResponse, Box<dyn std::error::Error + Send + Sync>> {
        let params = GetPlaylistsParams {
            origin: Some(PlaylistOrigin::Deezer),
        };
        self.playlist_service.get_playlists(params).await
    }

    pub async fn set_auth_header(&self, token: String) {
        self.user_service.set_auth_header(token.clone()).await;
        self.spotify_client.set_auth_header(token.clone()).await;
        self._deezer_client.set_auth_header(token.clone()).await;
        self.playlist_service.set_auth_header(token.clone()).await;
        self.user_service.set_auth_header(token).await;
    }
}
