use crate::features::tags::rows;

pub async fn select_by_names<Names>(
    pool: &sqlx::PgPool,
    names: Names,
) -> Result<Vec<rows::TagRow>, sqlx::Error>
where
    Names: IntoIterator,
    Names::Item: AsRef<str>,
{
    let names = names
        .into_iter()
        .map(|name| name.as_ref().to_string())
        .collect::<Vec<_>>();
    let mut conn = pool.acquire().await?;
    sqlx::query_file_as!(rows::TagRow, "sql/tags/select_by_names.sql", &names)
        .fetch_all(conn.as_mut())
        .await
}
