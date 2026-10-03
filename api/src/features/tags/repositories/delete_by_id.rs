use crate::features::tags::models;
use crate::features::tags::rows;
use crate::shared;

pub async fn delete_by_id(
    tx: &mut shared::Tx<'_>,
    id: models::TagId,
) -> Result<Option<rows::TagRow>, sqlx::Error> {
    sqlx::query_file_as!(rows::TagRow, "sql/tags/delete_by_id.sql", id.into_inner())
        .fetch_optional(tx.conn())
        .await
}
