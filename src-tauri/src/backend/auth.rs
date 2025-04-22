use crate::backend::backend::BackendClient;
use serde::{Deserialize, Serialize};
use tauri::AppHandle;

#[derive(Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterResponse {
    pub message: String,
}

pub struct AuthService {
    backend_client: BackendClient,
}

impl AuthService {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            backend_client: BackendClient::new(app_handle),
        }
    }

    pub async fn login(
        &self,
        username: &str,
        password: &str,
    ) -> Result<LoginResponse, Box<dyn std::error::Error + Send + Sync>> {
        let request = LoginRequest {
            username: username.to_string(),
            password: password.to_string(),
        };

        self.backend_client
            .post_with_return::<LoginResponse, _>(
                "auth/login",
                serde_json::to_vec(&request).unwrap(),
            )
            .await
    }
}
