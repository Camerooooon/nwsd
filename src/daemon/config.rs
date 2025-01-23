use std::{
    fs::File,
    io::{ErrorKind, Read, Write},
    path::{Path, PathBuf},
};

use crate::{daemon::Config, print_done, print_fatal, print_info};

fn config_exists(path: &Path) -> bool {
    path.exists()
}

fn check_direcotry(directory_string: Option<String>) -> PathBuf {
    match directory_string {
        Some(a) => PathBuf::from(a),
        None => {
            let mut path = dirs::config_dir().unwrap();
            path.push("nwsd.toml");
            path
        }
    }
}

pub fn load_config_from_file(directory_string: Option<String>) -> Option<Config> {
    let final_directory = check_direcotry(directory_string);

    print_info!("Loading config from: {}", final_directory.to_string_lossy());

    if !config_exists(final_directory.as_path()) {
        return None;
    }

    let read_text = match File::open(final_directory) {
        Ok(mut file) => {
            let mut read_config: String = "".to_string();
            match file.read_to_string(&mut read_config) {
                Ok(_) => {}
                Err(_) => {
                    print_fatal!("Could not open config for reading, reverting to default config.");
                    return None;
                }
            };
            read_config
        }
        Err(_) => {
            print_fatal!("Could not open config for reading, reverting to default config.");
            return None;
        }
    };
    Some(toml::from_str(read_text.as_str()).unwrap_or_default())
}

pub fn create_default_config(directory_string: Option<String>) {
    let final_directory = check_direcotry(directory_string);
    print_info!(
        "Attempting to generate default config at {}",
        final_directory.to_string_lossy()
    );
    let default_config = Config::default();
    let serialized = if let Ok(value) = toml::to_string(&default_config) {
        value
    } else {
        print_fatal!("Could not serialize default config. Please make an issue.");
        return;
    };
    let config_file = File::create(&final_directory);
    let mut config = match config_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::PermissionDenied => {
                print_fatal!(
                    "Looks like you don't have permission to write to {}.",
                    &final_directory.to_string_lossy()
                );
                return;
            }
            _ => {
                print_fatal!("Failed to create config file: {}", error);
                return;
            }
        },
    };

    config.write_all(serialized.as_bytes()).unwrap_or_else(|_| {
        print_fatal!(
            "Could not write serialized output to file {}",
            &final_directory.to_string_lossy()
        );
        return;
    });
    print_done!(
        "Created config file at {}",
        &final_directory.to_string_lossy()
    );
}

impl Default for Config {
    fn default() -> Self {
        Config {
            update_interval: 300,
            lat: 36.974117,
            lon: -122.030792,
            detailed_notification: false,
            notification_icon_path: None,
            user_agent: "nwsd notification app (https://github.com/Camerooooon/nwsd)".to_string(),
        }
    }
}
