use crate::features::posts::models;
use crate::shared;

pub async fn exists_by_id_user_id(
    pool: &sqlx::PgPool,
    id: models::PostId,
    user_id: shared::models::UserId,
) -> Result<bool, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_scalar!(
        "sql/posts/exists_by_id_user_id.sql",
        id.into_inner(),
        user_id.into_inner()
    )
    .fetch_one(conn.as_mut())
    .await
}
