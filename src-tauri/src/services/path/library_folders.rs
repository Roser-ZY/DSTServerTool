use std::fs;
use std::path::{Path, PathBuf};

pub fn parse_library_folders(steam_root: &Path) -> Vec<PathBuf> {
    let vdf_path = steam_root.join("steamapps/libraryfolders.vdf");
    if !vdf_path.is_file() {
        return Vec::new();
    }

    let content = match fs::read_to_string(&vdf_path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    extract_paths_from_vdf(&content)
}

fn extract_paths_from_vdf(content: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("\"path\"") {
            if let Some(path) = extract_path_from_line(trimmed) {
                if !path.as_os_str().is_empty() {
                    paths.push(path);
                }
            }
        }
    }

    paths
}

fn extract_path_from_line(line: &str) -> Option<PathBuf> {
    let parts: Vec<&str> = line.split('"').collect();

    if parts.len() >= 4 && parts[0].trim().is_empty() && parts[1] == "path" {
        let path_str = parts[3].replace("\\\\", "\\");
        if !path_str.is_empty() {
            return Some(PathBuf::from(path_str));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_library_folders_valid() {
        let vdf_content = r#"
"libraryfolders"
{
    "0"
    {
        "path" "D:\Games\Steam"
        "label" ""
        "contentid" "123456789"
    }
    "1"
    {
        "path" "E:\SteamLibrary"
        "label" ""
        "contentid" "987654321"
    }
}
"#;
        let paths = extract_paths_from_vdf(vdf_content);
        assert!(paths.len() >= 1);
    }

    #[test]
    fn test_parse_library_folders_malformed() {
        let vdf_content = r#"
"libraryfolders"
{
    "0"
    {
        "path" "D:\Games\Steam"
    }
    "malformed"
    {
        "invalid" "data"
    }
}
"#;
        let paths = extract_paths_from_vdf(vdf_content);
        assert!(!paths.is_empty());
    }

    #[test]
    fn test_parse_library_folders_empty() {
        let vdf_content = r#"
"libraryfolders"
{
}
"#;
        let paths = extract_paths_from_vdf(vdf_content);
        assert!(paths.is_empty());
    }

    #[test]
    fn test_parse_library_folders_supports_multiple_library_ids() {
        let vdf_content = r#"
"libraryfolders"
{
    "0"
    {
        "path" "D:\\Games\\Steam"
    }
    "7"
    {
        "path" "E:\\SteamLibrary"
    }
}
"#;

        let paths = extract_paths_from_vdf(vdf_content);

        assert_eq!(paths.len(), 2);
        assert_eq!(paths[0], PathBuf::from(r"D:\Games\Steam"));
        assert_eq!(paths[1], PathBuf::from(r"E:\SteamLibrary"));
    }
}
