use std::collections::HashMap;
use std::sync::LazyLock;

use std::sync::Mutex;

#[derive(Default, Debug, Clone)]
struct ModuleRegistry {
    pub(crate) map: HashMap<String, String>,
}

static MODULE_REGISTRY: LazyLock<Mutex<ModuleRegistry>> = LazyLock::new(|| Mutex::new(ModuleRegistry::default()));

impl ModuleRegistry {
    fn register_module(&mut self, name: String, path: String) {
        self.map.insert(name, path);
    }

    fn get_module_path(&self, name: &str) -> Option<&String> {
        self.map.get(name)
    }
}

macro_rules! register_module {
    ($name:expr, $path:expr) => {
        #[ctor::ctor]
        fn register_module(){
            let mut registry = MODULE_REGISTRY.lock().expect("Failed to get map");
            registry.register_module($name.to_string(), $path.to_string());
        }
    };
}

register_module!("ping", "modules::echo_module::PingModule");
