use crate::services::path::path_detection::{detect_steam_path_internal, SteamPathDetectionResult};

#[tauri::command]
pub fn detect_paths(manual_path: Option<String>) -> SteamPathDetectionResult {
    detect_steam_path_internal(manual_path)
}
