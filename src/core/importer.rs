use crate::utils;
use std::path::{Path, PathBuf};
use url::Url;

/// Target for an import operation
#[derive(Debug, Clone)]
pub enum ImportTarget {
    Url(String),
    File(PathBuf),
    Directory(PathBuf),
}

/// Helper to expand paths with ~ to standard `PathBuf`
pub fn expand_home(path_str: &str) -> PathBuf {
    if let Some(stripped) = path_str.strip_prefix("~/") {
        if let Some(home) = crate::utils::home_dir() {
            return home.join(stripped);
        }
    }
    PathBuf::from(path_str)
}

/// Resolves the import target type from a path string
pub fn resolve_target(input: &str) -> Result<ImportTarget, String> {
    let input = input.trim();

    // 1. Check for URL
    if input.starts_with("http://") || input.starts_with("https://") {
        match Url::parse(input) {
            Ok(url) => return Ok(ImportTarget::Url(url.to_string())),
            Err(_) => return Err("Invalid URL format".to_string()),
        }
    }

    // 2. Expand Path
    let path = expand_home(input);

    // 3. Check file existence and type
    if !path.exists() {
        return Err(format!("Path not found: {input}"));
    }

    // 4. Determine Type
    if path.is_file() {
        Ok(ImportTarget::File(path))
    } else if path.is_dir() {
        Ok(ImportTarget::Directory(path))
    } else {
        Err("Invalid path type (not a file or directory)".to_string())
    }
}
