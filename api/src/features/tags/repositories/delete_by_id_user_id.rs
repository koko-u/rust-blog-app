use crate::features::tags::models;
use crate::features::tags::rows;
use crate::features::users::models as u_models;
use crate::shared;

pub async fn delete_by_id_user_id(
    tx: &mut shared::Tx<'_>,
    id: models::TagId,
    user_id: u_models::UserId,
) -> Result<Option<rows::TagRow>, sqlx::Error> {
    sqlx::query_file_as!(
        rows::TagRow,
        "sql/tags/delete_by_id_user_id.sql",
        id.into_inner(),
        user_id.into_inner()
    )
    .fetch_optional(tx.conn())
    .await
}
