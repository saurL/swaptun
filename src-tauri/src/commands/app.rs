use std::sync::Arc;

use log::error;
use swaptun_backend::{
    AddFriendRequest, GetUsersRequest, RemoveFriendRequest, SearchField, UserBean,
};
use tauri::{command, AppHandle, State};
use tauri_plugin_deep_link::DeepLinkExt;
use tauri_plugin_opener::OpenerExt;

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
        exclude_self: Some(true),
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
        exclude_self: Some(true),
    };

    match app.search_users(request).await {
        Ok(users) => Ok(users),
        Err(e) => Err(format!("Error fetching users: {}", e)),
    }
}

#[command]
pub async fn open_external_app(
    app_handle: AppHandle,
    platform: String,
    playlist_id: Option<String>,
) -> Result<(), String> {
    let url = match platform.as_str() {
        "Spotify" => {
            if let Some(id) = playlist_id {
                // Spotify uses: spotify://playlist/{id}
                format!("spotify://playlist/{}", id)
            } else {
                "spotify://".to_string()
            }
        }
        "YoutubeMusic" => {
            #[cfg(target_os = "android")]
            {
                if let Some(id) = playlist_id {
                    // YouTube Music on Android uses: vnd.youtube://music/playlist?list={id}
                    format!("vnd.youtube://music/playlist?list={}", id)
                } else {
                    "vnd.youtube://music".to_string()
                }
            }
            #[cfg(not(target_os = "android"))]
            {
                if let Some(id) = playlist_id {
                    // YouTube Music on iOS: https://music.youtube.com/playlist?list={id}
                    format!("https://music.youtube.com/playlist?list={}", id)
                } else {
                    "https://music.youtube.com".to_string()
                }
            }
        }
        "AppleMusic" => {
            if let Some(id) = playlist_id {
                // Apple Music uses: music://playlist/{id} or https://music.apple.com/library/playlist/{id}
                #[cfg(target_os = "ios")]
                {
                    format!("music://library/playlist/{}", id)
                }
                #[cfg(not(target_os = "ios"))]
                {
                    format!("https://music.apple.com/library/playlist/{}", id)
                }
            } else {
                "music://".to_string()
            }
        }
        "Deezer" => {
            if let Some(id) = playlist_id {
                // Deezer uses: deezer://www.deezer.com/playlist/{id}
                format!("deezer://www.deezer.com/playlist/{}", id)
            } else {
                "deezer://".to_string()
            }
        }
        _ => return Err(format!("Unknown platform: {}", platform)),
    };

    // Use opener plugin which works cross-platform
    if let Err(e) = app_handle.opener().open_url(url.clone(), None::<&str>) {
        error!("Failed to open external app: {}", e);
        return Err(format!("Failed to open {}: {}", platform, e));
    }

    Ok(())
}
