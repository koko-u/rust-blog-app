use crate::features::categories::models;
use crate::features::categories::rows;
use crate::shared;

pub async fn delete_by_id(
    tx: &mut shared::Tx<'_>,
    id: models::CategoryId,
) -> Result<Option<rows::CategoryRow>, sqlx::Error> {
    sqlx::query_file_as!(
        rows::CategoryRow,
        "sql/categories/delete_by_id.sql",
        id.into_inner()
    )
    .fetch_optional(tx.conn())
    .await
}
