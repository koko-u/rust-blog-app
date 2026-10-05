use crate::features::posts::rows;
use crate::features::users::models as u_models;

pub async fn select_by_user_id<'c, E>(
    executor: E,
    user_id: u_models::UserId,
) -> Result<rows::PostRows, sqlx::Error>
where
    E: sqlx::Executor<'c, Database = sqlx::Postgres>,
{
    let rows = sqlx::query_file_as!(
        rows::PostRow,
        "sql/posts/select_by_user_id.sql",
        user_id.into_inner()
    )
    .fetch_all(executor)
    .await?;
    Ok(rows.into())
}
