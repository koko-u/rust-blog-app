use crate::features::categories::models;
use crate::features::categories::rows;

pub async fn select_by_id(
    pool: &sqlx::PgPool,
    id: models::CategoryId,
) -> Result<Option<rows::CategoryRow>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_as!(
        rows::CategoryRow,
        "sql/categories/select_by_id.sql",
        id.into_inner()
    )
    .fetch_optional(conn.as_mut())
    .await
}
