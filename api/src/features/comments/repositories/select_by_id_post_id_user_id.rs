use crate::features::comments::models;
use crate::features::comments::rows;
use crate::features::posts::models as p_models;
use crate::features::users::models as u_models;

pub async fn select_by_id_post_id_user_id(
    pool: &sqlx::PgPool,
    id: models::CommentId,
    post_id: p_models::PostId,
    user_id: u_models::UserId,
) -> Result<Option<rows::CommentRow>, sqlx::Error> {
    let mut conn = pool.acquire().await?;
    sqlx::query_file_as!(
        rows::CommentRow,
        "sql/comments/select_by_id_post_id_user_id.sql",
        id.into_inner(),
        post_id.into_inner(),
        user_id.into_inner()
    )
    .fetch_optional(conn.as_mut())
    .await
}
