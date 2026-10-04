use crate::errors;
use crate::features::categories::repositories as c_repositories;
pub async fn exists_category(
    pool: &sqlx::PgPool,
    category_id: &Option<String>,
) -> Result<Result<(), garde::Report>, errors::ValidationError> {
    if let Some(category_id) = category_id {
        // parse into uuid
        if let Ok(category_id) = category_id.parse::<uuid::Uuid>() {
            let category = c_repositories::select_by_id(pool, category_id.into()).await?;
            if category.is_none() {
                let mut report = garde::Report::new();
                report.append(
                    garde::Path::new("category_id"),
                    garde::Error::new(format!("Category of id={category_id} is not exists")),
                );
                return Ok(Err(report.into()));
            }
        }
    }

    Ok(Ok(()))
}
