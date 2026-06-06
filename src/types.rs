use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use serde::Serialize;
use xxhash_rust::xxh3;

#[derive(Debug, Serialize, Clone)]
pub enum Value {
    None,
    String(String),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
}

#[derive(Debug, Clone)]
pub enum ValueType {
    None,
    String,
    Short,
    Int,
    Long,
    Float,
    Double,
}

impl Default for ValueType {
    fn default() -> Self {
        ValueType::None
    }
}

#[derive(Debug, Default, Clone)]
pub struct FieldDefinition {
    pub name: String,
    pub kind: ValueType,
}

impl From<(&str, ValueType)> for FieldDefinition {
    fn from(tuple: (&str, ValueType)) -> Self {
        let (name, kind) = tuple;

        Self{
            name: name.to_string(),
            kind,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Schema {
    primary_key: Vec<String>,
    fields: HashMap<String, FieldDefinition>,
}

impl Schema {
    pub fn new(primary_key: Vec<&str>, fields: Vec<(&str, ValueType)>) -> Result<Schema, anyhow::Error> {
        let mut existing_fields: Vec<&str> = Vec::new();

        let mut field_definitions: HashMap<String, FieldDefinition> = HashMap::new();
        let mut primary_keys: Vec<String> = Vec::new();

        for (name, kind) in fields {
            let field_definition = FieldDefinition::from((name, kind));
            if field_definition.name == "" {
                return Err(anyhow::anyhow!("Field name is missing"));
            }

            existing_fields.push(name);

            field_definitions.insert(name.to_string(), field_definition);
        }

        for key in &primary_key {
            if !existing_fields.contains(key) {
                return Err(anyhow::anyhow!(format!("primary key field {} not found in fields", key)));
            }

            primary_keys.push(key.to_string());
        }

        Ok(Schema{
            primary_key: primary_keys,
            fields: field_definitions,
        })
    }
}

#[derive(Debug)]
pub struct Buffer {
    schema: Schema,
    records: HashMap<u64, Record>,
}

impl Buffer {
    pub fn new(schema: &Schema) -> Buffer {
        Buffer{schema: schema.clone(), records: HashMap::new()}
    }

    pub fn append(&mut self, record: Record) -> Result<(), anyhow::Error> {
        let key = record.primary_key.hash()?;
        self.records.insert(key, record);
        Ok(())
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct PrimaryKey {
    value: HashMap<String, Value>,
}

impl PrimaryKey {
    pub fn hash(&self) -> Result<u64, anyhow::Error> {
        let serialized = postcard::to_allocvec(&self).or(Err(anyhow::anyhow!("can't serialize primary key")))?;

        let hash = xxh3::xxh3_64(serialized.as_slice());
        Ok(hash)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FieldValue {
    field: String,
    value: Value,
}

impl FieldValue {
    pub fn parse(kind: ValueType, value: (&str, &str)) -> Result<FieldValue, anyhow::Error> {
        let (field, value) = value;

        tracing::info!(?value, "attempting to parse value");
        
        let mut v = Value::None;
        match kind {
            ValueType::String => { v = Value::String(value.to_string()); },
            ValueType::Short => { v = Value::Short(value.parse::<i16>()?); },
            ValueType::Int => { v = Value::Int(value.parse::<i32>()?); },
            ValueType::Long => { v = Value::Long(value.parse::<i64>()?); },
            ValueType::Float => { v = Value::Float(value.parse::<f32>()?); },
            ValueType::Double => { v = Value::Double(value.parse::<f64>()?); },
            ValueType::None => {},
        }

        Ok(FieldValue{
            field: field.to_string(),
            value: v,
        })
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Record {
    primary_key: PrimaryKey,
    fields: HashMap<String, FieldValue>,
}

impl Schema {
    pub fn new_record(&self, values: Vec<(&str, &str)>) -> Result<Record, anyhow::Error> {
        let mut fields: HashMap<String, FieldValue> = HashMap::new();
        let mut primary_key: PrimaryKey = PrimaryKey{value: HashMap::new()};
        for value in values {
            let (field_name, field_value) = value;
            let field_def = match self.fields.get(&field_name.to_string()) {
                None => {
                    return Err(anyhow::anyhow!(format!("Field not found: {}", field_name)));
                }
                Some(field) => field,
            };

            let val = FieldValue::parse(field_def.kind.clone(), (field_name, field_value))?;
            if self.primary_key.contains(&field_name.to_string()) {
                primary_key.value.insert(field_name.to_string(), val.value.clone());
            }
            fields.insert(field_name.to_string(), val);
        }

        Ok(Record{primary_key, fields})
    }
}