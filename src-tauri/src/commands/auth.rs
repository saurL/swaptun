use std::sync::Arc;

use log::{error, info};
use swaptun_backend::{
    CreateUserRequest, ForgotPasswordRequest, LoginEmailRequest, LoginRequest, LoginResponse,
    ResetPasswordRequest, VerifyTokenRequest,
};
use tauri::{command, Emitter, State};

use crate::app::App;

#[command]
pub async fn register(
    app: State<'_, Arc<App>>,
    username: &str,
    password: &str,
    first_name: &str,
    last_name: &str,
    email: &str,
) -> Result<bool, String> {
    let request = CreateUserRequest {
        username: username.to_string(),
        password: password.to_string(),
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        email: email.to_string(),
    };

    // ICI on peut accèder aux éléments de l'App
    match app.register(request).await {
        Ok(response) => {
            if response.is_success() {
                return Ok(true);
            } else {
                return Err("Failed to register".to_string());
            }
        }
        Err(e) => {
            return Err(format!("Error: {}", e));
        }
    }
}

#[command]
pub async fn logout(app: State<'_, Arc<App>>) -> Result<(), String> {
    // Emit an event to trigger the logout in the frontend
    app.app_handle()
        .emit("logout", "")
        .map_err(|e| format!("Failed to emit logout event: {}", e))?;
    Ok(())
}

#[command]
pub async fn login(
    app: State<'_, Arc<App>>,
    username: &str,
    password: &str,
) -> Result<LoginResponse, String> {
    let request = LoginRequest {
        username: username.to_string(),
        password: password.to_string(),
    };
    // ICI on peut accèder aux éléments de l'App
    match app.login(request).await {
        Ok(response) => Ok(response),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub async fn login_email(
    app: State<'_, Arc<App>>,
    email: &str,
    password: &str,
) -> Result<LoginResponse, String> {
    let request = LoginEmailRequest {
        email: email.to_string(),
        password: password.to_string(),
    };
    // ICI on peut accèder aux éléments de l'App
    match app.login_email(request).await {
        Ok(response) => Ok(response),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub async fn verify_token(app: State<'_, Arc<App>>, token: String) -> Result<bool, String> {
    let request = VerifyTokenRequest {
        token: token.clone(),
    };
    match app.verify_token(request).await {
        Ok(is_valid) => Ok(is_valid.valid),
        Err(e) => Err(e.to_string()),
    }
}

#[command]
pub async fn forgot_password(
    app: State<'_, Arc<App>>,
    request: ForgotPasswordRequest,
) -> Result<bool, String> {
    match app.forgot_password(request).await {
        Ok(status) => {
            if status.is_success() {
                info!("Forgot password request sent successfully");
                Ok(true)
            } else {
                error!("Failed to send forgot password request, status: {}", status);
                Err(format!(
                    "Failed to send forgot password request, status: {}",
                    status
                ))
            }
        }
        Err(e) => {
            error!("Failed to send forgot password request: {}", e);
            Err(e.to_string())
        }
    }
}

#[command]
pub async fn reset_password(
    app: State<'_, Arc<App>>,
    token: String,
    request: ResetPasswordRequest,
) -> Result<bool, String> {
    match app.reset_password(request, token).await {
        Ok(status) => {
            if status.is_success() {
                Ok(true)
            } else {
                error!("Failed to reset password, status: {}", status);
                Err(format!("Failed to reset password, status: {}", status))
            }
        }
        Err(e) => {
            error!("Failed to reset password: {}", e);
            Err(e.to_string())
        }
    }
}
