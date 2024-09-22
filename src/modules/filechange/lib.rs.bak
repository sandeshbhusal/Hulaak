use std::collections::HashMap;

use async_channel::Sender;
use inotify::WatchMask;

use crate::{
    configuration::module_configuration::ModuleConfiguration,
    messaging::message::Message,
    modules::{filechange::Configuration, module::ModuleTrait},
};

use super::Events;

pub(crate) struct FileChangeWatcherModule {
    path: String,
    watch_for: Vec<Events>,
    pub outbox: Option<Sender<Message>>,
}

impl ModuleTrait for FileChangeWatcherModule {
    fn new(configuration: ModuleConfiguration) -> Self
    where
        Self: Sized,
    {
        let _config = configuration.module_settings.clone();
        let serialized_config = serde_json::to_string(&_config).unwrap();

        // Convert the serialized config to module config.
        let module_config: Configuration = serde_json::from_str(&serialized_config)
            .expect("Error configuring the filechange module");

        FileChangeWatcherModule {
            outbox: None, // outbox is populated later by the controller.
            watch_for: module_config.watch_for,
            path: module_config.file_path,
        }
    }

    fn run(self: Box<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            // loop {
                // Wait until inotify says the file has changed.
                // Then, send a message to the outbox.
                // let mut outbox = self.outbox.clone();

                // let mut notify =
                //     inotify::Inotify::init().expect("Module failed to initialize inotify");
                // let mut watches = notify.watches();

                // for event in self.watch_for.iter() {
                //     let _ = match event {
                //         Events::Modify => {
                //             watches.add(self.path.clone(), WatchMask::MODIFY).unwrap()
                //         }
                //         Events::Close => watches.add(self.path.clone(), WatchMask::CLOSE).unwrap(),
                //     };
                // }

                // let mut buffer = Vec::with_capacity(1024);
                // while let Ok(events) = notify.read_events(&mut buffer) {
                //     for _ in events {
                //         let message = Message::new(HashMap::new());

                //         // Send the message to the outbox.
                //         if let Some(outbox) = &mut outbox {
                //             outbox.send(message).await.unwrap();
                //         }
                //     }
                // }
            // }
        })
    }

    fn get_name(&self) -> &'static str {
        "filechangewatcher"
    }

    fn set_outbox(&mut self, outbox: Option<Sender<crate::messaging::message::Message>>) {
        self.outbox = outbox;
    }

    fn set_inbox(
        &mut self,
        _inbox: Option<async_channel::Receiver<crate::messaging::message::Message>>,
    ) {
        unimplemented!("FileChangeWatcherModule does not need an inbox. Error in wiring?");
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
