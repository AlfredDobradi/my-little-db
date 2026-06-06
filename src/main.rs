use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use my_little_db::record::{Field, Value, ValueType};
use my_little_db::schema::{FieldDefinition, Schema};

fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    tracing::info!("start");

    let schema = Schema::new(
        vec!["id", "sub_id"],
        vec![
            FieldDefinition::new("id", ValueType::Integer),
            FieldDefinition::new("sub_id", ValueType::Integer),
            FieldDefinition::new("name", ValueType::String),
            FieldDefinition::new("timestamp", ValueType::Integer),
            FieldDefinition::new("duration", ValueType::Float),
        ]
    );

    let record = schema.new_record(
        vec![
            Field::new("id", Value::Integer(1)),
            Field::new("sub_id", Value::Integer(1)),
            Field::new("name", Value::String("testing".to_string())),
            Field::new("timestamp", Value::Integer(1)),
            Field::new("duration", Value::Float(1.0)),
        ]
    )?;

    tracing::info!(?record, "created new record");

    Ok(())
}
