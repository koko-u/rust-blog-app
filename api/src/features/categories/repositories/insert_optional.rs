use crate::features::categories::commands;
use crate::features::categories::rows;
use crate::shared;

pub async fn insert_optional(
    tx: &mut shared::Tx<'_>,
    command: &commands::CreateCategoryCommand,
) -> Result<Option<rows::CategoryRow>, sqlx::Error> {
    let commands::CreateCategoryCommand { user_id, name, slug } = command;

    sqlx::query_file_as!(
        rows::CategoryRow,
        "sql/categories/insert_optional.sql",
        user_id,
        name,
        slug
    )
    .fetch_optional(tx.conn())
    .await
}
