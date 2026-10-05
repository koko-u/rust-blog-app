use crate::features::users::models as u_models;

pub async fn exists_by_user_id_name(
    pool: &sqlx::PgPool,
    user_id: u_models::UserId,
    name: &str,
) -> Result<bool, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_scalar!("sql/tags/exists_by_user_id_name.sql", user_id.into_inner(), name)
        .fetch_one(conn.as_mut())
        .await
}
