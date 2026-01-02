use eyre::Result;
use sqlx::PgTransaction;

use crate::domain::User;


pub async fn find_by_email(transaction: &mut PgTransaction<'_>, email: &str) -> Result<Option<User>> {
    let row = sqlx::query_as!(
        User,
        r#"
        SELECT user_id, email, create_time, update_time
        FROM users
        WHERE email = $1
        "#,
        email,
    )
    .fetch_optional(&mut **transaction)
    .await?;
    Ok(row)
}
