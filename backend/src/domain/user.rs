use serde::{Deserialize, Serialize};
use sqlx::types::time::OffsetDateTime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub user_id: i32,
    pub email: String,
    pub create_time: OffsetDateTime,
    pub update_time: OffsetDateTime,
}
