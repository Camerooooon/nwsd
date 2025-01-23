# NWSD

A daemon for receiving desktop National Weather Service notifications through the use of the National Weather Service API ([api.weather.gov](https://api.weather.gov)).

## Features

- Periodically fetches real-time weather alerts from the National Weather Service based on a specific latitude and longitude.
- Sends notifications for severe weather alerts based on NWS data.

## Requirements

- Rust (latest stable version recommended)
- Network access to connect to [api.weather.gov](https://api.weather.gov)

## Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd nwsd
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Run the binary:
   ```bash
   ./target/release/nwsd
   ```

It is recommended to add the nwsd command to run when you start your desktop environment

## Usage

### Command-Line Interface

`nwsd` provides the following commands:

- **`run`**
  Starts the daemon to fetch and process weather notifications.
  ```bash
  nwsd run [--config-directory <path>] [--debug]
  ```
  **Options:**
  - `--config-directory`: Specify a custom directory for the configuration file.
  - `--debug`: Enable debug mode to log detailed information.

- **`init-config`**
  Creates a default configuration file. If the configuration directory is not specified, it will use the default path.
  ```bash
  nwsd init-config [--config-directory <path>]
  ```

- **`version`**
  Displays the current version of the application.
  ```bash
  nwsd version
  ```

### Example

1. Initialize the configuration file:
   ```bash
   nwsd init-config
   ```
2. Start the daemon:
   ```bash
   nwsd run --debug
   ```

## Configuration

The configuration file is used to customize the behavior of the daemon. By default, the configuration file is created in a standard directory (e.g., `$HOME/.nwsd/config.toml`), but a custom directory can be specified during initialization or runtime.

### Example Configuration (`config.toml`):

```toml
# Configuration file for NWSD
update_interval = 300 # How often to check the API for weather alerts
lat = 36.974117 # The latitute to check
lon = -122.030792 # The longitude to check
detailed_notification = false # If notifications will contain alert description instead of headline
# notification_icon_path = "/usr/share/icons/someicon.svg" # The notification icon path or comment out to use Papirus notifications
user_agent = "nwsd notification app (https://github.com/Camerooooon/nwsd)"
```

#### User Agent

The NWS API suggests a descriptive user-agent that is unique to every application. It is recommended to make your user-agent unique to prevent rate limiting.

## Development

### Project Structure

- **`commands/`**: Contains the CLI commands
- **`daemon/`**: Core daemon logic for running the service.
- **`weather/`**: Handles weather-related processing, including parsing and notifications.

### Adding New Features
1. Fork the repository.
2. Create a new branch for your feature:
   ```bash
   git checkout -b feature-name
   ```
3. Implement your changes and ensure the code is well-documented.
4. Test your changes:
5. Submit a pull request.

## Disclaimer

This software is not affiliated with or endorsed by the National Weather Service.
