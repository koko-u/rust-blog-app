use crate::features::tags::commands;
use crate::features::tags::rows;
use crate::shared;

pub async fn update_optional(
    tx: &mut shared::Tx<'_>,
    command: &commands::UpdateTagCommand,
) -> Result<Option<rows::TagRow>, sqlx::Error> {
    let commands::UpdateTagCommand { id, user_id, name } = command;

    sqlx::query_file_as!(
        rows::TagRow,
        "sql/tags/update_optional.sql",
        id.into_inner(),
        user_id.into_inner(),
        name
    )
    .fetch_optional(tx.conn())
    .await
}
