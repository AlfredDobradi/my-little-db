use std::collections::HashMap;
use crate::record::{Field, Record, Value, ValueType};

#[derive(Clone, Debug)]
pub struct FieldDefinition {
    name: String,
    kind: ValueType,
}

impl FieldDefinition {
    pub fn new(name: &str, kind: ValueType) -> FieldDefinition {
        FieldDefinition{
            name: name.to_string(),
            kind,
        }
    }
}

pub struct Schema {
    primary_key_fields: Vec<String>,
    fields: HashMap<String, FieldDefinition>
}

impl Schema {
    pub fn new(primary_key_fields: Vec<&str>, fields: Vec<FieldDefinition>) -> Schema {
        let mut schema_fields = HashMap::new();
        for field in fields {
            schema_fields.insert(field.name.clone(), field);
        }

        let pk_fields = primary_key_fields.into_iter().map(String::from).collect();

        Schema {
            primary_key_fields: pk_fields,
            fields: schema_fields,
        }
    }

    pub fn new_record(&self, values: Vec<Field>) -> anyhow::Result<Record> {
        let mut primary_key: HashMap<String, Field> = HashMap::new();
        let mut record_values: HashMap<String, Field> = HashMap::new();

        for field in values {
            let name = field.name.clone();
            if self.fields.contains_key(&name) && self.primary_key_fields.contains(&name) {
                primary_key.insert(name.clone(), field.clone());
            }

            if let Some(schema_field) = self.fields.get(&name) {
                let vc = field.value.clone();
                match schema_field.kind {
                    ValueType::String => if let Value::String(_) = vc {} else { return Err(anyhow::format_err!("invalid value type for {}", field.name)) }
                    ValueType::Integer => if let Value::Integer(_) = vc {} else { return Err(anyhow::format_err!("invalid value type for {}", field.name)) }
                    ValueType::Float => if let Value::Float(_) = vc {} else { return Err(anyhow::format_err!("invalid value type for {}", field.name)) },
                    ValueType::Bool => if let Value::Bool(_) = vc {} else { return Err(anyhow::format_err!("invalid value type for {}", field.name)) }
                    _ => return Err(anyhow::format_err!("invalid value type for {}", field.name)),
                }
                record_values.insert(name, field);
            }
        }

        Ok(Record {
            primary_key,
            values: record_values,
        })
    }
}