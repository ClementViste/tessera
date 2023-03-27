use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

/// Returns the username.
#[tracing::instrument(name = "Getting username", skip(pool))]
pub async fn get_username(pool: &PgPool, user_id: Uuid) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(
        r#"
        SELECT username
        FROM users
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
    .context("Failed to retrieve the username")?;

    Ok(row.username)
}
