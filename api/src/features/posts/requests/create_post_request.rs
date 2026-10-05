use std::collections;

use crate::errors;
use crate::features::posts::commands;
use crate::features::posts::requests::validators::*;
use crate::features::tags::repositories as t_repositories;
use crate::features::users::models as u_models;
use crate::shared::macros::merge;
use crate::shared::validators;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, garde::Validate, utoipa::ToSchema)]
pub struct CreatePostRequest {
    #[garde(required, inner(custom(validators::valid_type_of::<uuid::Uuid>)))]
    #[schema(required, value_type = uuid::Uuid)]
    pub category_id: Option<String>,

    #[garde(required, length(max = 255))]
    #[schema(required, max_length = 255)]
    pub title: Option<String>,

    #[garde(length(max = 4000))]
    #[schema(max_length = 4000)]
    pub content: Option<String>,

    #[garde(inner(length(max = 255)))]
    pub tag_names: Vec<String>,
}

impl CreatePostRequest {
    pub async fn validate_into(
        self,
        user_id: u_models::UserId,
        pool: &sqlx::PgPool,
    ) -> Result<commands::CreatePostCommand, errors::ValidationError> {
        use garde::Validate as _;
        let garde_result = self.validate();

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

        match (garde_result, category_result, tag_names_result) {
            (Ok(()), Ok(()), Ok(())) => Ok(commands::CreatePostCommand {
                user_id,
                category_id: category_id.into(),
                title: self.title.expect("title is required"),
                content: self.content,
                tag_ids: tag_names.into_iter().map(|row| row.id.into()).collect(),
            }),
            (r1, r2, r3) => {
                let report = merge!(r1, r2, r3);
                Err(errors::ValidationError::Validation(report))
            }
        }
    }
}
