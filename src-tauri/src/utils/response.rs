use log::error;
use tauri_plugin_http::reqwest::StatusCode;

/// Helper to convert StatusCode to Result<bool, String>
pub fn status_to_result(status: StatusCode, operation: &str) -> Result<bool, String> {
    if status.is_success() {
        Ok(true)
    } else {
        let error_msg = format!("{} failed with status: {}", operation, status);
        error!("{}", error_msg);
        Err(error_msg)
    }
}

/// Helper to log and convert errors to String
pub fn log_error<E: std::fmt::Display>(err: E, context: &str) -> String {
    let error_msg = format!("{}: {}", context, err);
    error!("{}", error_msg);
    error_msg
}

/// Macro to simplify command error handling
#[macro_export]
macro_rules! handle_result {
    ($result:expr) => {
        $result.map_err(|e| e.to_string())
    };
    ($result:expr, $context:expr) => {
        $result.map_err(|e| $crate::utils::log_error(e, $context))
    };
}
