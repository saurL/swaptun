use serde::Deserialize;
use tauri_plugin_http::reqwest;
/*
    Cette structure est une structure Généré par Copilot pour l'exemple
*/
#[derive(Debug, Deserialize)]
pub struct DeezerAuthResponse {
    pub access_token: String,
    pub expires: u64,
}

pub struct DeezerClient {
    access_token: Option<String>,
}

impl DeezerClient {
    pub fn new() -> Self {
        Self { access_token: None }
    }

    pub async fn authenticate(
        &mut self,
        app_id: &str,
        app_secret: &str,
        code: &str,
    ) -> Result<(), reqwest::Error> {
        let url = format!(
            "https://connect.deezer.com/oauth/access_token.php?app_id={}&secret={}&code={}&output=json",
            app_id, app_secret, code
        );

        self.access_token = Some("aerarzar".to_string()); // Placeholder for the access token

        Ok(())
    }
}
