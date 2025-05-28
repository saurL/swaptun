use std::fmt::Debug;

use log::{error, info};

use serde::de::DeserializeOwned;
use std::error::Error;
use tauri::{AppHandle, Emitter};
use tauri_plugin_http::reqwest::{Body, Client, RequestBuilder, Response, StatusCode};
use tauri_plugin_pinia::ManagerExt;

pub struct BackendClient {
    client: Client,
    base_url: String,
    app_handle: AppHandle,
}

impl BackendClient {
    pub fn new(app_handle: AppHandle) -> Self {
        let dev_url = &app_handle.config().build.dev_url;
        let host = match dev_url {
            Some(dev_url) => dev_url.host_str().unwrap_or("localhost").to_string(),
            None => "localhost".to_string(),
        };
        let port: &'static str = "8000";
        let base_url = format!("http://{}:{}/api", host, port);
        info!("Backend URL: {}", base_url);

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("Failed to build reqwest client");

        info!("client instance created:");
        let instance = Self {
            client,
            base_url,
            app_handle,
        };
        info!("BackendClient instance created:");
        instance
    }

    async fn check_internet_connection(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        // On essaie de se connecter à un serveur fiable (Google DNS)
        match self.client.get("https://8.8.8.8").send().await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("No internet connection: {}", e);
                self.app_handle
                    .emit("no_internet_connection", "".to_string())
                    .unwrap();

                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotConnected,
                    "No internet connection",
                )))
            }
        }
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<T, Box<dyn Error + Send + Sync>> {
        // Vérifier la connexion Internet avant d'effectuer la requête

        let url = format!("{}/{}", self.base_url, endpoint);
        info!("GET URL: {}", url);
        let get = self.client.get(&url);

        let response = self.send(get).await?;
        info!("GET Response: {:?}", response);
        if response.status().is_client_error() || response.status().is_server_error() {
            let tt = response.text().await?;
            let error_message = tt;
            error!("GET Response Error: {}", error_message);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                error_message,
            )));
        }
        let json: T = response.json().await?;
        Ok(json)
    }

    pub async fn post<U>(
        &self,
        endpoint: &str,
        body: U,
    ) -> Result<StatusCode, Box<dyn Error + Send + Sync>>
    where
        U: Into<Body> + Debug,
    {
        // Vérifier la connexion Internet avant d'effectuer la requête

        let response = self._post(endpoint, body).await?;
        let status: StatusCode = response.status();
        Ok(status)
    }

    async fn _post<U>(
        &self,
        endpoint: &str,
        body: U,
    ) -> Result<Response, Box<dyn Error + Send + Sync>>
    where
        U: Into<Body> + Debug,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        info!("POST URL: {}", url);
        info!("POST Body: {:?}", body);
        let post = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(body);
        let response = self.send(post).await?;
        info!("POST Response: {:?}", response);
        let status = response.status();
        info!("POST Response Status: {}", status);
        if status.is_client_error() {
            let tt = response.text().await?;

            let error_message = tt;
            error!("POST Response Error: {}", error_message);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                error_message,
            )));
        }
        Ok(response)
    }

    pub async fn post_with_return<T, U>(
        &self,
        endpoint: &str,
        body: U,
    ) -> Result<T, Box<dyn Error + Send + Sync>>
    where
        U: Into<Body> + Debug,
        T: DeserializeOwned + Debug,
    {
        // Vérifier la connexion Internet avant d'effectuer la requête

        let response = self._post(endpoint, body).await?;
        let json = response.json::<T>().await?;
        info!("POST Response Body: {:?}", json);
        Ok(json)
    }

    async fn send(
        &self,
        mut request: RequestBuilder,
    ) -> Result<Response, Box<dyn Error + Send + Sync>> {
        self.check_internet_connection().await?;

        request = self.add_authorization_header(request).await;
        let response = match request.send().await {
            Ok(resp) => resp,
            Err(e) => {
                if e.is_timeout() {
                    error!("Request timed out after 10 seconds");
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::TimedOut,
                        "Request timed out after 10 seconds",
                    )));
                } else if e.is_connect() {
                    error!("Failed to connect to server");
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::ConnectionRefused,
                        "Failed to connect to server",
                    )));
                } else {
                    error!("Request failed: {}", e);
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Request failed: {}", e),
                    )));
                }
            }
        };

        if response.status().is_client_error() || response.status().is_server_error() {
            let tt = response.text().await?;
            let error_message = tt;
            error!("Response Error: {}", error_message);
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                error_message,
            )));
        }
        Ok(response)
    }

    async fn add_authorization_header(&self, request: RequestBuilder) -> RequestBuilder {
        if let Some(token_value) = self
            .app_handle
            .pinia()
            .get("identification_token", "identification_token")
        {
            if let Some(token) = token_value.as_str() {
                info!("Token: {:?}", token);
                let auth = format!("Bearer {}", token);
                return request.header("Authorization", auth);
            }
        }
        request
    }

    pub async fn get_with_body<T: DeserializeOwned, U: Into<Body> + Debug>(
        &self,
        endpoint: &str,
        body: U,
    ) -> Result<T, Box<dyn Error + Send + Sync>> {
        // Vérifier la connexion Internet avant d'effectuer la requête

        let url = format!("{}/{}", self.base_url, endpoint);
        let get: RequestBuilder = self
            .client
            .get(&url)
            .header("Content-Type", "application/json");
        let get_with_body = get.body(body);
        let response = self.send(get_with_body).await?;
        let json: T = response.json().await?;
        Ok(json)
    }
}
