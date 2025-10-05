use std::sync::Arc;

use crate::backend::AppleService;
use crate::backend::DeezerClient;
use crate::backend::NotificationService;
use crate::backend::PlaylistService;
use crate::backend::SpotifyClient;
use crate::backend::UserService;
use crate::backend::YoutubeClient;
use crate::error::AppResult;
use log::error;
use log::info;

use swaptun_backend::AddFriendRequest;
use swaptun_backend::GetUsersRequest;
use swaptun_backend::RemoveFriendRequest;
use swaptun_backend::UserBean;
use swaptun_backend::{
    AddTokenRequest, CreateUserRequest, ForgotPasswordRequest, GetPlaylistResponse,
    GetPlaylistsParams, LoginEmailRequest, LoginRequest, LoginResponse, PlaylistOrigin,
    RegisterFcmTokenRequest, ResetPasswordRequest, SendPlaylistRequest,
    SendTestNotificationRequest, SharePlaylistRequest, SharedPlaylistsResponse, SpotifyUrlResponse,
    VerifyTokenRequest, VerifyTokenResponse,
};
use tauri::async_runtime::Mutex;
use tauri::http::StatusCode;
use tauri::AppHandle;
use tauri::Emitter;
use tauri::Url;
use tauri_plugin_custom_tabs_manager::{CustomTabsManagerExt, OpenCustomTabSimpleRequest};
use tauri_plugin_musickit::AuthorizationResponse;
use tauri_plugin_musickit::MusicKitExt;
pub struct App {
    app_handle: AppHandle,
    spotify_client: SpotifyClient,
    _deezer_client: DeezerClient,
    user_service: UserService,
    playlist_service: PlaylistService,
    youtube_service: YoutubeClient,
    notification_service: NotificationService,
    apple_service: AppleService,
    ready: Mutex<bool>,
}

impl App {
    pub fn new(app_handle: AppHandle) -> Arc<Self> {
        let instance = Self {
            app_handle: app_handle.clone(),
            spotify_client: SpotifyClient::new(app_handle.clone()),
            _deezer_client: DeezerClient::new(app_handle.clone()),
            user_service: UserService::new(app_handle.clone()),
            playlist_service: PlaylistService::new(app_handle.clone()),
            youtube_service: YoutubeClient::new(app_handle.clone()),
            notification_service: NotificationService::new(app_handle.clone()),
            apple_service: AppleService::new(app_handle.clone()),
            ready: Mutex::new(false),
        };
        let instance = Arc::new(instance);
        instance
    }
    pub fn app_handle(&self) -> &AppHandle {
        &self.app_handle
    }

    pub async fn get_autorization_url_spotify(&self) -> AppResult<SpotifyUrlResponse> {
        self.spotify_client.get_auth_url().await
    }

    pub async fn register(&self, request: CreateUserRequest) -> AppResult<StatusCode> {
        self.user_service.register(request).await
    }

    pub async fn login(&self, request: LoginRequest) -> AppResult<LoginResponse> {
        self.user_service.login(request).await
    }

    pub async fn login_email(&self, request: LoginEmailRequest) -> AppResult<LoginResponse> {
        self.user_service.login_email(request).await
    }

