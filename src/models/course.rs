use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    pub id: Uuid,
    pub blockchain_id: String,
    pub instructor_address: String,
    pub title: String,
    pub metadata: String,
    pub price: i64,
    pub is_active: bool,
    pub created_at: DateTime<Utc>
}