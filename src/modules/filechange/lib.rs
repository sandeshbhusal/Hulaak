use async_channel::Sender;

use crate::{
    configuration::module_configuration::ModuleDefinition,
    modules::{filechange::Configuration, module::ModuleTrait},
};

pub(crate) struct FileChangeWatcherModule {
    pub outbox: Option<Sender<String>>
}

impl ModuleTrait for FileChangeWatcherModule {
    fn new(configuration: ModuleDefinition) -> Self
    where
        Self: Sized,
    {
        let _config = configuration.module_settings.clone();
        let serialized_config = serde_json::to_string(&_config).unwrap();

        // Convert the serialized config to module config.
        let _module_config: Configuration = serde_json::from_str(&serialized_config)
            .expect("Error configuring the filechange module");

        FileChangeWatcherModule {
            outbox: None, // outbox is populated later by the controller.
        }
    }

    fn run(self) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            // We do not care about the inbox.
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::modules::module::ModuleTrait;

    use super::FileChangeWatcherModule;

    // Check configuration parsing.
    #[test]
    fn test_configuration_parsing() {
        let global_config = r#"
            version = "1.3.0"
            local_port = 8080
            allow_sync = true

            [[modules]]
            name = "filechange_1"
            module = "filechange"
            module_type = "Input"
            description = "File Change Module"
            address_type = "Managed"

            [modules.module_settings]
            file_path = "/tmp"
            watch_for = ["Modify"]

            [[routes]]
            name = "some_route"
            from = { Single = "abc" }
            to = { Single = "def" }
        "#;

        // Parse configuration.
        let parsedconfig: crate::configuration::global_configuration::GlobalConfiguration =
            toml::from_str(global_config).unwrap();

        // Get first module definition, and try to generate FileChangeWatcherModule.
        let module_definition = parsedconfig.modules.first().unwrap();
        let _module = FileChangeWatcherModule::new(module_definition.clone());
    }
}