use crate::{
    daemon::Config,
    print_done, print_info,
    weather::weather::{generate_test_alert, send_notification, Severity},
};

pub fn test_alert(config: &Config, severity: &Severity) {
    let alert = generate_test_alert(severity);
    print_info!("Generating test alert {:?}", &alert);
    send_notification(&alert, &config);
    print_done!("Sent a test alert of severity {}", severity);
}
