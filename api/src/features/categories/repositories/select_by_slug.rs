use crate::features::categories::rows;

pub async fn select_by_slug(
    pool: &sqlx::PgPool,
    slug: &str,
) -> Result<Option<rows::CategoryRow>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_as!(rows::CategoryRow, "sql/categories/select_by_slug.sql", slug)
        .fetch_optional(conn.as_mut())
        .await
}
