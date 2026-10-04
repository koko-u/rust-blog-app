use crate::features::tags::rows;
use crate::shared;

pub async fn insert_or_select(
    tx: &mut shared::Tx<'_>,
    user_id: shared::models::UserId,
    name: &str,
) -> Result<rows::TagRow, sqlx::Error> {
    sqlx::query_file_as!(
        rows::TagRow,
        "sql/tags/insert_or_select.sql",
        user_id.into_inner(),
        name
    )
    .fetch_one(tx.conn())
    .await
}
