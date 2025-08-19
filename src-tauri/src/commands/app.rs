use std::sync::Arc;

use log::error;
use swaptun_backend::{
    AddFriendRequest, GetUsersRequest, RemoveFriendRequest, SearchField, UserBean,
};
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

#[command]
pub async fn search_users(
    app: State<'_, Arc<App>>,
    search: Option<String>,
) -> Result<Vec<UserBean>, String> {
    let request = GetUsersRequest {
        include_deleted: Some(false),
        search,
        search_field: Some(SearchField::Username),
        limit: Some(100),
        offset: None,
        friends_priority: false,
        exclude_friends: false,
    };

    match app.search_users(request).await {
        Ok(users) => Ok(users),
        Err(e) => Err(format!("Error fetching users: {}", e)),
    }
}

#[command]
pub async fn get_friends(app: State<'_, Arc<App>>) -> Result<Vec<UserBean>, String> {
    match app.get_friends().await {
        Ok(friends) => Ok(friends),
        Err(e) => Err(format!("Error fetching friends: {}", e)),
    }
}

#[command]
pub async fn add_friend(app: State<'_, Arc<App>>, request: AddFriendRequest) -> Result<(), String> {
    match app.add_friend(request).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error adding friend: {}", e)),
    }
}

#[command]
pub async fn remove_friend(
    app: State<'_, Arc<App>>,
    request: RemoveFriendRequest,
) -> Result<(), String> {
    match app.remove_friend(request).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Error removing friend: {}", e)),
    }
}

#[command]
pub async fn search_non_friends_users(
    app: State<'_, Arc<App>>,
    search: Option<String>,
) -> Result<Vec<UserBean>, String> {
    let request = GetUsersRequest {
        include_deleted: Some(false),
        search,
        search_field: Some(SearchField::Username),
        limit: Some(100),
        offset: None,
        friends_priority: true,
        exclude_friends: true,
    };

    match app.search_users(request).await {
        Ok(users) => Ok(users),
        Err(e) => Err(format!("Error fetching users: {}", e)),
    }
}
