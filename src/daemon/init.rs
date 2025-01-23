use crate::daemon::Config;
use crate::daemon::Daemon;

pub fn init_daemon(config: Config, debug: bool) -> Daemon {
    Daemon {
        config,
        debug,
        acknowledged_alerts: Vec::new(),
    }
}
