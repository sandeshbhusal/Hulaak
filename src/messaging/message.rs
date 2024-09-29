#![allow(unused)]

use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Message {
    pub(crate) fields: HashMap<String, Value>,
}

impl Message {
    pub fn new(fields: HashMap<String, Value>) -> Self {
        Message { fields }
    }
}
