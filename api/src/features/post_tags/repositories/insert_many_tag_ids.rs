use into_inner::IntoInner;

use crate::features::posts::models as p_models;
use crate::features::tags::models as t_models;
use crate::features::users::models as u_models;
use crate::shared;

pub async fn insert_many_tag_ids(
    tx: &mut shared::Tx<'_>,
    user_id: u_models::UserId,
    post_id: p_models::PostId,
    tag_ids: impl IntoIterator<Item = t_models::TagId>,
) -> Result<u64, sqlx::Error> {
    let tag_ids = tag_ids.into_iter().map(|id| id.into_inner()).collect::<Vec<_>>();
    let result = sqlx::query_file!(
        "sql/post_tags/insert_many_tag_ids.sql",
        user_id.into_inner(),
        post_id.into_inner(),
        &tag_ids
    )
    .execute(tx.conn())
    .await?;
    Ok(result.rows_affected())
}
