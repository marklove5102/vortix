//! VPN connection state types.

use std::time::Instant;

/// Technical details parsed from the VPN interface.
///
/// Contains network addresses, transfer statistics, and cryptographic information.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DetailedConnectionInfo {
    /// System interface name (e.g., utun3, wg0).
    pub interface: String,
    /// Internal IP address assigned by the VPN.
    pub internal_ip: String,
    /// Remote server endpoint (IP:port).
    pub endpoint: String,
    /// Maximum Transmission Unit size.
    pub mtu: String,
    /// `WireGuard` public key (empty for `OpenVPN`).
    pub public_key: String,
    /// Local listening port.
    pub listen_port: String,
    /// Total bytes received.
    pub transfer_rx: String,
    /// Total bytes transmitted.
    pub transfer_tx: String,
    /// Time since last successful handshake.
    pub latest_handshake: String,
    /// Process ID (for targeted termination).
    pub pid: Option<u32>,
}

/// VPN connection state machine.
///
/// Represents the current state of the VPN connection.
/// Scanner is the source of truth and will override Connecting/Disconnecting
/// states based on actual system state.
#[derive(Clone, PartialEq, Default)]
pub enum ConnectionState {
    /// No active VPN connection.
    #[default]
    Disconnected,
    /// Connection attempt in progress.
    Connecting {
        /// When the connection attempt started.
        started: Instant,
        /// Name of the profile being connected.
        profile: String,
    },
    /// Active VPN connection established.
    Connected {
        /// When the connection was established.
        since: Instant,
        /// Name of the connected profile.
        profile: String,
        /// Geographic location of the server.
        server_location: String,
        /// Current latency in milliseconds.
        latency_ms: u64,
        /// Detailed connection information.
        details: Box<DetailedConnectionInfo>,
    },
    /// Disconnection in progress.
    Disconnecting {
        /// When the disconnection attempt started.
        started: Instant,
        /// Name of the profile being disconnected.
        profile: String,
    },
}
