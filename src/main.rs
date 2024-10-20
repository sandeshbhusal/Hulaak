use std::fs::read_to_string;

use anyhow::Result;
use clap::Parser;
use configuration::global_configuration::GlobalConfiguration;
use modules::manager::Manager;
mod configuration;
mod messaging;
mod modules;

#[derive(clap::Parser, Debug)]
struct Configuration {
    #[clap(
        short = 'c',
        long = "config",
        default_value = "config.toml",
        help = "Path to the configuration file"
    )]

    configuration_file: String,
}

fn main() -> Result<()> {
    let configuration = Configuration::parse();
    let config_contents =
        read_to_string(configuration.configuration_file).expect("error reading configuration toml");

    // parse the global configuration.
    let configuration: GlobalConfiguration = toml::from_str(&config_contents)?;

    // Start an executor for our "manager" module, and block on it.
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async {
            match Manager::new(configuration).run().await {
                Ok(_) => {
                    println!("Running manager")
                }
                Err(_) => {
                    panic!("Could  not run manager module")
                }
            }
        });

    Ok(())
}
