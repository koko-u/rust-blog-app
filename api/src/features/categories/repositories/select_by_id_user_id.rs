use crate::features::categories::models;
use crate::features::categories::rows;
use crate::features::users::models as u_models;

pub async fn select_by_id_user_id(
    pool: &sqlx::PgPool,
    id: models::CategoryId,
    user_id: u_models::UserId,
) -> Result<Option<rows::CategoryRow>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_as!(
        rows::CategoryRow,
        "sql/categories/select_by_id_user_id.sql",
        id.into_inner(),
        user_id.into_inner()
    )
    .fetch_optional(conn.as_mut())
    .await
}
