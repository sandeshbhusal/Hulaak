use anyhow::Result;
use configuration::global_configuration::GlobalConfiguration;
use modules::manager;
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
    // Let's just use a global configuration for now.
    let _global_configuration = r#"
        [modules]
        [modules.filechange_file]
        module = "filechangewatcher"

        [modules.filechange_file.module_settings]
        path = "/tmp/file"

        [modules.echo_file]
        module = "echo"

        [routes]
        [routes.simple_echo_from_file]
        from = { Single = "filechange_file"}
        to = { Single = "echo_file"}
        "#;

    // parse the global configuration.
    let configuration: GlobalConfiguration = toml::from_str(_global_configuration)?;

    // Start an executor for our "manager" module, and block on it.
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(async {
            let manager = manager::manager::Manager::new(configuration);
            manager.run().await;
        });

    Ok(())
}
