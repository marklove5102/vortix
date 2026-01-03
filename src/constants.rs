//! Application-wide constants and configuration values.
//!
//! This module defines all static configuration values used throughout Vortix,
//! including timing intervals, API endpoints, file paths, and UI messages.

#![allow(dead_code)]
use std::time::Duration;

// === Application Metadata ===

/// Application name and title (from Cargo.toml).
pub const APP_NAME: &str = env!("CARGO_PKG_NAME");
/// Current application version (from Cargo.toml).
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");
/// Short technical summary of the application (from Cargo.toml).
pub const APP_SUMMARY: &str = env!("CARGO_PKG_DESCRIPTION");
/// Documentation and homepage URL (from Cargo.toml).
pub const APP_URL: &str = env!("CARGO_PKG_HOMEPAGE");

// === Timing Configuration ===

/// UI refresh rate in milliseconds.
pub const DEFAULT_TICK_RATE: u64 = 1000;
/// Interval between telemetry API calls.
pub const TELEMETRY_POLL_RATE: Duration = Duration::from_secs(30);

// === Path Configuration ===

/// Name of the profiles subdirectory.
pub const PROFILES_DIR_NAME: &str = "profiles";
/// Name of the logs subdirectory.
pub const LOGS_DIR_NAME: &str = "logs";
/// Name of the profile metadata file.
pub const METADATA_FILE_NAME: &str = "metadata.json";

// === Telemetry API Endpoints ===

/// Primary API endpoint for IP address and ISP lookup.
pub const IP_API_PRIMARY: &str = "https://ipinfo.io/json";
/// Fallback API 1: ipify.org (IP only, very reliable).
pub const IP_API_FALLBACK_1: &str = "https://api.ipify.org";
/// Fallback API 2: icanhazip.com (IP only).
pub const IP_API_FALLBACK_2: &str = "https://icanhazip.com";
/// Fallback API 3: ifconfig.me (IP only).
pub const IP_API_FALLBACK_3: &str = "https://ifconfig.me/ip";

/// IPv6 leak detection endpoints (any success = leak).
pub const IPV6_CHECK_APIS: [&str; 3] = [
    "https://ipv6.icanhazip.com",
    "https://v6.ident.me",
    "https://api6.ipify.org",
];

/// Ping targets for latency measurement (tried in order).
pub const PING_TARGETS: [&str; 4] = [
    "1.1.1.1",        // Cloudflare
    "8.8.8.8",        // Google
    "9.9.9.9",        // Quad9
    "208.67.222.222", // OpenDNS
];

/// Timeout for HTTP API calls in seconds.
pub const API_TIMEOUT_SECS: u8 = 5;
/// Timeout for file downloads in seconds.
pub const HTTP_TIMEOUT_SECS: u64 = 10;
/// Timeout for ping commands in seconds.
pub const PING_TIMEOUT_SECS: u8 = 2;
/// Delay between retry attempts in milliseconds.
pub const RETRY_DELAY_MS: u64 = 500;
/// Number of retry attempts per API/target.
pub const RETRY_ATTEMPTS: u8 = 2;

// === UI Messages ===

/// Initialization message template.
pub const MSG_INIT: &str = "INIT: {} v{} starting...";
/// Backend initialization message.
pub const MSG_BACKEND_INIT: &str = "IO: Initializing VPN backend...";
/// Ready state message template.
pub const MSG_READY: &str = "SUCCESS: System active. Press [x] for actions.";
/// Connection in progress message template.
pub const MSG_CONNECTING: &str = "Connecting to {}";
/// Connection established message template.
pub const MSG_CONNECTED: &str = "Connected to {}";
/// Disconnection message.
pub const MSG_DISCONNECTED: &str = "Disconnected";
/// Detection in progress placeholder.
pub const MSG_DETECTING: &str = "Detecting...";
/// Data fetching placeholder.
pub const MSG_FETCHING: &str = "Fetching...";
/// No data available placeholder.
pub const MSG_NO_DATA: &str = "---";

// === Cryptographic Defaults ===

/// Default cipher suite for `WireGuard` connections.
pub const DEFAULT_CIPHER: &str = "ChaCha20Poly1305";
// === Import & Download Configuration ===

/// Default filename for downloaded profiles if none can be determined.
pub const DEFAULT_IMPORTED_FILENAME: &str = "imported_profile.conf";

// === UI Labels & Titles ===

