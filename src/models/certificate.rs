use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Certificate {
    pub id: Uuid,
    pub student_address: String,
    pub course_id: Uuid,
    pub blockchain_hash: String,
    pub issued_at: DateTime<Utc>,
}