use crate::errors;
use crate::features::categories::commands;
use crate::features::categories::models;
use crate::features::categories::repositories;
use crate::shared;
use crate::shared::macros::merge;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, garde::Validate, utoipa::ToSchema)]
pub struct UpdateCategoryRequest {
    #[garde(required, length(max = 255))]
    #[schema(required, max_length = 255)]
    pub name: Option<String>,
    #[garde(required, length(max = 255))]
    #[schema(required, max_length = 255)]
    pub slug: Option<String>,
}

impl UpdateCategoryRequest {
    pub async fn validate_into(
        self,
        id: models::CategoryId,
        user_id: shared::models::UserId,
        pool: &sqlx::PgPool,
    ) -> Result<commands::UpdateCategoryCommand, errors::ValidationError> {
        use garde::Validate as _;
        let garde_result = self.validate();

        // check exists id and user_id
        let exists = repositories::exists_by_key(pool, id, user_id).await?;
        if !exists {
            return Err(errors::ValidationError::NoResource {
                message: "There is no category corresponding to id and user_id".to_string(),
            });
        }

        // slug should not be duplicate
        let mut slug_result = Ok(());
        if let Some(slug) = &self.slug {
            let row = repositories::select_by_slug(pool, slug).await?;
            if let Some(row) = row
                && row.id != id.into_inner()
            {
                // find slug which id is not self id
                let mut report = garde::Report::new();
                report.append(
                    garde::Path::new("slug"),
                    garde::Error::new(format!("The slug to be changed({slug}) is duplicated.")),
                );
                slug_result = Err(report);
            }
        }

        match (garde_result, slug_result) {
            (Ok(()), Ok(())) => Ok(commands::UpdateCategoryCommand {
                id,
                user_id,
                name: self.name.expect("category name should be required"),
                slug: self.slug.expect("slug should be required"),
            }),
            (r1, r2) => {
                let report = merge!(r1, r2);
                Err(errors::ValidationError::Validation(report))
            }
        }
    }
}
