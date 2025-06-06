use serde::Deserialize;
use tauri_plugin_http::reqwest;

use crate::backend::backend::BackendClient;
/*
    Cette structure est une structure Généré par Copilot pour l'exemple
*/
#[derive(Debug, Deserialize)]
pub struct DeezerAuthResponse {
    pub _access_token: String,
    pub _expires: u64,
}

pub struct DeezerClient {
    _access_token: Option<String>,
    backend_client: BackendClient,
}

impl DeezerClient {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self {
            _access_token: None,
            backend_client: BackendClient::new(app_handle),
        }
    }

    pub async fn _authenticate(
        &mut self,
        app_id: &str,
        app_secret: &str,
        code: &str,
    ) -> Result<(), reqwest::Error> {
        let _url = format!(
            "https://connect.deezer.com/oauth/access_token.php?app_id={}&secret={}&code={}&output=json",
            app_id, app_secret, code
        );

        self._access_token = Some("aerarzar".to_string()); // Placeholder for the access token

        Ok(())
    }

    pub async fn set_auth_header(&self, token: String) {
        self.backend_client.set_auth_header(token).await;
    }
}
