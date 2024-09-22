use serde::{Deserialize, Serialize};

pub mod lib;

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Events {
    Modify,
    Close,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Configuration {
    file_path: String,
    watch_for: Vec<Events>,
}