use crate::backend::backend::BackendClient;
use log::info;
use swaptun_backend::{
    AddFriendRequest, CreateUserRequest, ForgotPasswordRequest, GetUsersRequest, LoginEmailRequest,
    LoginRequest, LoginResponse, RemoveFriendRequest, ResetPasswordRequest, UserBean,
    VerifyTokenRequest, VerifyTokenResponse,
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

    pub async fn forgot_password(
        &self,
        req: ForgotPasswordRequest,
    ) -> Result<StatusCode, Box<dyn std::error::Error + Send + Sync>> {
        self.backend_client
            .post("auth/forgot-password", serde_json::to_string(&req).unwrap())
            .await
    }

    pub async fn reset_password(
        &self,
        token: String,
        request: ResetPasswordRequest,
    ) -> Result<StatusCode, Box<dyn std::error::Error + Send + Sync>> {
        self.backend_client.add_temporary_token(token).await;
        self.backend_client
            .post(
                "users/reset-password",
                serde_json::to_string(&request).unwrap(),
            )
            .await
    }
    pub async fn get_users(
        &self,
        request: GetUsersRequest,
    ) -> Result<Vec<UserBean>, Box<dyn std::error::Error + Send + Sync>> {
        info!("Fetching users with request: {:?}", request);
        self.backend_client
            .get_with_body("users", serde_json::to_string(&request).unwrap())
            .await
    }

    pub async fn get_friends(
        &self,
    ) -> Result<Vec<UserBean>, Box<dyn std::error::Error + Send + Sync>> {
        self.backend_client.get("users/friends").await
    }

    pub async fn add_friend(
        &self,
        request: AddFriendRequest,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.backend_client
            .post(
                "users/friends/add",
                serde_json::to_string(&request).unwrap(),
            )
            .await?;
        Ok(())
    }

    pub async fn remove_friend(
        &self,
        request: RemoveFriendRequest,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.backend_client
            .post(
                "users/friends/remove",
                serde_json::to_string(&request).unwrap(),
            )
            .await?;
        Ok(())
    }
}