pub const TITLE_IMPORT_PROFILE: &str = " Import VPN Profile ";
pub const TITLE_IMPORT_FOOTER: &str = " [Enter] Import  [Esc] Cancel ";
pub const PROMPT_IMPORT_PATH: &str = "Enter path to file, directory, or URL:";
pub const HINT_IMPORT_BULK: &str = "💡 Tip: Enter a directory to bulk import all profiles";
pub const LABEL_SUPPORTED_FORMATS: &str = "Supported formats:";
pub const EXT_CONF: &str = ".conf";
pub const EXT_OVPN: &str = ".ovpn";
pub const PROTO_WIREGUARD: &str = "WireGuard";
pub const PROTO_OPENVPN: &str = "OpenVPN";

// === Messages: General (Toast/Logs) ===

pub const MSG_DOWNLOADING: &str = "Downloading profile...";
pub const MSG_DOWNLOAD_FAILED: &str = "Download failed: ";
pub const MSG_IMPORT_SUCCESS: &str = "Imported: ";
pub const MSG_IMPORT_ERROR: &str = "Error: ";
pub const MSG_PATH_NOT_FOUND: &str = "Path not found: ";
pub const MSG_INVALID_PATH_TYPE: &str = "Invalid path type";
pub const MSG_NO_FILES_FOUND: &str = "No .conf or .ovpn files found";
pub const MSG_BATCH_IMPORTED: &str = "Imported ";
pub const MSG_BATCH_IMPORTED_SUFFIX: &str = " profile(s)";
pub const MSG_BATCH_IMPORTED_WITH_FAILURES: &str = "Imported ";
pub const MSG_BATCH_LOG_TEMPLATE: &str = "IMPORT: Batch imported {} profiles from {}";

// === Messages: CLI Output ===

pub const CLI_MSG_DOWNLOADING: &str = "Downloading profile from URL...";
pub const CLI_MSG_IMPORT_SUCCESS: &str = "Imported profile: ";
pub const CLI_MSG_IMPORT_DETAILS_PROTO: &str = "   Protocol: ";
pub const CLI_MSG_IMPORT_DETAILS_LOC: &str = "   Location: ";
pub const CLI_MSG_IMPORT_DETAILS_PATH: &str = "   Saved to: ";
pub const CLI_MSG_IMPORT_FAILED: &str = "Import failed: ";
pub const CLI_MSG_PATH_NOT_FOUND: &str = "Path not found: ";
pub const CLI_MSG_INVALID_PATH: &str = "Invalid path type";
pub const CLI_MSG_SUMMARY_HEADER: &str = "\nImport Summary:";
pub const CLI_MSG_SUMMARY_IMPORTED: &str = "   Imported: ";
pub const CLI_MSG_SUMMARY_FAILED: &str = "   Failed: ";
pub const CLI_MSG_NO_FILES: &str = "\nNo .conf or .ovpn files found in directory";
pub const CLI_MSG_DIR_ERROR: &str = "Error reading directory: ";
pub const CLI_MSG_ERROR: &str = "Error: ";

pub const CLI_MSG_UPDATE_START: &str = "🔄 Updating vortix...\n";
pub const CLI_MSG_UPDATE_SUCCESS: &str = "Successfully updated vortix!";
pub const CLI_MSG_UPDATE_CHECK: &str = "   Run 'vortix --version' to see the new version.";
pub const CLI_MSG_UPDATE_FAIL_MANUAL: &str = "\nUpdate failed. Please try manually:";
pub const CLI_MSG_UPDATE_CMD: &str = "   cargo install vortix --force";
pub const CLI_MSG_UPDATE_FAIL_CARGO: &str = "Failed to run cargo: ";
pub const CLI_MSG_UPDATE_PATH_HINT: &str = "   Make sure cargo is installed and in your PATH.";

// === Error Messages ===

pub const ERR_HTML_CONTENT: &str =
    "URL returned HTML content. Did you mean to use the 'raw' version of the link?";
pub const ERR_EMPTY_CONTENT: &str = "Downloaded content is empty";
pub const ERR_SERVER_ERROR: &str = "Server returned error: ";
pub const ERR_HTTP_CLIENT_BUILD_FAILED: &str = "Failed to build HTTP client";
pub const ERR_NETWORK_REQUEST_FAILED: &str = "Network request failed";
pub const ERR_READ_CONTENT_FAILED: &str = "Failed to read content";
