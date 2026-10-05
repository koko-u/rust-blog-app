use crate::features::categories::rows;
use crate::features::users::models as u_models;
use crate::shared;

pub async fn insert_optional(
    tx: &mut shared::Tx<'_>,
    user_id: u_models::UserId,
    name: &str,
    slug: &str,
) -> Result<Option<rows::CategoryRow>, sqlx::Error> {
    sqlx::query_file_as!(
        rows::CategoryRow,
        "sql/categories/insert_optional.sql",
        user_id.into_inner(),
        name,
        slug
    )
    .fetch_optional(tx.conn())
    .await
}
