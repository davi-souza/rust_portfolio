use chrono::prelude::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct InternalItem {
    pub pk: Uuid,
    pub id: Uuid,
    pub text: String,
    pub number: i64,
    pub created_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct Item {
    pub id: Uuid,
    pub text: String,
    pub number: i64,
    pub created_at: DateTime<Utc>,
}
