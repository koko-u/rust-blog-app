use std::collections;

use crate::errors;
use crate::features::tags::repositories as t_repositories;

pub async fn exists_tag_names<Names>(
    pool: &sqlx::PgPool,
    names: Names,
) -> Result<Result<(), garde::Report>, errors::ValidationError>
where
    Names: IntoIterator,
    Names::Item: AsRef<str>,
{
    let distinct_tag_names = names
        .into_iter()
        .map(|name| name.as_ref().to_string())
        .collect::<collections::HashSet<_>>();
    let exists_rows = t_repositories::exists_by_names(pool, &distinct_tag_names).await?;
    if exists_rows.iter().any(|row| !row.exists) {
        let mut report = garde::Report::new();
        for row in exists_rows.iter().filter(|row| !row.exists) {
            report.append(
                garde::Path::new("tag_name"),
                garde::Error::new(format!("Tag name={name} are not exists", name = row.name)),
            );
        }
        return Ok(Err(report));
    }

    Ok(Ok(()))
}
