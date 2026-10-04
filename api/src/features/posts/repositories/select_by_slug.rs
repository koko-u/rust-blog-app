use crate::features::posts::rows;

pub async fn select_by_slug(pool: &sqlx::PgPool, slug: &str) -> Result<rows::PostRows, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    let rows = sqlx::query_file_as!(rows::PostRow, "sql/posts/select_by_slug.sql", slug)
        .fetch_all(conn.as_mut())
        .await?;
    Ok(rows.into())
}
