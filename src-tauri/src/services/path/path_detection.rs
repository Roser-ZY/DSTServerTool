use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::services::path::library_folders;

const WINDOWS_LINUX_REQUIRED_DIRECTORIES: [&str; 2] = ["steamapps/common", "steamapps/workshop"];
const WINDOWS_LINUX_STEAM_MARKER_PATHS: [&str; 2] =
    ["steamapps/libraryfolders.vdf", "config/libraryfolders.vdf"];
const MACOS_REQUIRED_DIRECTORIES: [&str; 0] = [];
const MACOS_STEAM_MARKER_PATHS: [&str; 3] = [
    "registry.vdf",
    "Steam.AppBundle",
    "config/libraryfolders.vdf",
];
const MACOS_STEAM_APPBUNDLE_EXECUTABLE: &str = "Steam.AppBundle/Steam";

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SteamPathStatus {
    Valid,
    Invalid,
    NotFound,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SteamPathDetectionResult {
    pub status: SteamPathStatus,
    pub path: Option<String>,
    pub source: Option<String>,
    #[serde(rename = "validationErrors")]
    pub validation_errors: Vec<String>,
}

fn platform_required_directories() -> &'static [&'static str] {
    if cfg!(target_os = "macos") {
        &MACOS_REQUIRED_DIRECTORIES
    } else {
        &WINDOWS_LINUX_REQUIRED_DIRECTORIES
    }
}

fn platform_marker_paths() -> &'static [&'static str] {
    if cfg!(target_os = "macos") {
        &MACOS_STEAM_MARKER_PATHS
    } else {
        &WINDOWS_LINUX_STEAM_MARKER_PATHS
    }
}

fn marker_exists(root: &Path, marker_path: &str) -> bool {
    let path = root.join(marker_path);
    path.is_file() || path.is_dir()
}

fn validate_steam_root_errors(root: &Path) -> Vec<String> {
    let mut errors = Vec::new();

    for required_path in platform_required_directories() {
        let required = root.join(required_path);
        if !required.is_dir() {
            errors.push(format!(
                "Missing required directory: {}",
                required.to_string_lossy()
            ));
        }
    }

    #[cfg(target_os = "macos")]
    {
        let app_bundle = root.join(MACOS_STEAM_APPBUNDLE_EXECUTABLE);
        if !app_bundle.is_dir() {
            errors.push(format!(
                "Missing Steam.AppBundle/Steam subdirectory: {}",
                app_bundle.to_string_lossy()
            ));
        }
    }

    let marker_paths = platform_marker_paths();
    let has_marker = marker_paths
        .iter()
        .any(|marker_path| marker_exists(root, marker_path));
    if !has_marker {
        errors.push(format!(
            "Missing Steam marker file: one of {}",
            marker_paths.join(", ")
        ));
    }

    errors
}

pub fn validate_steam_root(root: &Path) -> SteamPathDetectionResult {
    let errors = validate_steam_root_errors(root);

    let status = if errors.is_empty() {
        SteamPathStatus::Valid
    } else {
        SteamPathStatus::Invalid
    };

    SteamPathDetectionResult {
        status,
        path: Some(root.to_string_lossy().into_owned()),
        source: None,
        validation_errors: errors,
    }
}

fn get_probe_candidates() -> Vec<(String, PathBuf)> {
    let mut candidates = Vec::new();

    #[cfg(target_os = "windows")]
    {
        let from_env = ["PROGRAMFILES(X86)", "PROGRAMFILES"]
            .iter()
            .filter_map(|var| std::env::var(var).ok())
            .map(|base| PathBuf::from(base).join("Steam"));

        for path in from_env {
            candidates.push(("windows-default".to_owned(), path));
        }

        candidates.push((
            "windows-default".to_owned(),
            PathBuf::from(r"C:\Program Files (x86)\Steam"),
        ));
        candidates.push((
            "windows-default".to_owned(),
            PathBuf::from(r"C:\Program Files\Steam"),
        ));
    }

    #[cfg(target_os = "macos")]
    {
        if let Some(home) = std::env::var_os("HOME") {
            candidates.push((
                "macos-default".to_owned(),
                PathBuf::from(home).join("Library/Application Support/Steam"),
            ));
        }
    }

    candidates
}

#[cfg_attr(not(target_os = "windows"), allow(dead_code))]
fn parse_registry_install_path_output(output: &str) -> Option<PathBuf> {
    for line in output.lines() {
        if !line.contains("InstallPath") {
            continue;
        }

        if let Some((_, value)) = line.split_once("REG_SZ") {
            let path = value.trim();
            if !path.is_empty() {
                return Some(PathBuf::from(path));
            }
        }

        if let Some((_, value)) = line.split_once("REG_EXPAND_SZ") {
            let path = value.trim();
            if !path.is_empty() {
                return Some(PathBuf::from(path));
            }
        }
    }

    None
}

