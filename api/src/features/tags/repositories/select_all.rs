use crate::features::tags::rows;

pub async fn select_all(pool: &sqlx::PgPool) -> Result<Vec<rows::TagRow>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_as!(rows::TagRow, "sql/tags/select_all.sql")
        .fetch_all(conn.as_mut())
        .await
}
