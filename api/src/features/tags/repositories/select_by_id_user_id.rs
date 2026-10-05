use crate::features::tags::models;
use crate::features::tags::rows;
use crate::features::users::models as u_models;

pub async fn select_by_id_user_id(
    pool: &sqlx::PgPool,
    id: models::TagId,
    user_id: u_models::UserId,
) -> Result<Option<rows::TagRow>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_as!(
        rows::TagRow,
        "sql/tags/select_by_id_user_id.sql",
        id.into_inner(),
        user_id.into_inner()
    )
    .fetch_optional(conn.as_mut())
    .await
}
