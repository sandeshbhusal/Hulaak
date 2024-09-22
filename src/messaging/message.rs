use serde_json::Value;
use std::collections::HashMap;

macro_rules! impl_from_for_values {
    ($(($t:ty, $variant:ident, $default:expr)),*) => {
        $(
            impl From<$t> for Values {
                fn from(value: $t) -> Self {
                    Values::$variant(value)
                }
            }

            impl From<Values> for $t {
                fn from(value: Values) -> Self {
                    match value {
                        Values::$variant(value) => value,
                        _ => $default
                    }
                }
            }
        )*
    };
}

pub enum Values {
    String(String),
    Number(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<Value>),
    Object(HashMap<Value, Value>),
    Null,
}

impl_from_for_values!(
    (String, String, String::new()),
    (i64, Number, 0),
    (f64, Float, 0.0),
    (bool, Boolean, false),
    (Vec<Value>, Array, Vec::new()),
    (HashMap<Value, Value>, Object, HashMap::new())
);

impl From<()> for Values {
    fn from(_: ()) -> Self {
        Values::Null
    }
}

impl From<Values> for () {
    fn from(_: Values) -> Self {
        ()
    }
}

pub struct Message {
    fields: HashMap<Values, Values>,
}

impl Message {
}
