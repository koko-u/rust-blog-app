use crate::features::categories::rows;
use crate::features::users::models as u_models;

pub async fn select_by_user_id(
    pool: &sqlx::PgPool,
    user_id: u_models::UserId,
) -> Result<Vec<rows::CategoryRow>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_as!(
        rows::CategoryRow,
        "sql/categories/select_by_user_id.sql",
        user_id.into_inner()
    )
    .fetch_all(conn.as_mut())
    .await
}
