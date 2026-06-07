use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use rustc_hash::FxHasher;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    None,
    String(String),
    Integer(i64),
    Float(f64),
    Bool(bool),
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::None => {
                0u8.hash(state);
            }
            Value::String(s) => {
                1u8.hash(state);
                s.hash(state);
            }
            Value::Integer(i) => {
                2u8.hash(state);
                i.hash(state);
            }
            Value::Float(f) => {
                3u8.hash(state);
                f.to_bits().hash(state);
            }
            Value::Bool(b) => {
                4u8.hash(state);
                b.hash(state);
            }
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::None => write!(f, ""),
            Value::String(string) => write!(f, "{}", string),
            Value::Integer(i) => write!(f, "{}", i),
            Value::Float(float) => write!(f, "{:?}", float), // debug is easier than formatting float
            Value::Bool(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ValueType {
    None,
    String,
    Integer,
    Float,
    Bool,
}

#[derive(Clone, Debug, PartialEq)]
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

pub struct PrimaryKey {
    pub fields: HashMap<String, Field>,
    pub hash: u64,
}

impl PrimaryKey {
    pub fn new(fields: HashMap<String, Field>) -> PrimaryKey {
        let mut hasher = FxHasher::default();
        let field_names: Vec<&String> = fields.keys().collect();
        for field in field_names {
            fields[field].value.hash(&mut hasher);
        }

        PrimaryKey {
            fields,
            hash: hasher.finish(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Record {
    pub primary_key: HashMap<String, Field>,
    pub values: HashMap<String, Field>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::record::Value;

    #[test]
    fn test_value_display() {
        assert_eq!(format!("{}", Value::None), "");
        assert_eq!(format!("{}", Value::String("1".to_string())), "1");
        assert_eq!(format!("{}", Value::Integer(1)), "1");
        assert_eq!(format!("{}", Value::Float(1.0)), "1.0");
        assert_eq!(format!("{}", Value::Bool(true)), "true");
    }

    #[test]
    fn test_new_primary_key() {
        let values: HashMap<String, Field> = HashMap::from(
            [
                ("id".to_string(), Field{ name: "id".to_string(), value: Value::Integer(1) }),
                ("sub_id".to_string(), Field{ name: "sub_id".to_string(), value: Value::Integer(1) }),
            ]
        );

        let pk = PrimaryKey::new(values);

        assert_eq!(pk.hash, 7011865262788786561);
        assert_eq!(pk.fields.len(), 2);
        assert_eq!(pk.fields.get("id").unwrap(), &Field{ name: "id".to_string(), value: Value::Integer(1) });
    }
}