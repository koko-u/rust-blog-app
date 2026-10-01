use crate::features::categories::commands;
use crate::features::categories::rows;
use crate::shared;

pub async fn update_optional(
    tx: &mut shared::Tx<'_>,
    command: &commands::UpdateCategoryCommand,
) -> Result<Option<rows::CategoryRow>, sqlx::Error> {
    let commands::UpdateCategoryCommand {
        id,
        user_id,
        name,
        slug,
    } = command;

    sqlx::query_file_as!(
        rows::CategoryRow,
        "sql/categories/update_optional.sql",
        id.into_inner(),
        user_id.into_inner(),
        name,
        slug
    )
    .fetch_optional(tx.conn())
    .await
}
