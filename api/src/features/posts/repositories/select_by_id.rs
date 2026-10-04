use crate::features::posts::models;
use crate::features::posts::rows;

pub async fn select_by_id(pool: &sqlx::PgPool, id: models::PostId) -> Result<rows::PostRows, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    let rows = sqlx::query_file_as!(rows::PostRow, "sql/posts/select_by_id.sql", id.into_inner())
        .fetch_all(conn.as_mut())
        .await?;
    Ok(rows.into())
}
