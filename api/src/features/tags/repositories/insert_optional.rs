use crate::features::tags::commands;
use crate::features::tags::rows;
use crate::shared;

pub async fn insert_optional(
    tx: &mut shared::Tx<'_>,
    command: &commands::CreateTagCommand,
) -> Result<Option<rows::TagRow>, sqlx::Error> {
    let commands::CreateTagCommand { user_id, name } = command;

    sqlx::query_file_as!(
        rows::TagRow,
        "sql/tags/insert_optional.sql",
        user_id.into_inner(),
        name
    )
    .fetch_optional(tx.conn())
    .await
}
