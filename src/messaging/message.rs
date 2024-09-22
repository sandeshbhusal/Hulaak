#![allow(unused)]

use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Message {
    fields: HashMap<Value, Value>,
}

impl Message {
    pub fn new(fields: HashMap<Value, Value>) -> Self {
        Message { fields }
    }
}