fn detect_from_candidate(path: &Path, source: &str) -> Option<SteamPathDetectionResult> {
    let mut result = validate_steam_root(path);
    result.source = Some(source.to_owned());
    if result.status == SteamPathStatus::Valid {
        return Some(result);
    }

    let library_paths = library_folders::parse_library_folders(path);
    for lib_path in library_paths {
        let mut lib_result = validate_steam_root(&lib_path);
        lib_result.source = Some("library_folders".to_owned());
        if lib_result.status == SteamPathStatus::Valid {
            return Some(lib_result);
        }
    }

    None
}

fn not_found_result() -> SteamPathDetectionResult {
    SteamPathDetectionResult {
        status: SteamPathStatus::NotFound,
        path: None,
        source: None,
        validation_errors: vec![
            "No Steam installation found in default probe locations.".to_owned()
        ],
    }
}

pub fn detect_steam_path_from_candidates(
    candidates: Vec<(String, PathBuf)>,
) -> SteamPathDetectionResult {
    for (source, path) in candidates {
        if !path.exists() {
            continue;
        }

        if let Some(result) = detect_from_candidate(&path, &source) {
            return result;
        }
    }

    not_found_result()
}

#[cfg(target_os = "windows")]
#[allow(dead_code)]
fn get_registry_steam_path() -> Option<PathBuf> {
    use std::process::Command;

    let reg_path = r"SOFTWARE\WOW6432Node\Valve\Steam";
    let output = Command::new("reg")
        .args(["query", &format!("HKLM\\{}", reg_path), "/v", "InstallPath"])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    parse_registry_install_path_output(&stdout).filter(|path| path.exists())
}

#[cfg(not(target_os = "windows"))]
#[allow(dead_code)]
fn get_registry_steam_path() -> Option<PathBuf> {
    None
}

pub fn detect_steam_path_internal(manual_path: Option<String>) -> SteamPathDetectionResult {
    if let Some(raw_path) = manual_path {
        let provided = PathBuf::from(raw_path);
        let mut result = validate_steam_root(&provided);
        result.source = Some("manual".to_owned());
        return result;
    }

    let mut candidates = Vec::new();

    #[cfg(target_os = "windows")]
    {
        if let Some(registry_path) = get_registry_steam_path() {
            candidates.push(("registry".to_owned(), registry_path));
        }
    }

    candidates.extend(get_probe_candidates());

    detect_steam_path_from_candidates(candidates)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_path(name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("dst_server_tool_{name}_{nanos}"))
    }

    #[test]
    fn test_parse_registry_output_preserves_spaces_in_install_path() {
        let output = "HKEY_LOCAL_MACHINE\\SOFTWARE\\WOW6432Node\\Valve\\Steam\n    InstallPath    REG_SZ    C:\\Program Files (x86)\\Steam\n";

        let path = parse_registry_install_path_output(output);

        assert_eq!(path, Some(PathBuf::from(r"C:\Program Files (x86)\Steam")));
    }

    #[test]
    fn test_validate_steam_root_requires_steam_marker_for_manual_path() {
        let root = unique_temp_path("manual_missing_marker");
        fs::create_dir_all(root.join("steamapps/common")).unwrap();
        fs::create_dir_all(root.join("steamapps/workshop")).unwrap();

        let result = validate_steam_root(&root);

        assert_eq!(result.status, SteamPathStatus::Invalid);
        assert!(result
            .validation_errors
            .iter()
            .any(|error| error.contains("config/libraryfolders.vdf")));

        fs::remove_dir_all(&root).unwrap();
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn test_validate_steam_root_accepts_macos_markers() {
        let root = unique_temp_path("macos_real_layout");
        fs::create_dir_all(root.join("Steam.AppBundle/Steam")).unwrap();
        fs::write(root.join("registry.vdf"), "registry").unwrap();
        fs::create_dir_all(root.join("config")).unwrap();
        fs::write(root.join("config/libraryfolders.vdf"), "").unwrap();

        let result = validate_steam_root(&root);

        assert_eq!(result.status, SteamPathStatus::Valid);

        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn test_auto_probe_false_positive_returns_not_found() {
        let root = unique_temp_path("auto_false_positive");
        fs::create_dir_all(root.join("steamapps/common")).unwrap();
        fs::create_dir_all(root.join("steamapps/workshop")).unwrap();

        let result =
            detect_steam_path_from_candidates(vec![("macos-default".to_owned(), root.clone())]);

        assert_eq!(result.status, SteamPathStatus::NotFound);
        assert_eq!(result.path, None);

        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn test_auto_probe_without_candidates_returns_not_found() {
        let result = detect_steam_path_from_candidates(Vec::new());

        assert_eq!(result.status, SteamPathStatus::NotFound);
        assert_eq!(result.path, None);
    }
}
