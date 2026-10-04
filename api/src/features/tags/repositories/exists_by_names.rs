use crate::features::tags::rows;

pub async fn exists_by_names<Names>(
    pool: &sqlx::PgPool,
    names: Names,
) -> Result<Vec<rows::ExistsRow>, sqlx::Error>
where
    Names: IntoIterator,
    Names::Item: AsRef<str>,
{
    let mut conn = pool.acquire().await?;
    let names = names
        .into_iter()
        .map(|name| name.as_ref().to_string())
        .collect::<Vec<_>>();
    sqlx::query_file_as!(rows::ExistsRow, "sql/tags/exists_by_names.sql", &names)
        .fetch_all(conn.as_mut())
        .await
}
