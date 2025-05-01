use log::info;
use rspotify::{prelude::*, scopes, AuthCodeSpotify, Credentials, OAuth};
use serde::Deserialize;
use tauri_plugin_http::reqwest;
/*
    Cette structure est une structure Généré par Copilot pour l'exemple
*/

#[derive(Debug, Deserialize)]
pub struct SpotifyAuthResponse {
    pub _access_token: String,
    pub _token_type: String,
    pub _expires_in: u64,
}

pub struct SpotifyClient {
    _access_token: Option<String>,
}

impl SpotifyClient {
    pub fn new() -> Self {
        Self {
            _access_token: None,
        }
    }

    pub async fn authenticate(
        &mut self,
        client_id: &str,
        client_secret: &str,
    ) -> Result<(), reqwest::Error> {
        let creds = Credentials::new(client_id, client_secret);

        let scopes = scopes!(
            "user-read-email",
            "user-read-private",
            "user-top-read",
            "user-read-recently-played",
            "user-follow-read",
            "user-library-read",
            "user-read-currently-playing",
            "user-read-playback-state",
            "user-read-playback-position",
            "playlist-read-collaborative",
            "playlist-read-private",
            "user-follow-modify",
            "user-library-modify",
            "user-modify-playback-state",
            "playlist-modify-public",
            "playlist-modify-private",
            "ugc-image-upload"
        );
        let oauth = OAuth::from_env(scopes).unwrap();

        let spotify = AuthCodeSpotify::new(creds, oauth);

        let url = spotify.get_authorize_url(false).unwrap();
        // This function requires the `cli` feature enabled.
        spotify.prompt_for_token(&url).await.unwrap();

        let token = spotify.token.lock().await.unwrap();
        info!("user token: {:?}", token);
        Ok(())
    }
}
