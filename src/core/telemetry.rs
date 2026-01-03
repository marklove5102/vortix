//! Background telemetry collection service.
//!
//! This module handles asynchronous collection of network telemetry data
//! including public IP address, ISP information, latency measurements,
//! DNS configuration, and IPv6 leak detection.
//!
//! The telemetry worker runs in a background thread and communicates
//! updates via an MPSC channel to the main application.

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use crate::constants;

/// Telemetry update messages sent from background workers to the main application.
#[derive(Debug, Clone)]
pub enum TelemetryUpdate {
    /// Updated public IP address.
    PublicIp(String),
    /// Updated latency measurement in milliseconds.
    Latency(u64),
    /// Packet loss percentage (0.0-100.0).
    PacketLoss(f32),
    /// Jitter (latency standard deviation) in milliseconds.
    Jitter(u64),
    /// Updated ISP/organization name.
    Isp(String),
    /// Updated DNS server address.
    Dns(String),
    /// Updated physical location (City, Country).
    Location(String),
    /// IPv6 leak detection result (true = leak detected).
    Ipv6Leak(bool),
    /// Error message for logging.
    Error(String),
}

/// Spawns a background telemetry worker that periodically fetches network information.
///
/// # Returns
///
/// A receiver channel that yields [`TelemetryUpdate`] messages as they become available.
///
/// # Panics
///
/// This function does not panic. All errors in background threads are silently handled.
///
/// # Example
///
/// ```ignore
/// let rx = spawn_telemetry_worker();
/// while let Ok(update) = rx.try_recv() {
///     match update {
///         TelemetryUpdate::PublicIp(ip) => println!("IP: {}", ip),
///         // ...
///     }
/// }
/// ```
pub fn spawn_telemetry_worker() -> Receiver<TelemetryUpdate> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || loop {
        fetch_ip_and_isp(&tx);
        fetch_latency(&tx);
        fetch_security_info(&tx);

        thread::sleep(constants::TELEMETRY_POLL_RATE);
    });

    rx
}

/// Fetches public IP address and ISP information with fallback APIs.
fn fetch_ip_and_isp(tx: &Sender<TelemetryUpdate>) {
    let tx_clone = tx.clone();
    thread::spawn(move || {
        // Primary: ipinfo.io (provides IP + ISP + Location)
        if let Some((ip, isp, loc)) = try_ipinfo_api() {
            let _ = tx_clone.send(TelemetryUpdate::PublicIp(ip));
            if let Some(org) = isp {
                let _ = tx_clone.send(TelemetryUpdate::Isp(org));
            }
            if let Some(location) = loc {
                let _ = tx_clone.send(TelemetryUpdate::Location(location));
            }
            return;
        }

        // Fallback 1: ipify.org (IP only, very reliable)
        if let Some(ip) = try_ipify_api() {
            let _ = tx_clone.send(TelemetryUpdate::PublicIp(ip));
            let _ = tx_clone.send(TelemetryUpdate::Isp("Unknown".to_string()));
            let _ = tx_clone.send(TelemetryUpdate::Location("Unknown".to_string()));
            return;
        }

        // Fallback 2: icanhazip.com (IP only)
        if let Some(ip) = try_icanhazip_api() {
            let _ = tx_clone.send(TelemetryUpdate::PublicIp(ip));
            let _ = tx_clone.send(TelemetryUpdate::Isp("Unknown".to_string()));
            let _ = tx_clone.send(TelemetryUpdate::Location("Unknown".to_string()));
            return;
        }

        // Fallback 3: ifconfig.me (IP only)
        if let Some(ip) = try_ifconfig_api() {
            let _ = tx_clone.send(TelemetryUpdate::PublicIp(ip));
            let _ = tx_clone.send(TelemetryUpdate::Isp("Unknown".to_string()));
            let _ = tx_clone.send(TelemetryUpdate::Location("Unknown".to_string()));
            return;
        }

        // All APIs failed - report error
        let _ = tx_clone.send(TelemetryUpdate::Error(
            "TELEMETRY: Failed to fetch public IP (check network/curl)".to_string(),
        ));
        let _ = tx_clone.send(TelemetryUpdate::PublicIp("Unavailable".to_string()));
    });
}

