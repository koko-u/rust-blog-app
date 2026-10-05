use crate::features::categories::models as c_models;
use crate::features::posts::rows;
use crate::features::users::models as u_models;
use crate::shared;

pub async fn insert_optional(
    tx: &mut shared::Tx<'_>,
    user_id: u_models::UserId,
    category_id: c_models::CategoryId,
    title: &str,
    slug: &str,
    content: Option<&String>,
) -> Result<Option<rows::PostRow>, sqlx::Error> {
    sqlx::query_file_as!(
        rows::PostRow,
        "sql/posts/insert_optional.sql",
        user_id.into_inner(),
        category_id.into_inner(),
        title,
        slug,
        content
    )
    .fetch_optional(tx.conn())
    .await
}
