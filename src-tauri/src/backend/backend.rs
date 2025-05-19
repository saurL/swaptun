use std::fmt::Debug;

use log::{error, info};

use serde::de::DeserializeOwned;
use std::error::Error;
use tauri::AppHandle;
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
        let port = "8000";
        let base_url = format!("http://{}:{}/api", host, port);
        info!("Backend URL: {}", base_url);

        let client = Client::new();

        info!("client instance created:");
        let instance = Self {
            client,
            base_url,
            app_handle,
        };
        info!("BackendClient instance created:");
        instance
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<T, Box<dyn Error + Send + Sync>> {
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
        let response = self._post(endpoint, body).await?;

        let json = response.json::<T>().await?;

        info!("POST Response Body: {:?}", json);
        Ok(json)
    }
    async fn send(
        &self,
        mut request: RequestBuilder,
    ) -> Result<Response, Box<dyn Error + Send + Sync>> {
        request = self.add_authorization_header(request).await;
        let response = request.send().await?;
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
