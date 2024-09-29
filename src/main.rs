use anyhow::Result;
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
    // Let's just use a global configuration for now.
    let _global_configuration = r#"
        [modules]
        [modules.echo_file]
        module_type = "echo"

        [modules.udp_sock_list]
        module_type = "udpsocketlistener"
        address = "0.0.0.0"
        port = 8080
        buffer_size = 1024

        [modules.tcp_socket_check]
        module_type = "tcpsocketlistener"
        address = "0.0.0.0"
        port = 8081
        buffer_size = 4096

        [routes]
        [routes.simple_echo_from_file]
        from = { Multiple = ["udp_sock_list", "tcp_socket_check"]}
        to = { Single = "echo_file"}
        "#;

    // parse the global configuration.
    let configuration: GlobalConfiguration = toml::from_str(_global_configuration)?;

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
