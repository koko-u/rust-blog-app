use crate::features::comments::models;
use crate::features::comments::rows;
use crate::features::posts::models as p_models;
use crate::features::users::models as u_models;
use crate::shared;

pub async fn update(
    tx: &mut shared::Tx<'_>,
    id: models::CommentId,
    post_id: p_models::PostId,
    user_id: u_models::UserId,
    content: &str,
) -> Result<rows::CommentRow, sqlx::Error> {
    sqlx::query_file_as!(
        rows::CommentRow,
        "sql/comments/update.sql",
        id.into_inner(),
        post_id.into_inner(),
        user_id.into_inner(),
        content
    )
    .fetch_one(tx.conn())
    .await
}
