use structopt::StructOpt;

use crate::commands::{test, version};
use crate::daemon::{self, config, Config};
use crate::print_warn;
use crate::weather::weather::{AlertProperties, Severity};

#[derive(StructOpt)]
#[structopt(
    name = "nwsd",
    about = "A daemon for sending/retreiving national weather service notifications through the use of the nation weather service api (api.weather.gov)"
)]
enum NWSDCommand {
    #[structopt(name = "run", alias = "r")]
    Run {
        #[structopt(short, long, name = "config directory")]
        config_directory: Option<String>,

        #[structopt(short, long, name = "debug")]
        debug: bool,
    },
    #[structopt(name = "init-config")]
    InitConfig {
        #[structopt(name = "config directory")]
        config_directory: Option<String>,
    },
    #[structopt(name = "version", alias = "v")]
    Version {},
    #[structopt(name = "test")]
    Test {
        #[structopt(name = "severity", parse(try_from_str))]
        severity: Severity,

        #[structopt(short, long, name = "config directory")]
        config_directory: Option<String>,
    },
}

fn get_config(config_directory: Option<String>) -> Config {
    let config_option = config::load_config_from_file(config_directory);

    match config_option {
        Some(config) => config,
        None => {
            print_warn!("You have not initalized the config file yet. Please run nwsd init-config to create a config file. Reverting to default config.");
            crate::daemon::Config::default()
        }
    }
}

pub fn parse_args() {
    match NWSDCommand::from_args() {
        NWSDCommand::Run {
            config_directory,
            debug,
        } => {
            let config = get_config(config_directory);
            let daemon = daemon::init::init_daemon(config, debug);
            daemon::run::run(daemon)
        }
        NWSDCommand::Version {} => version::print_version(),
        NWSDCommand::InitConfig { config_directory } => {
            config::create_default_config(config_directory)
        }
        NWSDCommand::Test { severity, config_directory } => {
            println!("{:?}", severity);
            let config = get_config(config_directory);
            test::test_alert(&config, &severity)
        },
    }
}
