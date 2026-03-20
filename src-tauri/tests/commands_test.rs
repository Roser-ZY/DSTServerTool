use dst_server_tool::commands::detect_paths;
use dst_server_tool::services::path::path_detection::SteamPathStatus;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_path(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("dst_server_tool_{name}_{nanos}"))
}

#[cfg(not(target_os = "macos"))]
#[test]
fn detect_paths_accepts_valid_manual_path() {
    let root = unique_temp_path("valid_integration");
    fs::create_dir_all(root.join("steamapps/common")).unwrap();
    fs::create_dir_all(root.join("steamapps/workshop")).unwrap();
    fs::create_dir_all(root.join("config")).unwrap();
    fs::write(
        root.join("config/libraryfolders.vdf"),
        "\"libraryfolders\"\n{\n}\n",
    )
    .unwrap();

    let result = detect_paths(Some(root.to_string_lossy().into_owned()));

    assert_eq!(result.status, SteamPathStatus::Valid);
    assert_eq!(
        result.path.as_deref(),
        Some(root.to_string_lossy().as_ref())
    );

    fs::remove_dir_all(&root).unwrap();
}

#[test]
fn detect_paths_rejects_invalid_manual_path() {
    let root = unique_temp_path("invalid_integration");
    fs::create_dir_all(root.join("steamapps/common")).unwrap();

    let result = detect_paths(Some(root.to_string_lossy().into_owned()));

    assert_eq!(result.status, SteamPathStatus::Invalid);

    fs::remove_dir_all(&root).unwrap();
}
