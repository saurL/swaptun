use std::fmt::Debug;

use log::{error, info};
use serde::de::DeserializeOwned;
use std::error::Error;
use tauri::{http::response, AppHandle};
use tauri_plugin_http::reqwest::{Body, Client, Response, StatusCode};
pub struct BackendClient {
    client: Client,
    base_url: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
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

        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
    ) -> Result<T, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/{}", self.base_url, endpoint);
        info!("GET URL: {}", url);
        let response = self.client.get(&url).send().await?;
        info!("GET Response: {:?}", response);
        let text: T = response.json().await?;
        Ok(text)
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
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await?;
        info!("POST Response: {:?}", response);
        let status = response.status();
        info!("POST Response Status: {}", status);
        if status.is_client_error() {
            let tt = response.text().await?;
            // let json = response.json::<ErrorResponse>().await?;
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
}
