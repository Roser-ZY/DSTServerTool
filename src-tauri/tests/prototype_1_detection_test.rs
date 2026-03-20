use dst_server_tool::services::path::path_detection::{
    detect_steam_path_from_candidates, detect_steam_path_internal, validate_steam_root,
    SteamPathStatus,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root should exist")
        .to_path_buf()
}

fn fixture_root() -> PathBuf {
    repo_root().join("tests/prototype-1/mock")
}

fn unique_temp_path(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("dst_server_tool_{name}_{nanos}"))
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).expect("destination directory should be created");

    for entry in fs::read_dir(src).expect("source directory should be readable") {
        let entry = entry.expect("directory entry should be readable");
        let source_path = entry.path();
        let destination_path = dst.join(entry.file_name());

        if entry
            .file_type()
            .expect("file type should be readable")
            .is_dir()
        {
            copy_dir_recursive(&source_path, &destination_path);
        } else {
            fs::copy(&source_path, &destination_path).expect("file should be copied");
        }
    }
}

fn fixture_path(relative_path: &str) -> PathBuf {
    fixture_root().join(relative_path)
}

fn escaped_vdf_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "\\\\")
}

fn prepare_libraryfolders_fixture(platform: &str) -> (PathBuf, PathBuf) {
    let root = unique_temp_path(&format!("{platform}_libraryfolders"));
    let host_source = fixture_path(&format!("{platform}/library-host"));
    let library_source = fixture_path(&format!("{platform}/library-root"));

    let host_root = root.join("host");
    let library_root = root.join("library-root");

    copy_dir_recursive(&host_source, &host_root);
    copy_dir_recursive(&library_source, &library_root);

    let host_vdf = host_root.join("steamapps/libraryfolders.vdf");

    let content = fs::read_to_string(&host_vdf).expect("host vdf should exist");
    let updated = content.replace("__LIBRARY_ROOT__", &escaped_vdf_path(&library_root));
    fs::write(&host_vdf, updated).expect("host vdf should be updated");

    (host_root, library_root)
}

#[cfg(target_os = "windows")]
#[test]
fn prototype_1_windows_real_install_detection_smoke() {
    let result = detect_steam_path_internal(None);

    assert_eq!(result.status, SteamPathStatus::Valid);
    assert!(result.path.is_some());
}

#[cfg(target_os = "windows")]
#[test]
fn prototype_1_windows_default_mock_detects_valid_root() {
    let result = detect_steam_path_from_candidates(vec![(
        "windows-default".to_owned(),
        fixture_path("windows/default-root"),
    )]);

    assert_eq!(result.status, SteamPathStatus::Valid);
    assert_eq!(result.source.as_deref(), Some("windows-default"));
}

#[cfg(target_os = "windows")]
#[test]
fn prototype_1_windows_registry_mock_detects_valid_root() {
    let result = detect_steam_path_from_candidates(vec![(
        "registry".to_owned(),
        fixture_path("windows/registry-root"),
    )]);

    assert_eq!(result.status, SteamPathStatus::Valid);
    assert_eq!(result.source.as_deref(), Some("registry"));
}

#[cfg(target_os = "windows")]
#[test]
fn prototype_1_windows_libraryfolders_mock_detects_library_root() {
    let (host_root, library_root) = prepare_libraryfolders_fixture("windows");

    let result =
        detect_steam_path_from_candidates(vec![("windows-default".to_owned(), host_root.clone())]);

    assert_eq!(result.status, SteamPathStatus::Valid);
    assert_eq!(result.source.as_deref(), Some("library_folders"));
    assert_eq!(
        result.path.as_deref(),
        Some(library_root.to_string_lossy().as_ref())
    );

    fs::remove_dir_all(host_root.parent().expect("temp root should exist")).unwrap();
}

#[cfg(target_os = "macos")]
#[test]
fn prototype_1_macos_real_install_detection_smoke() {
    let result = detect_steam_path_internal(None);

    assert_eq!(result.status, SteamPathStatus::Valid);
    assert!(result.path.is_some());
}

#[cfg(target_os = "macos")]
#[test]
fn prototype_1_macos_default_mock_detects_valid_root() {
    let result = detect_steam_path_from_candidates(vec![(
        "macos-default".to_owned(),
        fixture_path("macos/valid-root"),
    )]);

    assert_eq!(result.status, SteamPathStatus::Valid);
    assert_eq!(result.source.as_deref(), Some("macos-default"));
}

#[cfg(target_os = "macos")]
#[test]
fn prototype_1_macos_libraryfolders_mock_detects_library_root() {
    let (host_root, library_root) = prepare_libraryfolders_fixture("macos");

    let result =
        detect_steam_path_from_candidates(vec![("macos-default".to_owned(), host_root.clone())]);

    assert_eq!(result.status, SteamPathStatus::Valid);
    assert_eq!(result.source.as_deref(), Some("library_folders"));
    assert_eq!(
        result.path.as_deref(),
        Some(library_root.to_string_lossy().as_ref())
    );

    fs::remove_dir_all(host_root.parent().expect("temp root should exist")).unwrap();
}

#[cfg(target_os = "macos")]
#[test]
fn prototype_1_macos_manual_invalid_missing_markers() {
    let result = validate_steam_root(&fixture_path("macos/invalid-root"));

    assert_eq!(result.status, SteamPathStatus::Invalid);
    assert!(result
        .validation_errors
        .iter()
        .any(|error| error.contains("registry.vdf") || error.contains("Steam.AppBundle")));
}
