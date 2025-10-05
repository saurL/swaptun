use crate::error::{AppError, AppResult};
use crate::models::ErrorNotification;
use core::str;
use log::{debug, error, info, warn};
use rspotify::model::error;
use serde::de::DeserializeOwned;
use std::fmt::Debug;
use tauri::{async_runtime::Mutex, AppHandle, Emitter};
use tauri_plugin_http::reqwest::{Body, Client, RequestBuilder, Response, StatusCode};
use tauri_plugin_pinia::ManagerExt;
use tokio::time::sleep;
const DEFAULT_PORT: &str = "8000";

pub struct BackendClient {
    client: Client,
    base_url: String,
    app_handle: AppHandle,
    next_request_token: Mutex<Option<String>>,
}

impl BackendClient {
    pub fn new(app_handle: AppHandle) -> Self {
        let base_url = Self::determine_base_url(&app_handle);
        info!("Backend URL initialized: {}", base_url);

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            base_url,
            app_handle,
            next_request_token: Mutex::new(None),
        }
    }

    /// Determine the backend URL based on build configuration
    fn determine_base_url(app_handle: &AppHandle) -> String {
        let host = if cfg!(debug_assertions) {
            Self::get_dev_host(app_handle)
        } else {
            "localhost".to_string()
        };

        format!("http://{}:{}/api", host, DEFAULT_PORT)
    }

    /// Get development host based on target platform
    fn get_dev_host(app_handle: &AppHandle) -> String {
        if cfg!(target_os = "ios") {
            "192.168.50.54".to_string()
        } else if cfg!(target_os = "android") {
            app_handle
                .config()
                .build
                .dev_url
                .as_ref()
                .and_then(|url| url.host_str().map(|s| s.to_string()))
                .unwrap_or_else(|| "localhost".to_string())
        } else {
            "localhost".to_string()
        }
    }

    /// Check internet connectivity with retries
    async fn check_connectivity(&self) -> AppResult<()> {
        let max_retries = 30;
        let mut retry_count = 0;

        loop {
            match self.client.get("https://8.8.8.8").send().await {
                Ok(_) => {
                    if retry_count > 0 {
                        info!("Internet connection restored after {} retries", retry_count);
                    }
                    return Ok(());
                }
                Err(e) => {
                    warn!(
                        "No internet connection (attempt {}): {}",
                        retry_count + 1,
                        e
                    );
                    if retry_count == 0 {
                        let error = ErrorNotification::network_error("No internet connection");
                        let _ = self.app_handle.emit("error_notification", error);
                    }

                    retry_count += 1;
                    if retry_count >= max_retries {
                        return Err(AppError::Network(
                            "No internet connection after maximum retries".to_string(),
                        ));
                    }

                    sleep(std::time::Duration::from_secs(2)).await;
                }
            }
        }
    }

    /// Generic GET request
    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> AppResult<T> {
        let url = format!("{}/{}", self.base_url, endpoint);
        debug!("GET {}", url);

        let request = self.client.get(&url);
        let response = self.send_request(request).await?;

        self.handle_response(response).await
    }

    /// Generic POST request returning status code
    pub async fn post<U>(&self, endpoint: &str, body: U) -> AppResult<StatusCode>
    where
        U: Into<Body> + Debug,
    {
        let response = self.post_internal(endpoint, body).await?;
        Ok(response.status())
    }

    /// Generic POST request with response body
    pub async fn post_with_return<T, U>(&self, endpoint: &str, body: U) -> AppResult<T>
    where
        U: Into<Body> + Debug,
        T: DeserializeOwned + Debug,
    {
        let response = self.post_internal(endpoint, body).await?;
        self.handle_response(response).await
    }

    /// Internal POST implementation
    async fn post_internal<U>(&self, endpoint: &str, body: U) -> AppResult<Response>
    where
        U: Into<Body> + Debug,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        debug!("POST {} with body: {:?}", url, body);

        let request = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(body);

        self.send_request(request).await
    }

    /// GET request with body (non-standard but used in some APIs)
    pub async fn get_with_body<T, U>(&self, endpoint: &str, body: U) -> AppResult<T>
    where
        T: DeserializeOwned,
        U: Into<Body> + Debug,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        debug!("GET {} with body: {:?}", url, body);

        let request = self
            .client
            .get(&url)
            .header("Content-Type", "application/json")
            .body(body);

        let response = self.send_request(request).await?;
        self.handle_response(response).await
    }

    /// Generic DELETE request returning status code
    pub async fn delete(&self, endpoint: &str) -> AppResult<StatusCode> {
        let url = format!("{}/{}", self.base_url, endpoint);
        debug!("DELETE {}", url);

        let request = self.client.delete(&url);
        let response = self.send_request(request).await?;
        Ok(response.status())
    }

    /// Send request with authentication and error handling
    async fn send_request(&self, request: RequestBuilder) -> AppResult<Response> {
        self.check_connectivity().await?;

        let request = self.add_authorization_header(request).await;

        match request.send().await {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(response)
                } else {
                    let status = response.status();
                    let error_text = response.text().await.unwrap_or_default();
                    error!("Request failed with status {}: {}", status, error_text);

                    let error_msg = if error_text.is_empty() {
                        format!("Server error: {}", status)
                    } else {
                        error_text.clone()
                    };
                    let error = ErrorNotification::server_error("Server error");
                    let _ = self.app_handle.emit("error_notification", error);

                    if status.is_client_error() {
                        Err(AppError::Backend(error_text))
                    } else {
                        Err(AppError::Internal(error_text))
                    }
                }
            }
            Err(e) => {
                if e.is_timeout() {
                    Err(AppError::Network("Request timeout".to_string()))
                } else if e.is_connect() {
                    Err(AppError::Network("Connection refused".to_string()))
                } else {
                    Err(AppError::Network(e.to_string()))
                }
            }
        }
    }

    /// Handle response and deserialize JSON
    async fn handle_response<T: DeserializeOwned>(&self, response: Response) -> AppResult<T> {
        response
            .json::<T>()
            .await
            .map_err(|e| AppError::Internal(format!("Failed to parse response: {}", e)))
    }

    /// Add authorization header to request
    async fn add_authorization_header(&self, request: RequestBuilder) -> RequestBuilder {
        // Try temporary token first, then pinia token
        let mut temp_token = self.next_request_token.lock().await;

        if let Some(token) = temp_token.take() {
            debug!("Using temporary auth token");
            return request.header("Authorization", format!("Bearer {}", token));
        }

        if let Some(token) = self.app_handle.pinia().get("user", "token") {
            let string_token = token.as_str().unwrap_or_default();
            return request.header("Authorization", format!("Bearer {}", string_token));
        }

        request
    }

    /// Set a temporary token for the next request
    pub async fn set_temporary_token(&self, token: String) {
        let mut temp_token = self.next_request_token.lock().await;
        *temp_token = Some(token);
    }
}
