use serde::Deserialize;

use crate::backend::backend::BackendClient;
use crate::error::AppResult;

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
    _backend_client: BackendClient,
}

impl DeezerClient {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self {
            _access_token: None,
            _backend_client: BackendClient::new(app_handle),
        }
    }

    pub async fn _authenticate(
        &mut self,
        app_id: &str,
        app_secret: &str,
        code: &str,
    ) -> AppResult<()> {
        let _url = format!(
            "https://connect.deezer.com/oauth/access_token.php?app_id={}&secret={}&code={}&output=json",
            app_id, app_secret, code
        );

        self._access_token = Some("aerarzar".to_string()); // Placeholder for the access token

        Ok(())
    }
}
