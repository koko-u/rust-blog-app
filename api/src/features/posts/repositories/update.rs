use crate::features::posts::commands;
use crate::shared;

pub async fn update(
    tx: &mut shared::Tx<'_>,
    command: &commands::UpdatePostCommand,
) -> Result<u64, sqlx::Error> {
    let commands::UpdatePostCommand {
        id,
        user_id,
        category_id,
        title,
        slug,
        content,
        ..
    } = command;

    let result = sqlx::query_file!(
        "sql/posts/update.sql",
        id.into_inner(),
        user_id.into_inner(),
        category_id.into_inner(),
        title,
        slug,
        content.as_ref()
    )
    .execute(tx.conn())
    .await?;
    Ok(result.rows_affected())
}
