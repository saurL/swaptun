use std::sync::Arc;

use log::error;
use tauri::{command, State};
use tauri_plugin_deep_link::DeepLinkExt;

use crate::app::App;

#[command]
pub async fn is_app_ready(app: State<'_, Arc<App>>) -> Result<bool, String> {
    app.is_app_ready().await
}
#[command]

pub async fn check_opening_url(app: State<'_, Arc<App>>) -> Result<(), String> {
    let app_handle = app.app_handle();
    let current_route = match app_handle.deep_link().get_current() {
        Ok(urls) => urls,
        Err(e) => {
            error!("Failed to get current route: {}", e);
            None
        }
    };
    if let Some(urls) = current_route {
        app.handle_open_url(urls).await;
    }
    Ok(())
}
