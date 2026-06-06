use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Value {
    None,
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ValueType {
    None,
    String,
    Integer,
    Float,
    Bool,
}

#[derive(Clone, Debug)]
pub struct Field {
    pub name: String,
    pub value: Value,
}

impl Field {
    pub fn new(name: &str, value: Value) -> Field {
        Field {
            name: name.to_string(),
            value,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Record {
    pub primary_key: HashMap<String, Field>,
    pub values: HashMap<String, Field>,
}