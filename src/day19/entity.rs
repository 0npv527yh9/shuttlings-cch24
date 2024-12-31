use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Quote {
    pub id: Uuid,
    pub author: String,
    pub quote: String,
    pub created_at: DateTime<Utc>,
    pub version: i32,
}

#[derive(Deserialize)]
pub struct UpdateQuote {
    pub author: String,
    pub quote: String,
}

#[derive(Deserialize)]
pub struct InsertQuote {
    pub author: String,
    pub quote: String,
}
