use crate::features::posts::models;
use crate::features::users::models as u_models;
use crate::shared;

pub async fn delete_by_id_user_id(
    tx: &mut shared::Tx<'_>,
    id: models::PostId,
    user_id: u_models::UserId,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query_file!(
        "sql/posts/delete_by_id_user_id.sql",
        id.into_inner(),
        user_id.into_inner()
    )
    .execute(tx.conn())
    .await?;
    Ok(result.rows_affected())
}
