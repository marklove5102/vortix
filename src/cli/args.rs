//! Command-line argument definitions.

use clap::{Parser, Subcommand};

/// Vortix - Professional TUI VPN Manager
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Available CLI commands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Import VPN profile(s) from a file, directory, or URL
    Import {
        /// Path to a .conf/.ovpn file, directory, or a URL (http/https)
        file: String,
    },
    /// Update vortix to the latest version from crates.io
    Update,
    /// Emergency release of kill switch (use if locked out)
    ReleaseKillSwitch,
}
