use crate::features::posts::models;
use crate::shared;

pub async fn delete_by_id(tx: &mut shared::Tx<'_>, id: models::PostId) -> Result<u64, sqlx::Error> {
    let result = sqlx::query_file!("sql/posts/delete_by_id.sql", id.into_inner())
        .execute(tx.conn())
        .await?;
    Ok(result.rows_affected())
}
