use my_little_db::types;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};
use my_little_db::types::Buffer;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let schema = types::Schema::new(
        vec!["id"],
        vec![
            ("id", types::ValueType::Long),
            ("name", types::ValueType::String),
        ],
    )?;

    let mut buffer = Buffer::new(&schema);

    let record = schema.new_record(vec![
        ("id", "1"),
        ("name", "Alice"),
    ])?;

    tracing::info!(?schema, "created schema");
    tracing::info!(?buffer, "created buffer");
    tracing::info!(?record, "created record");

    buffer.append(record)?;
    
    tracing::info!(?buffer, "appended record to buffer");
    
    Ok(())
}
