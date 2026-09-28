use crate::features::categories::rows;

pub async fn select_all(pool: &sqlx::PgPool) -> Result<Vec<rows::CategoryRow>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_as!(rows::CategoryRow, "sql/categories/select_all.sql")
        .fetch_all(conn.as_mut())
        .await
}
