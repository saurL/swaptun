use crate::backend::backend::BackendClient;
use swaptun_backend::{
    CreateUserRequest, LoginEmailRequest, LoginRequest, LoginResponse, VerifyTokenRequest,
    VerifyTokenResponse,
};
use tauri::AppHandle;
use tauri_plugin_http::reqwest::StatusCode;
pub struct UserService {
    backend_client: BackendClient,
}

impl UserService {
    pub fn new(app_handle: AppHandle) -> Self {
        Self {
            backend_client: BackendClient::new(app_handle),
        }
    }

    pub async fn register(
        &self,
        request: CreateUserRequest,
    ) -> Result<StatusCode, Box<dyn std::error::Error + Send + Sync>> {
        self.backend_client
            .post("register", serde_json::to_string(&request).unwrap())
            .await
    }

    pub async fn login(
        &self,
        login_request: LoginRequest,
    ) -> Result<LoginResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.backend_client
            .post_with_return::<LoginResponse, _>(
                "auth/login",
                serde_json::to_string(&login_request).unwrap(),
            )
            .await
    }

    pub async fn login_email(
        &self,
        login_request: LoginEmailRequest,
    ) -> Result<LoginResponse, Box<dyn std::error::Error + Send + Sync>> {
        self.backend_client
            .post_with_return::<LoginResponse, _>(
                "auth/login_email",
                serde_json::to_string(&login_request).unwrap(),
            )
            .await
    }

    pub async fn verify_token(
        &self,
        request: VerifyTokenRequest,
    ) -> Result<VerifyTokenResponse, Box<dyn std::error::Error + Send + Sync>> {
        let response = self
            .backend_client
            .post_with_return::<VerifyTokenResponse, _>(
                "auth/verify_token",
                serde_json::to_string(&request).unwrap(),
            )
            .await?;
        Ok(response)
    }


}
