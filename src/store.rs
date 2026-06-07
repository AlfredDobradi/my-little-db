use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::buffer::Buffer;
use crate::schema::Schema;

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct Store {
    path: String,
    buffer: Buffer,
    schema: Schema,
}

impl Store {
    pub fn open(path: &str, schema: &Schema) -> anyhow::Result<Self> {
        std::fs::create_dir_all(path)?;

        let global = GlobalMetadata::new(path, &schema);
        global.write()?;

        for (_, mut chunk) in global.chunks {
            chunk.flush(path, None)?;
        }

        Ok(Self{
            path: path.to_string(),
            buffer: Buffer::default(),
            schema: schema.clone(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Chunk {
    id: String,
    column: String,
    seq: u64,
    path: String,
    start_hash: u64,
    end_hash: u64,
    records: u64,
    size: u64,
}

impl Chunk {
    pub fn new(column: String, seq: u64) -> Self {
        let id = format!("chunk-{}-{}", column, seq);
        let path = format!("{}.d", id);
        Self {
            id,
            column,
            seq,
            path,
            start_hash: 0,
            end_hash: 0,
            records: 0,
            size: 0,
        }
    }

    fn flush(&mut self, path: &str, _buffer: Option<&Buffer>) -> anyhow::Result<()> {
        // check if file already exists
        //  - if not, create it
        //  - if yes, read contents into read buffer
        let mut path = PathBuf::from(path);
        path.push(&self.path);
        if !std::fs::exists(path.clone())? {
            std::fs::write(path, serde_json::to_string(&self)?)?;
            return Ok(());
        }



        Ok(())
        // if write buffer is Some and read buffer is Some, append write buffer to read, update metadata and flush
        // if write buffer is Some and read buffer is None, update metadata and flush
        // if write buffer is None and read buffer is Some, do nothing
        // * if write buffer is None and read buffer is None, write initial metadata
    }
}

struct GlobalMetadata {
    path: PathBuf,
    chunks: HashMap<String, Chunk>,
}

impl GlobalMetadata {
    pub fn new(path: &str, schema: &Schema) -> Self {
        let mut file_path = PathBuf::from(path);
        file_path.push("metadata.json");

        let fields = schema.fields().clone();
        let mut chunks = HashMap::new();
        for (name, _) in fields {
            let chunk = Chunk::new(name, 0);
            chunks.insert(chunk.id.clone(), chunk);
        }

        GlobalMetadata{
            path: file_path,
            chunks
        }
    }

    pub fn write(&self) -> anyhow::Result<()> {
        std::fs::write(&self.path, serde_json::to_string(&self.chunks)?)?;
        
        Ok(())
    }
}

