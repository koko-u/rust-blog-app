use crate::features::categories::models;
use crate::features::categories::rows;
use crate::features::users::models as u_models;
use crate::shared;

pub async fn delete_by_id_user_id(
    tx: &mut shared::Tx<'_>,
    id: models::CategoryId,
    user_id: u_models::UserId,
) -> Result<Option<rows::CategoryRow>, sqlx::Error> {
    sqlx::query_file_as!(
        rows::CategoryRow,
        "sql/categories/delete_by_id_user_id.sql",
        id.into_inner(),
        user_id.into_inner()
    )
    .fetch_optional(tx.conn())
    .await
}