/// Try ipinfo.io API (returns IP and optionally ISP + Location) with retry
fn try_ipinfo_api() -> Option<(String, Option<String>, Option<String>)> {
    let timeout = constants::API_TIMEOUT_SECS.to_string();

    for attempt in 0..constants::RETRY_ATTEMPTS {
        let output = std::process::Command::new("curl")
            .args(["-s", "--max-time", &timeout, constants::IP_API_PRIMARY])
            .output()
            .ok()?;

        if output.status.success() {
            let text = String::from_utf8_lossy(&output.stdout);
            if let Some(ip) = extract_json_string(&text, "ip") {
                let isp = extract_json_string(&text, "org");
                let city = extract_json_string(&text, "city");
                let country = extract_json_string(&text, "country");

                let location = match (city, country) {
                    (Some(c), Some(ct)) => Some(format!("{c}, {ct}")),
                    (Some(c), None) => Some(c),
                    (None, Some(ct)) => Some(ct),
                    _ => None,
                };

                return Some((ip, isp, location));
            }
        }

        if attempt == 0 {
            thread::sleep(std::time::Duration::from_millis(constants::RETRY_DELAY_MS));
        }
    }
    None
}

/// Try ipify.org API (IP only, very reliable) with retry
fn try_ipify_api() -> Option<String> {
    let timeout = constants::API_TIMEOUT_SECS.to_string();

    for attempt in 0..constants::RETRY_ATTEMPTS {
        let output = std::process::Command::new("curl")
            .args(["-s", "--max-time", &timeout, constants::IP_API_FALLBACK_1])
            .output()
            .ok()?;

        if output.status.success() {
            let ip = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !ip.is_empty() && ip.chars().all(|c| c.is_ascii_digit() || c == '.') {
                return Some(ip);
            }
        }

        if attempt == 0 {
            thread::sleep(std::time::Duration::from_millis(constants::RETRY_DELAY_MS));
        }
    }
    None
}

/// Try icanhazip.com API (IP only) with retry
fn try_icanhazip_api() -> Option<String> {
    let timeout = constants::API_TIMEOUT_SECS.to_string();

    for attempt in 0..constants::RETRY_ATTEMPTS {
        let output = std::process::Command::new("curl")
            .args(["-s", "--max-time", &timeout, constants::IP_API_FALLBACK_2])
            .output()
            .ok()?;

        if output.status.success() {
            let ip = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !ip.is_empty() {
                return Some(ip);
            }
        }

        if attempt == 0 {
            thread::sleep(std::time::Duration::from_millis(constants::RETRY_DELAY_MS));
        }
    }
    None
}

/// Try ifconfig.me API (IP only) with retry
fn try_ifconfig_api() -> Option<String> {
    let timeout = constants::API_TIMEOUT_SECS.to_string();

    for attempt in 0..constants::RETRY_ATTEMPTS {
        let output = std::process::Command::new("curl")
            .args(["-s", "--max-time", &timeout, constants::IP_API_FALLBACK_3])
            .output()
            .ok()?;

        if output.status.success() {
            let ip = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !ip.is_empty() {
                return Some(ip);
            }
        }

        if attempt == 0 {
            thread::sleep(std::time::Duration::from_millis(constants::RETRY_DELAY_MS));
        }
    }
    None
}

/// Extracts a string value from a simple JSON object.
/// Looks for pattern `"key": "value"` and returns the value.
fn extract_json_string(json: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{key}\":");
    let start = json.find(&pattern)? + pattern.len();
    let rest = &json[start..];
    // Skip whitespace and find opening quote
    let rest = rest.trim_start();
    if !rest.starts_with('"') {
        return None;
    }
    let rest = &rest[1..]; // Skip opening quote
    let end = rest.find('"')?;
    Some(rest[..end].to_string())
}

