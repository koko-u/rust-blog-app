use crate::features::posts::models;
use crate::features::posts::rows;

pub async fn select_by_id<'c, E>(executor: E, id: models::PostId) -> Result<rows::PostRows, sqlx::Error>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    // let mut conn = pool.acquire().await?;
    let rows = sqlx::query_file_as!(rows::PostRow, "sql/posts/select_by_id.sql", id.into_inner())
        .fetch_all(executor)
        .await?;
    Ok(rows.into())
}
