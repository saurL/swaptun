use crate::backend::backend::BackendClient;
use crate::validators::user_validators::{validate_no_spaces, validate_password};
use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_http::reqwest::StatusCode;
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(
        min = 3,
        max = 200,
        message = "Username must be between 3 and 50 characters"
    ))]
    #[validate(custom(function = validate_no_spaces))]
    pub username: String,

    #[validate(custom(function = validate_password))]
    pub password: String,

    #[validate(length(min = 3, max = 20, message = "First name cannot exceed 20 characters"))]
    pub first_name: String,

    #[validate(length(min = 3, max = 20, message = "Last name cannot exceed 20 characters"))]
    pub last_name: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Deserialize, Validate, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user_id: i32,
    pub username: String,
    pub role: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterResponse {
    pub succed: bool,
}

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
}
