use core::time;
use std::thread;

use crate::{
    daemon::Daemon,
    print_debug, print_error, print_info,
    weather::weather::{extract_weather_features, send_notification},
};

pub fn run(mut daemon: Daemon) {
    print_info!("Starting up daemon");

    print_info!("{:?}", daemon);

    let honk_shooo = time::Duration::from_millis(1000 * daemon.config.update_interval);
    loop {
        thread::sleep(honk_shooo);
        let url = format!(
            "https://api.weather.gov/alerts/active?point={},{}",
            daemon.config.lat, daemon.config.lon
        );
        print_info!("Updating weather service information from {}", url);

        // Send GET request to the API
        let client_builder = reqwest::blocking::Client::builder();

        let client = client_builder
            .user_agent(&daemon.config.user_agent)
            .build()
            .expect("Could not build request client"); // TODO: REPLACE WITH CONFIG OPTION
        let response = match client.get(url).send() {
            Ok(r) => r,
            Err(e) => {
                print_error!("Failed to fetch data. Response: {}", e.to_string());
                continue;
            }
        };

        // Ensure the request was successful
        if !response.status().is_success() {
            print_error!(
                "Failed to fetch data. Response: {}",
                response.text().unwrap_or("None".to_string())
            );
            continue;
        }

        let response_text = response.text().expect("Failed to unwrap response text");

        if daemon.debug {
            print_debug!("Response text {:?}", response_text);
        }

        let weather_features = extract_weather_features(response_text);

        for feature in weather_features {
            if daemon.acknowledged_alerts.contains(&feature.properties.id) {
                continue;
            }

            daemon
                .acknowledged_alerts
                .push(feature.properties.id.clone());

            println!("{}", feature);

            // feature.properties.severity = Severity::Extreme;

            send_notification(&feature, &daemon.config);
        }
    }
}
