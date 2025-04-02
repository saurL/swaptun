use serde::Deserialize;
use tauri_plugin_http::reqwest;

/*
    Cette structure est une structure Généré par Copilot pour l'exemple
*/

#[derive(Debug, Deserialize)]
pub struct SpotifyAuthResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
}

pub struct SpotifyClient {
    access_token: Option<String>,
}

impl SpotifyClient {
    pub fn new() -> Self {
        Self { access_token: None }
    }

    pub async fn authenticate(
        &mut self,
        client_id: &str,
        client_secret: &str,
        code: &str,
        redirect_uri: &str,
    ) -> Result<(), reqwest::Error> {
        let url = "https://accounts.spotify.com/api/token";
        let params = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", redirect_uri),
            ("client_id", client_id),
            ("client_secret", client_secret),
        ];
        self.access_token = Some("TOKEN".to_string()); // Placeholder for the access token

        Ok(())
    }
}
