use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::record::{Field, Record, Value, ValueType};

#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Clone, Debug, Serialize, Deserialize)]
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

        for (_, field) in self.fields.clone() {
            let val = Value::get_default(field.kind);

            let name = field.name;
            record_values.insert(name.clone(), Field::new(name.clone().as_str(), val));
        }

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

    pub fn fields(&self) -> &HashMap<String, FieldDefinition> {
        &self.fields
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_schema() -> Schema {
        Schema::new(
            vec!["id", "timestamp"],
            vec![
                FieldDefinition::new("id", ValueType::Integer),
                FieldDefinition::new("timestamp", ValueType::Integer),
                FieldDefinition::new("duration", ValueType::Float),
                FieldDefinition::new("filename", ValueType::String),
                FieldDefinition::new("status", ValueType::Integer),
            ]
        )
    }

    #[test]
    fn test_new_record_correct_fields() {
        let schema = test_schema();

        let record = schema.new_record(
            vec![
                Field::new("id", Value::Integer(2)),
                Field::new("timestamp", Value::Integer(3)),
                Field::new("duration", Value::Float(3.5)),
                Field::new("filename", Value::String(String::from("filename"))),
                Field::new("status", Value::Integer(1)),
            ]
        ).unwrap();

        assert_eq!(record.primary_key.len(), 2);
        assert!(record.values.contains_key("id"));
        assert_eq!(record.values.get("id").unwrap().value, Value::Integer(2));
        assert!(record.values.contains_key("timestamp"));
        assert_eq!(record.values.get("timestamp").unwrap().value, Value::Integer(3));
        assert!(record.values.contains_key("duration"));
        assert_eq!(record.values.get("duration").unwrap().value, Value::Float(3.5));
        assert!(record.values.contains_key("filename"));
        assert_eq!(record.values.get("filename").unwrap().value, Value::String("filename".to_string()));
        assert!(record.values.contains_key("status"));
        assert_eq!(record.values.get("status").unwrap().value, Value::Integer(1));
    }

    #[test]
    fn test_new_record_default_fields() {
        let schema = test_schema();

        let record = schema.new_record(
            vec![
                Field::new("id", Value::Integer(2)),
                Field::new("timestamp", Value::Integer(3)),
                Field::new("duration", Value::Float(3.5)),
            ]
        ).unwrap();

        assert!(record.values.contains_key("filename"));
        assert_eq!(record.values.get("filename").unwrap().value, Value::String("".to_string()));
        assert!(record.values.contains_key("status"));
        assert_eq!(record.values.get("status").unwrap().value, Value::Integer(0));
    }

    #[test]
    fn test_new_record_unrecognized_fields() {
        let schema = test_schema();

        let record = schema.new_record(
            vec![
                Field::new("id", Value::Integer(2)),
                Field::new("timestamp", Value::Integer(3)),
                Field::new("duration", Value::Float(3.5)),
                Field::new("filename", Value::String(String::from("filename"))),
                Field::new("status", Value::Integer(1)),
                Field::new("foo", Value::String(String::from("bar"))),
            ]
        ).unwrap();

        assert!(!record.values.contains_key("foo"));
    }
}