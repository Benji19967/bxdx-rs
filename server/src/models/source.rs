use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Source {
    pub id: u64,
    pub name: String,
    pub raw_as_parsed: bool,
    pub categories: Vec<String>,
    pub batch_count: u32,
    pub instant_flush: bool,
    pub created_at: DateTime<Utc>,
}