    pub async fn verify_token(
        &self,
        request: VerifyTokenRequest,
    ) -> AppResult<VerifyTokenResponse> {
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

    pub async fn handle_open_url(&self, urls: Vec<Url>) {
        info!("deep link URLs: {:?}", urls);
        if let Some(url) = urls.first() {
            if url.path() == "/open/spotify" {
                self.handle_spotify_auth(url).await
            }
            if url.path() == "/open/youtube" {
                self.handle_youtube_auth(url).await
            }
            if url.path() == "/reset-password" {
                let fullurl = format!("{}?{}", url.path(), url.query().unwrap_or(""));
                self.app_handle
                    .emit("routing", fullurl)
                    .expect("Failed to emit routing event");
            }
        }
    }

    pub async fn handle_spotify_auth(&self, url: &Url) {
        let params = url.query_pairs();
        let mut code = None;
        for (key, value) in params {
            match key.as_ref() {
                "code" => code = Some(value.to_string()),
                _ => {}
            }
        }
        if let Some(code) = code {
            self.send_spotify_token(code.to_string()).await;
            info!("token send");
            match self.import_playlist_backend_request().await {
                Ok(status) => {
                    info!("import playlist backend request status: {:?}", status);
                    if status == StatusCode::OK {
                        info!("playlist imported successfully");
                    } else {
                        error!("Failed to import playlist, status: {:?}", status);
                    }
                }
                Err(e) => {
                    error!("Error importing playlist: {}", e);
                }
            };
            info!("importing playlist");
            match self.get_playlists_spotify().await {
                Ok(playlists) => {
                    info!("playlist imported");
                    match self.app_handle.emit("spotify_playlists", playlists) {
                        Ok(_) => info!("spotify_playlists event emitted"),
                        Err(e) => error!("Error emitting spotify_playlists event: {}", e),
                    };
                }
                Err(e) => {
                    error!("Error getting playlists: {}", e);
                }
            }
        }
    }

    pub async fn handle_youtube_auth(&self, url: &Url) {
        let params = url.query_pairs();
        let mut code = None;
        for (key, value) in params {
            match key.as_ref() {
                "code" => code = Some(value.to_string()),
                _ => {}
            }
        }
        if let Some(code) = code {
            info!("youtube code: {}", code);
            match self.set_youtube_token(code.to_string()).await {
                Ok(status) => {
                    info!("YouTube token set successfully with status: {:?}", status);
                }
                Err(e) => {
                    error!("Error setting YouTube token: {}", e);
                }
            }
        } else {
            error!("No 'code' parameter found in the URL");
        }
    }

    pub async fn send_spotify_token(self: &Self, token: String) {
        let req = AddTokenRequest {
            token: token.to_string(),
        };
        match self.spotify_client.add_token(req).await {
            Ok(status) => {
                info!("Spotify token set successfully with status: {:?}", status);
            }
            Err(e) => {
                error!("Error setting Spotify token: {}", e);
            }
        };
    }

    pub async fn import_playlist_backend_request(&self) -> AppResult<StatusCode> {
        self.spotify_client.import_playlist_backend_request().await
    }

    pub async fn get_playlists_spotify(&self) -> AppResult<GetPlaylistResponse> {
        let params = GetPlaylistsParams {
            origin: Some(PlaylistOrigin::Spotify),
        };
        self.playlist_service.get_playlists(params).await
    }

    pub async fn get_playlists_deezer(&self) -> AppResult<GetPlaylistResponse> {
        let params = GetPlaylistsParams {
            origin: Some(PlaylistOrigin::Deezer),
        };
        self.playlist_service.get_playlists(params).await
    }

    pub async fn get_playlists_youtube(&self) -> AppResult<GetPlaylistResponse> {
        let params = GetPlaylistsParams {
            origin: Some(PlaylistOrigin::YoutubeMusic),
        };
        self.playlist_service.get_playlists(params).await
    }

    pub async fn connect_youtube(&self) -> AppResult<()> {
        let url_response = self.youtube_service.get_auth_url().await;
        match url_response {
            Ok(response) => {
                info!("get_auth_url_youtube response: {}", response.url);
                self.app_handle
                    .custom_tabs_manager()
                    .open_custom_tab_simple(OpenCustomTabSimpleRequest {
                        url: response.url.clone(),
                        try_native_app: true,
                    })
                    .expect("error while opening custom tab");
                Ok(())
            }
            Err(e) => {
                error!("Error getting YouTube auth URL: {}", e);
                Err(e)
            }
        }
    }

    pub async fn set_youtube_token(&self, token: String) -> AppResult<StatusCode> {
        let req = AddTokenRequest {
            token: token.to_string(),
        };
        self.youtube_service.add_token(req).await
    }

    pub async fn set_fcm_token(
        &self,
        register_fcm_token_request: RegisterFcmTokenRequest,
    ) -> AppResult<StatusCode> {
        self.notification_service
            .set_fcm_token(register_fcm_token_request)
            .await
    }

    pub async fn send_test_notification(
        &self,
        notification_request: SendTestNotificationRequest,
    ) -> AppResult<StatusCode> {
        self.notification_service
            .send_test_notification(notification_request)
            .await
    }

    pub async fn send_playlist(
        &self,
        playlist_id: i32,
        req: SendPlaylistRequest,
    ) -> AppResult<StatusCode> {
        self.playlist_service.send_playlist(playlist_id, req).await
    }

    pub async fn share_playlist(&self, playlist_id: i32, user_id: i32) -> AppResult<StatusCode> {
        let req = SharePlaylistRequest { user_id };
        self.playlist_service.share_playlist(playlist_id, req).await
    }

    pub async fn forgot_password(&self, req: ForgotPasswordRequest) -> AppResult<StatusCode> {
        self.user_service.forgot_password(req).await
    }

    pub async fn reset_password(
        &self,
        req: ResetPasswordRequest,
        token: String,
    ) -> AppResult<StatusCode> {
        self.user_service.reset_password(token, req).await
    }

    pub async fn search_users(&self, request: GetUsersRequest) -> AppResult<Vec<UserBean>> {
        self.user_service.get_users(request).await
    }

    pub async fn add_friend(&self, request: AddFriendRequest) -> AppResult<()> {
        self.user_service.add_friend(request).await
    }

    pub async fn remove_friend(&self, request: RemoveFriendRequest) -> AppResult<()> {
        self.user_service.remove_friend(request).await
    }

    pub async fn get_friends(&self) -> AppResult<Vec<UserBean>> {
        self.user_service.get_friends().await
    }

    pub async fn get_shared_playlists(&self) -> AppResult<SharedPlaylistsResponse> {
        self.playlist_service.get_shared_playlists().await
    }

    pub async fn mark_shared_playlist_viewed(
        &self,
        shared_playlist_id: i32,
    ) -> AppResult<StatusCode> {
        self.playlist_service
            .mark_shared_playlist_viewed(shared_playlist_id)
            .await
    }

    pub async fn synchronize_apple_playlists(&self) -> AppResult<StatusCode> {
        self.apple_service.synchronize_playlists().await
    }

    pub async fn connect_apple_music(&self) -> AppResult<AuthorizationResponse> {
        let developer_token = self.apple_service.get_developer_token().await?;
        self.app_handle
            .music_kit()
            .set_developer_token(developer_token.developer_token)?;
        let auth_response = self.app_handle.music_kit().authorize()?;
        info!("Apple Music authorization response: {:?}", auth_response);
        match auth_response.clone().status {
            val if val == "authorized".to_string() => {
                info!("Apple Music authorized");
                let get_user_token_response = self.app_handle.music_kit().get_user_token()?;
                if let Some(token) = get_user_token_response.token {
                    info!("Apple Music user token: {}", token);
                    self.send_user_token_apple_music(token).await?;
                    self.synchronize_apple_playlists().await?;

                    // Récupérer et émettre les playlists Apple Music
                    match self.get_apple_music_playlists().await {
                        Ok(playlists) => {
                            info!("Apple Music playlists retrieved successfully");
                            match self.app_handle.emit("apple_music_playlists", playlists) {
                                Ok(_) => info!("apple_music_playlists event emitted"),
                                Err(e) => {
                                    error!("Error emitting apple_music_playlists event: {}", e)
                                }
                            };
                        }
                        Err(e) => {
                            error!("Error getting Apple Music playlists: {}", e);
                        }
                    }
                } else {
                    error!("Apple Music user token is None");
                }
            }
            val if val == "notAuthorized".to_string() => error!("Apple Music not authorized"),
            _ => error!("Apple Music authorization failed"),
        }
        Ok(auth_response)
    }

    pub async fn send_user_token_apple_music(&self, token: String) -> AppResult<()> {
        let request = AddTokenRequest { token };
        self.apple_service.send_authorization_token(request).await?;
        Ok(())
    }

    pub async fn get_apple_music_playlists(&self) -> AppResult<GetPlaylistResponse> {
        let params = GetPlaylistsParams {
            origin: Some(PlaylistOrigin::AppleMusic),
        };
        let response = self.playlist_service.get_playlists(params).await?;
        Ok(response)
    }

    pub async fn disconnect_spotify(&self) -> AppResult<StatusCode> {
        self.spotify_client.disconnect().await
    }

    pub async fn disconnect_youtube(&self) -> AppResult<StatusCode> {
        self.youtube_service.disconnect().await
    }

    pub async fn disconnect_apple_music(&self) -> AppResult<StatusCode> {
        self.apple_service.disconnect().await
    }
}
