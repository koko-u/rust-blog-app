use crate::features::tags::rows;
use crate::features::users::models as u_models;
use crate::shared;
pub async fn insert_optional(
    tx: &mut shared::Tx<'_>,
    user_id: u_models::UserId,
    name: &str,
) -> Result<Option<rows::TagRow>, sqlx::Error> {
    sqlx::query_file_as!(
        rows::TagRow,
        "sql/tags/insert_optional.sql",
        user_id.into_inner(),
        name
    )
    .fetch_optional(tx.conn())
    .await
}
