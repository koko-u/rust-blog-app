use crate::features::categories::models;
use crate::shared;

pub async fn exists_by_key(
    pool: &sqlx::PgPool,
    id: models::CategoryId,
    user_id: shared::models::UserId,
) -> Result<bool, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_scalar!(
        "sql/categories/exists_by_key.sql",
        id.into_inner(),
        user_id.into_inner()
    )
    .fetch_one(conn.as_mut())
    .await
}
