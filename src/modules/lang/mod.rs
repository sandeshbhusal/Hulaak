use std::{collections::HashMap, fmt::Debug};

use anyhow::Error;

use crate::messaging::message::Message;

struct FunctionCall {
    pub inner: Box<dyn FnOnce(Context, Vec<Param>) -> Result<Param, Error>>,
}

impl Debug for FunctionCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MessageMutator").finish()
    }
}

impl FunctionCall {
    fn new<T>(function: T) -> Self
    where
        T: FnOnce(Context, Vec<Param>) -> Result<Param, Error> + 'static,
    {
        FunctionCall {
            inner: Box::new(function),
        }
    }
}

#[derive(Default, Debug)]
enum Param {
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Function(FunctionCall),
    Message(Message),

    #[default]
    Nil,
}

impl From<Param> for serde_json::Value {
    fn from(value: Param) -> Self {
        match value {
            Param::String(val) => serde_json::value::Value::String(val),
            Param::Int(val) => serde_json::value::Value::Number(serde_json::Number::from(val)),
            Param::Float(val) => serde_json::value::Value::Number(serde_json::Number::from_f64(val).unwrap()),
            Param::Bool(val) => serde_json::value::Value::Bool(val),

            // Nulls and functions are encoded as nulls.
            _ => serde_json::Value::Null,
        }
    }
}

impl Param {
    pub fn eval(self) -> Result<Param, Error> {
        match self {
            Param::Function(f) => (f.inner)(Context::default(), vec![]),
            _ => Ok(self),
        }
    }

    pub fn add(self, other: Param) -> Result<Param, Error> {
        match (&self, &other) {
            (Param::Int(a), Param::Int(b)) => Ok(Param::Int(a + b)),
            (Param::Float(a), Param::Float(b)) => Ok(Param::Float(a + b)),
            (Param::String(a), Param::String(b)) => Ok(Param::String(a.clone() + b)),
            _ => Err(anyhow::anyhow!("Cannot add {:?} and {:?}", self, other)),
        }
    }
}

#[derive(Default)]
struct Context {
    symbols: HashMap<String, Param>,
}

pub struct HulangModule {
    compiled_funcs: HashMap<String, FunctionCall>,
}

impl HulangModule {
    pub fn new() -> Self {
        let mut compiled_funcs: HashMap<String, FunctionCall> = Default::default();

        compiled_funcs.insert("drop".into(), FunctionCall::new(drop_key));
        compiled_funcs.insert("add".into(), FunctionCall::new(add_key_value));

        HulangModule { compiled_funcs }
    }
}

fn drop_key(ctx: Context, params: Vec<Param>) -> Result<Param, Error> {
    assert!(matches!(params[0], Param::Message(_)));

    Ok(Param::default())
}

fn add_key_value(ctx: Context, params: Vec<Param>) -> Result<Param, Error> {
    assert!(params.len() == 3);
    assert!(matches!(params[0], Param::Message(_)));

    let key = params[1].eval()?;
    let value = params[2].eval()?;

    let mut message = match params[0] {
        Param::Message(m) => m,
        _ => unreachable!("Should be a message"),
    };

    message.fields.insert(key.eval().unwrap().into(), value.eval().unwrap().into());
    Ok(Param::default())
}
