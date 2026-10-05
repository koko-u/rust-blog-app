use crate::features::categories::rows;
use crate::features::users::models as u_models;
pub async fn select_by_user_id_name(
    pool: &sqlx::PgPool,
    user_id: u_models::UserId,
    name: &str,
) -> Result<Vec<rows::CategoryRow>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_as!(
        rows::CategoryRow,
        "sql/categories/select_by_user_id_name.sql",
        user_id.into_inner(),
        name
    )
    .fetch_all(conn.as_mut())
    .await
}
