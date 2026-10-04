use crate::features::posts::models as p_models;
use crate::shared;

pub async fn delete_tags_of_post(
    tx: &mut shared::Tx<'_>,
    user_id: shared::models::UserId,
    post_id: p_models::PostId,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query_file!(
        "sql/post_tags/delete_tags_of_post.sql",
        user_id.into_inner(),
        post_id.into_inner()
    )
    .execute(tx.conn())
    .await?;
    Ok(result.rows_affected())
}
