use serde::{Deserialize, Serialize};
use crate::record::Record;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Buffer {
    records: Vec<Record>
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            records: Vec::new(),
        }
    }
}