use crate::features::posts::models as p_models;
use crate::features::tags::models as t_models;
use crate::features::users::models as u_models;
use crate::shared;

pub async fn insert_optional(
    tx: &mut shared::Tx<'_>,
    user_id: u_models::UserId,
    post_id: p_models::PostId,
    tag_id: t_models::TagId,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query_file!(
        "sql/post_tags/insert_optional.sql",
        user_id.into_inner(),
        post_id.into_inner(),
        tag_id.into_inner()
    )
    .execute(tx.conn())
    .await?;
    Ok(result.rows_affected())
}
