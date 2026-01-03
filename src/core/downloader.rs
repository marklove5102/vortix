//! URL Downloader logic for profile imports.

use crate::constants;
use crate::utils;
use reqwest::blocking::Client;
use std::path::PathBuf;
use std::time::Duration;
use url::Url;

/// Downloads a VPN profile from a given URL and saves it to the profiles directory.
///
/// # Arguments
///
/// * `url` - The direct URL to download the config from.
///
/// # Returns
///
/// The `PathBuf` of the saved file, or an Error string.
pub fn download_profile(url: &str) -> Result<PathBuf, String> {
    let client = Client::builder()
        .timeout(Duration::from_secs(constants::HTTP_TIMEOUT_SECS))
        // .danger_accept_invalid_certs(false) // Removed this line as per example
        .user_agent(format!("{}/{}", crate::constants::APP_NAME, crate::constants::APP_VERSION))
        .build()
        .map_err(|e| format!("{}: {e}", constants::ERR_HTTP_CLIENT_BUILD_FAILED))?;

    let response = client
        .get(url)
        .send()
        .map_err(|e| format!("{}: {e}", constants::ERR_NETWORK_REQUEST_FAILED))?;

    if !response.status().is_success() {
        return Err(format!(
            "{}{}",
            constants::ERR_SERVER_ERROR,
            response.status()
        ));
    }

    // Check for HTML content (common mistake with GitHub/GitLab links)
    if let Some(content_type) = response.headers().get("content-type") {
        let ct = content_type.to_str().unwrap_or("").to_lowercase();
        if ct.contains("text/html") {
            return Err(constants::ERR_HTML_CONTENT.to_string());
        }
    }

    // Try to get filename from Content-Disposition
    let mut filename = String::from(constants::DEFAULT_IMPORTED_FILENAME);

    if let Some(disposition) = response.headers().get("content-disposition") {
        let disp_str = disposition.to_str().unwrap_or("");
        if let Some(start) = disp_str.find("filename=") {
            let rest = &disp_str[start + 9..];
            let end = rest.find(';').unwrap_or(rest.len());
            let raw_name = rest[..end].trim().trim_matches('"');
            if !raw_name.is_empty() {
                filename = raw_name.to_string();
            }
        }
    }

    // Fallback: try to get from URL path if filename is still default
    if filename == constants::DEFAULT_IMPORTED_FILENAME {
        if let Ok(parsed_url) = Url::parse(url) {
            if let Some(mut segments) = parsed_url.path_segments() {
                if let Some(last) = segments.next_back() {
                    if !last.is_empty()
                        && (last.ends_with(constants::EXT_OVPN)
                            || last.ends_with(constants::EXT_CONF))
                    {
                        filename = last.to_string();
                    }
                }
            }
        }
    }

    let default_ext = if url.contains(constants::EXT_OVPN) {
        constants::EXT_OVPN
    } else {
        constants::EXT_CONF
    };
    if !filename.contains('.') {
        filename = format!("{filename}.{default_ext}");
    }

    let content = response
        .bytes()
        .map_err(|e| format!("{}: {e}", constants::ERR_READ_CONTENT_FAILED))?;

    if content.is_empty() {
        return Err(constants::ERR_EMPTY_CONTENT.to_string());
    }

    let profiles_dir = std::env::temp_dir();
    let target_path = utils::get_unique_path(&profiles_dir, &filename);

    std::fs::write(&target_path, content).map_err(|e| format!("Failed to write file: {e}"))?;

    Ok(target_path)
}
