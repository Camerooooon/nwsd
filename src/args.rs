use structopt::StructOpt;

use crate::commands::version;
use crate::daemon::{self, config};
use crate::print_warn;

#[derive(StructOpt)]
#[structopt(
    name = "nwsd",
    about = "A daemon for sending/retreiving national weather service notifications through the use of the nation weather service api (api.weather.gov)"
)]
enum NWSDCommand {
    #[structopt(name = "run", alias = "r")]
    Run {
        #[structopt(name = "config directory")]
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
}

pub fn parse_args() {
    match NWSDCommand::from_args() {
        NWSDCommand::Run {
            config_directory,
            debug,
        } => {
            let config_option = config::load_config_from_file(config_directory);

            let config = match config_option {
                Some(config) => config,
                None => {
                    print_warn!("You have not initalized the config file yet. Please run nwsd init-config to create a config file. Reverting to default config.");
                    crate::daemon::Config::default()
                }
            };
            let daemon = daemon::init::init_daemon(config, debug);
            daemon::run::run(daemon)
        }
        NWSDCommand::Version {} => version::print_version(),
        NWSDCommand::InitConfig { config_directory } => {
            config::create_default_config(config_directory)
        }
    }
}
