use crate::features::posts::rows;

pub async fn select_all(pool: &sqlx::PgPool) -> Result<rows::PostRows, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    let rows = sqlx::query_file_as!(rows::PostRow, "sql/posts/select_all.sql")
        .fetch_all(conn.as_mut())
        .await?;
    Ok(rows.into())
}