/// Measures network latency, packet loss, and jitter by pinging reliable hosts.
fn fetch_latency(tx: &Sender<TelemetryUpdate>) {
    let tx_clone = tx.clone();
    thread::spawn(move || {
        let timeout = constants::PING_TIMEOUT_SECS.to_string();

        for target in constants::PING_TARGETS {
            for attempt in 0..constants::RETRY_ATTEMPTS {
                if let Ok(output) = std::process::Command::new("ping")
                    .args(["-c", "10", "-i", "0.2", "-t", &timeout, target])
                    .output()
                {
                    if output.status.success() {
                        let stdout = String::from_utf8_lossy(&output.stdout);

                        let mut latency_ms = 0u64;
                        let mut packet_loss = 0.0f32;
                        let mut jitter_ms = 0u64;

                        for line in stdout.lines() {
                            if line.contains("packet loss") {
                                if let Some(loss_str) = line.split(',').nth(2) {
                                    if let Some(percent_part) = loss_str.trim().split('%').next() {
                                        if let Ok(val) = percent_part.trim().parse::<f32>() {
                                            packet_loss = val;
                                        }
                                    }
                                }
                            }

                            // Handle both "min/avg/max/stddev" (Linux) and "round-trip min/avg/max/stddev" (macOS)
                            if line.contains("min/avg/max") {
                                // Find the = sign and parse what comes after
                                if let Some(eq_pos) = line.find('=') {
                                    let values_str = &line[eq_pos + 1..].trim();
                                    let values: Vec<&str> = values_str.split('/').collect();
                                    if values.len() >= 4 {
                                        // avg is index 1
                                        if let Ok(avg) = values[1].trim().parse::<f64>() {
                                            #[allow(
                                                clippy::cast_possible_truncation,
                                                clippy::cast_sign_loss
                                            )]
                                            {
                                                latency_ms = avg.max(0.0) as u64;
                                            }
                                        }
                                        // stddev is index 3, might have " ms" suffix
                                        let stddev_str = values[3].trim_end_matches(" ms").trim();
                                        if let Ok(stddev) = stddev_str.parse::<f64>() {
                                            #[allow(
                                                clippy::cast_possible_truncation,
                                                clippy::cast_sign_loss
                                            )]
                                            {
                                                jitter_ms = stddev.max(0.0) as u64;
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        if latency_ms > 0 {
                            let _ = tx_clone.send(TelemetryUpdate::Latency(latency_ms));
                            let _ = tx_clone.send(TelemetryUpdate::PacketLoss(packet_loss));
                            let _ = tx_clone.send(TelemetryUpdate::Jitter(jitter_ms));
                            return;
                        }
                    }
                }

                if attempt == 0 {
                    thread::sleep(std::time::Duration::from_millis(constants::RETRY_DELAY_MS));
                }
            }
        }

        let _ = tx_clone.send(TelemetryUpdate::Latency(0));
        let _ = tx_clone.send(TelemetryUpdate::PacketLoss(100.0));
        let _ = tx_clone.send(TelemetryUpdate::Jitter(0));
    });
}

/// Fetches DNS configuration and checks for IPv6 leaks.
fn fetch_security_info(tx: &Sender<TelemetryUpdate>) {
    let tx_clone = tx.clone();
    thread::spawn(move || {
        // Try multiple methods to get DNS server
        let dns = try_get_dns_resolv_conf()
            .or_else(try_get_dns_scutil)
            .or_else(try_get_dns_networksetup);

        if let Some(dns_server) = dns {
            let _ = tx_clone.send(TelemetryUpdate::Dns(dns_server));
        }

        // Check for IPv6 connectivity with multiple endpoints (indicates potential leak when VPN active)
        let mut is_leaking = false;
        for endpoint in constants::IPV6_CHECK_APIS {
            let output6 = std::process::Command::new("curl")
                .args(["-6", "-s", "--max-time", "2", endpoint])
                .output();
            if output6.map(|o| o.status.success()).unwrap_or(false) {
                is_leaking = true;
                break;
            }
        }
        let _ = tx_clone.send(TelemetryUpdate::Ipv6Leak(is_leaking));
    });
}

/// Try to get DNS from /etc/resolv.conf
fn try_get_dns_resolv_conf() -> Option<String> {
    let output = std::process::Command::new("grep")
        .args(["nameserver", "/etc/resolv.conf"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.lines().next()?;
    let dns = line.replace("nameserver", "").trim().to_string();
    if dns.is_empty() {
        return None;
    }
    Some(dns)
}

/// Try to get DNS from scutil (macOS)
fn try_get_dns_scutil() -> Option<String> {
    let output = std::process::Command::new("scutil")
        .args(["--dns"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("nameserver[0]") {
            if let Some(dns) = trimmed.split(':').nth(1) {
                let dns = dns.trim().to_string();
                if !dns.is_empty() {
                    return Some(dns);
                }
            }
        }
    }
    None
}

/// Try to get DNS from networksetup (macOS)
fn try_get_dns_networksetup() -> Option<String> {
    // First get the primary service
    let output = std::process::Command::new("networksetup")
        .args(["-listallnetworkservices"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Try common service names
    for service in ["Wi-Fi", "Ethernet", "USB 10/100/1000 LAN"] {
        if stdout.contains(service) {
            if let Ok(dns_output) = std::process::Command::new("networksetup")
                .args(["-getdnsservers", service])
                .output()
            {
                let dns_stdout = String::from_utf8_lossy(&dns_output.stdout);
                let first_line = dns_stdout.lines().next().unwrap_or("").trim();
                // Skip "There aren't any DNS Servers" message
                if !first_line.is_empty() && !first_line.contains("aren't") {
                    return Some(first_line.to_string());
                }
            }
        }
    }
    None
}

/// Network traffic statistics tracker.
///
/// Tracks cumulative byte counts and calculates per-second throughput rates.
#[derive(Default)]
pub struct NetworkStats {
    last_bytes_in: u64,
    last_bytes_out: u64,
}

impl NetworkStats {
    /// Updates network statistics by reading system interface data.
    ///
    /// Parses `netstat -ib` output on macOS to calculate network throughput.
    ///
    /// # Returns
    ///
    /// A tuple of (`bytes_down_per_second`, `bytes_up_per_second`).
    pub fn update(&mut self) -> (u64, u64) {
        let mut current_down = 0u64;
        let mut current_up = 0u64;

        if let Ok(output) = std::process::Command::new("netstat").args(["-ib"]).output() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let mut total_bytes_in: u64 = 0;
            let mut total_bytes_out: u64 = 0;

            for line in stdout.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                // netstat -ib format: Name Mtu Network Address Ipkts Ierrs Ibytes Opkts Oerrs Obytes
                if parts.len() >= 10 {
                    let iface = parts[0];
                    // Skip loopback interfaces
                    if iface.starts_with("lo") {
                        continue;
                    }
                    if let (Ok(ibytes), Ok(obytes)) =
                        (parts[6].parse::<u64>(), parts[9].parse::<u64>())
                    {
                        total_bytes_in += ibytes;
                        total_bytes_out += obytes;
                    }
                }
            }

            // Calculate rate (bytes per second since last tick)
            if self.last_bytes_in > 0 {
                current_down = total_bytes_in.saturating_sub(self.last_bytes_in);
                current_up = total_bytes_out.saturating_sub(self.last_bytes_out);
            }
            self.last_bytes_in = total_bytes_in;
            self.last_bytes_out = total_bytes_out;
        }

        (current_down, current_up)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_json_string_ip() {
        let json = r#"{"ip": "1.2.3.4", "org": "Test ISP"}"#;
        assert_eq!(extract_json_string(json, "ip"), Some("1.2.3.4".to_string()));
    }

    #[test]
    fn test_extract_json_string_org() {
        let json = r#"{"ip": "1.2.3.4", "org": "AS12345 Test Company"}"#;
        assert_eq!(
            extract_json_string(json, "org"),
            Some("AS12345 Test Company".to_string())
        );
    }

    #[test]
    fn test_extract_json_string_missing_key() {
        let json = r#"{"ip": "1.2.3.4"}"#;
        assert_eq!(extract_json_string(json, "org"), None);
    }

    #[test]
    fn test_extract_json_string_with_whitespace() {
        let json = r#"{"ip":   "1.2.3.4"}"#;
        assert_eq!(extract_json_string(json, "ip"), Some("1.2.3.4".to_string()));
    }

    #[test]
    fn test_extract_json_string_empty() {
        let json = r"{}";
        assert_eq!(extract_json_string(json, "ip"), None);
    }

    #[test]
    fn test_network_stats_new() {
        let stats = NetworkStats::default();
        assert_eq!(stats.last_bytes_in, 0);
        assert_eq!(stats.last_bytes_out, 0);
    }

    #[test]
    fn test_network_stats_initial_update() {
        let mut stats = NetworkStats::default();
        let (down, up) = stats.update();
        // First update should return 0 (no previous baseline)
        assert_eq!(down, 0);
        assert_eq!(up, 0);
    }
}
