use crate::features::comments::rows;
use crate::features::posts::models as p_models;
use crate::features::users::models as u_models;

pub async fn select_by_post_id_user_id(
    pool: &sqlx::PgPool,
    post_id: p_models::PostId,
    user_id: u_models::UserId,
) -> Result<Vec<rows::CommentRow>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_as!(
        rows::CommentRow,
        "sql/comments/select_by_post_id_user_id.sql",
        post_id.into_inner(),
        user_id.into_inner()
    )
    .fetch_all(conn.as_mut())
    .await
}
