use futures::stream::FuturesUnordered;
use futures::StreamExt;
use std::collections::{HashMap, HashSet};
use tokio::task::JoinHandle;

use crate::configuration::global_configuration::GlobalConfiguration;
use crate::messaging::message::Message;
use crate::modules::registry::ModulesRegistry;

pub struct Manager {
    configuration: GlobalConfiguration,
}

impl Manager {
    pub fn new(configuration: GlobalConfiguration) -> Self {
        Manager { configuration }
    }

    pub fn run(self) -> JoinHandle<()> {
        let configuration = self.configuration; // Move configuration out of self

        tokio::spawn(async move {
            // Parse the configuration for the modules, and start them.
            let modconfigs = configuration.modules;
            let mut modules: HashMap<String, Box<dyn crate::modules::module::ModuleTrait>> =
                HashMap::new();

            for (init_name, configuration) in modconfigs {
                let module =
                    ModulesRegistry::get_module(&configuration.module_type.clone(), configuration);

                modules.insert(init_name, module);
            }

            // All modules are ready to run. Next, we generate the routes to push
            // to the modules. We will use senders/receivers from async-channel crate.
            let (sender, receiver) = async_channel::unbounded::<Message>();

            let mut routable_modules = HashSet::new();

            for (name, route_config) in configuration.routes {
                // Check if we have the modules from "from" and "to".
                for module_name in route_config.from.get_modules().into_iter() {
                    if !modules.contains_key(&module_name) {
                        panic!("Module {} not found for route {}", module_name, name);
                    }

                    let module = modules.get_mut(&module_name).unwrap();
                    module.set_outbox(Some(sender.clone()));
                    routable_modules.insert(module_name);
                }

                for module_name in route_config.to.get_modules().into_iter() {
                    if !modules.contains_key(&module_name) {
                        panic!("Module {} not found for route {}", module_name, name);
                    }

                    let module = modules.get_mut(&module_name).unwrap();
                    module.set_inbox(Some(receiver.clone()));
                    routable_modules.insert(module_name);
                }
            }

            // We can fire up the modules now! Exciting stuff going to happen next!
            // We will use futuresunordered to run all the modules concurrently.

            let mut handles = FuturesUnordered::new();
            for (name, module) in modules.drain() {
                if routable_modules.contains(&name) {
                    handles.push(module.run());
                    println!("Module {} is running", name);
                } else {
                    println!("Module {} is not configured, but has no routes for it. It will not be run.", name);
                }
            }

            while let Some(handle) = handles.next().await {
                match handle {
                    Ok(_) => {
                        println!("Module finished successfully");
                    }
                    Err(e) => {
                        println!("Module failed with error: {:?}", e);
                    }
                }
            }
        })
    }
}
