use crate::features::tags::models;
use crate::features::users::models as u_models;

pub async fn exists_by_id_user_id(
    pool: &sqlx::PgPool,
    id: models::TagId,
    user_id: u_models::UserId,
) -> Result<bool, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_scalar!(
        "sql/tags/exists_by_id_user_id.sql",
        id.into_inner(),
        user_id.into_inner()
    )
    .fetch_one(conn.as_mut())
    .await
}
