use serde::{Deserialize, Serialize};

pub mod config;
pub mod init;
pub mod run;

/// An instance of a running daemon. Contains state for that specific daemon
#[derive(Debug)]
pub struct Daemon {
    pub config: Config,
    pub debug: bool,
    pub acknowledged_alerts: Vec<String>,
}

/// Config for a daemon
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub update_interval: u64,
    pub lat: f64,
    pub lon: f64,
    pub detailed_notification: bool,
    pub notification_icon_path: Option<String>,
    pub user_agent: String,
}
