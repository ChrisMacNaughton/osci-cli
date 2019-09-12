extern crate osci_cli;

use std::path::PathBuf;

use failure::Error;
use log::{debug, Level};
use env_logger::Builder;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Activate debug mode, options are: trace, debug, info, warn, error
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(
        short = "l",
        long = "--log-level",
        // help = "Verbosity level filter of the logger",
        default_value = "warn"
    )]
    pub log_level: Level,
    /// Path to the configuration file
    #[structopt(short = "c", long = "--config")]
    pub config_path: Option<PathBuf>,
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Gerrit(Gerrit),
}

#[derive(Debug, StructOpt)]
enum Gerrit {
    Check {
        /// ID of the Gerrit Review to check
        review_id: usize,
    },
}

fn main() -> Result<(), Error> {
    let opt = {
        let mut opt = Opt::from_args();
        if opt.config_path.is_none() {
            opt.config_path = Some(config_path())
        }
        opt
    };
    // init_with_level(opt.log_level).expect("Couldn't setup logger");
    Builder::new()
        .parse_filters(&format!("warn,osci_cli={}", opt.log_level))
        .init();
    debug!("CLI options: {:?}", opt);
    let config: osci_cli::Config = osci_cli::Config::load(&opt.config_path.unwrap())?;
    debug!("Config: {:?}", config);
    match opt.command {
        Command::Gerrit(Gerrit::Check { review_id }) => {
            osci_cli::check_gerrit_review(review_id, &config)?
        }
    }
    Ok(())
}

fn config_path() -> PathBuf {
    xdg::BaseDirectories::with_prefix("osci-cli")
        .expect("Couldn't identify base directory")
        .place_config_file("config.toml")
        .expect("Couldn't find base path")
}
