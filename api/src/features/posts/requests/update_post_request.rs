use std::collections;

use crate::errors;
use crate::features::posts::commands;
use crate::features::posts::models;
use crate::features::posts::repositories;
use crate::features::posts::requests::validators::*;
use crate::features::tags::repositories as t_repositories;
use crate::features::users::models as u_models;
use crate::shared::macros::merge;
use crate::shared::validators::valid_type_of;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, garde::Validate, utoipa::ToSchema)]
pub struct UpdatePostRequest {
    #[garde(required, inner(custom(valid_type_of::<uuid::Uuid>)))]
    #[schema(required, value_type = uuid::Uuid)]
    pub category_id: Option<String>,

    #[garde(required, length(max = 255))]
    #[schema(required, max_length = 255)]
    pub title: Option<String>,

    #[garde(required, length(max = 255))]
    #[schema(required, max_length = 255)]
    pub slug: Option<String>,

    #[garde(length(max = 4000))]
    #[schema(max_length = 4000)]
    pub content: Option<String>,

    #[garde(inner(length(max = 255)))]
    pub tag_names: Vec<String>,
}

impl UpdatePostRequest {
    pub async fn validate_into(
        self,
        id: models::PostId,
        user_id: u_models::UserId,
        pool: &sqlx::PgPool,
    ) -> Result<commands::UpdatePostCommand, errors::ValidationError> {
        use garde::Validate as _;
        let garde_result = self.validate();

        // check exists id and user_id
        let exists = repositories::exists_by_id_user_id(pool, id, user_id).await?;
        if !exists {
            return Err(errors::ValidationError::NoResource {
                message: "There is no Posts corresponding to id and user_id".to_string(),
            });
        }

        // check changed slug
        let mut slug_result = Ok(());
        if let Some(slug) = &self.slug {
            let rows = repositories::select_by_slug(pool, slug).await?;
            if !rows.iter().all(|row| row.id == id.into_inner()) {
                // found post does not have target post id
                let mut report = garde::Report::new();
                report.append(
                    garde::Path::new("slug"),
                    garde::Error::new(format!("changed slug={slug} is already exists")),
                );
                slug_result = Err(report);
            }
        }

        // check exists category
        let category_result = exists_category(pool, &self.category_id).await?;

        // check exists tag names
        let tag_names_result = exists_tag_names(pool, &self.tag_names).await?;

        let category_id = self
            .category_id
            .expect("category_id must be provided")
            .parse::<uuid::Uuid>()
            .expect("valid category_id must be type of uuid::Uuid");

        let distinct_tag_names = self.tag_names.into_iter().collect::<collections::HashSet<_>>();
        let tag_names = t_repositories::select_by_names(pool, distinct_tag_names).await?;

        match (garde_result, slug_result, category_result, tag_names_result) {
            (Ok(()), Ok(()), Ok(()), Ok(())) => Ok(commands::UpdatePostCommand {
                id,
                user_id,
                category_id: category_id.into(),
                title: self.title.expect("title is required"),
                slug: self.slug.expect("slug is required"),
                content: self.content,
                tag_ids: tag_names.into_iter().map(|row| row.id.into()).collect(),
            }),
            (r1, r2, r3, r4) => {
                let report = merge!(r1, r2, r3, r4);
                Err(errors::ValidationError::Validation(report))
            }
        }
    }
}
