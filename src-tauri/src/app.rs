use tauri::AppHandle;
pub struct App {
    app_handle: AppHandle,
}

impl App {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
    }
    pub fn app_handle(&self) -> &AppHandle {
        &self.app_handle
    }
}
