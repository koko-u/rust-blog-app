use crate::features::tags::rows;
use crate::shared;

pub async fn select_by_user_id_name(
    pool: &sqlx::PgPool,
    user_id: shared::models::UserId,
    name: &str,
) -> Result<Option<rows::TagRow>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_as!(
        rows::TagRow,
        "sql/tags/select_by_user_id_name.sql",
        user_id.into_inner(),
        name
    )
    .fetch_optional(conn.as_mut())
    .await
}
