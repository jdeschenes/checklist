use eyre::Result;
use sqlx::PgPool;

use crate::domain::User;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let row = sqlx::query_as!(
            User,
            r#"
            SELECT user_id, email, create_time, update_time
            FROM users
            WHERE email = $1
            "#,
            email,
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }
}
