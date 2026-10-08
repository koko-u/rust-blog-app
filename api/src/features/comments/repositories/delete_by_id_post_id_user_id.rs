use crate::features::comments::models;
use crate::features::comments::rows;
use crate::features::posts::models as p_models;
use crate::features::users::models as u_models;
use crate::shared;

pub async fn delete_by_id_post_id_user_id(
    tx: &mut shared::Tx<'_>,
    id: models::CommentId,
    post_id: p_models::PostId,
    user_id: u_models::UserId,
) -> Result<Option<rows::CommentRow>, sqlx::Error> {
    sqlx::query_file_as!(
        rows::CommentRow,
        "sql/comments/delete_by_id_post_id_user_id.sql",
        id.into_inner(),
        post_id.into_inner(),
        user_id.into_inner()
    )
    .fetch_optional(tx.conn())
    .await
}
