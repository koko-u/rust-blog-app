use crate::features::tags::models;
use crate::features::tags::rows;

pub async fn select_by_id(
    pool: &sqlx::PgPool,
    id: models::TagId,
) -> Result<Option<rows::TagRow>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_as!(rows::TagRow, "sql/tags/select_by_id.sql", id.into_inner())
        .fetch_optional(conn.as_mut())
        .await
}
