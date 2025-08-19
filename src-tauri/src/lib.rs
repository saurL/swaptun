mod app;
mod backend;
mod commands;
mod models;

use tauri::{async_runtime::spawn, Emitter, Manager};
use tauri_plugin_log::{Target, TargetKind};

use app::App;
use commands::*;
use models::Notification;
use tauri_plugin_deep_link::DeepLinkExt;
use tauri_plugin_push_notifications::PushNotificationsExt;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_custom_tabs_manager::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_pinia::init())
        .plugin(tauri_plugin_push_notifications::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Webview),
                ])
                .build(),
        )
        .plugin(tauri_plugin_safe_area_insets_css::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            let swaptun_app = App::new(app_handle.clone());

            let app_handle_navigation = app_handle.clone();
            app_handle
                .push_notifications()
                .on_notification_clicked(move |data: Notification| {
                    handle_notification(&app_handle_navigation, data);
                });
            let app = swaptun_app.clone();

            app_handle.deep_link().on_open_url(move |event| {
                let app = app.clone();
                spawn(async move {
                    app.handle_open_url(event.urls()).await;
                });
            });

            app_handle.manage(swaptun_app.clone());

            spawn(async move {
                swaptun_app.set_app_ready().await;

                app_handle.emit("app_ready", "").unwrap();
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            register,
            login,
            login_email,
            verify_token,
            get_autorization_url_spotify,
            is_app_ready,
            test_spotify,
            get_playlists_spotify,
            get_playlists_deezer,
            connect_youtube,
            get_playlists_youtubemusic,
            set_fcm_token,
            send_test_notification,
            check_opening_notification,
            send_playlist,
            forgot_password,
            reset_password,
            logout,
            check_opening_url,
            search_users,
            get_friends,
            add_friend,
            remove_friend,
            search_non_friends_users
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
