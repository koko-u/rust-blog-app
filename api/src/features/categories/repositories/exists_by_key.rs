use crate::features::categories::models;
use crate::features::users::models as u_models;

pub async fn exists_by_key(
    pool: &sqlx::PgPool,
    id: models::CategoryId,
    user_id: u_models::UserId,
) -> Result<bool, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_scalar!(
        "sql/categories/exists_by_key.sql",
        id.into_inner(),
        user_id.into_inner()
    )
    .fetch_one(conn.as_mut())
    .await
}
