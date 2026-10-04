use std::collections;

use crate::errors;
use crate::features::posts::commands;
use crate::shared;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, garde::Validate, utoipa::ToSchema)]
pub struct CreatePostRequest {
    #[garde(required, length(max = 255))]
    #[schema(required, max_length = 255)]
    pub category_name: Option<String>,

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
        user_id: shared::models::UserId,
        pool: &sqlx::PgPool,
    ) -> Result<commands::CreatePostCommand, errors::ValidationError> {
        use garde::Validate as _;
        let garde_result = self.validate();

        let distinct_tag_names = self.tag_names.into_iter().collect::<collections::HashSet<_>>();

        match garde_result {
            Ok(()) => Ok(commands::CreatePostCommand {
                user_id,
                category_name: self.category_name.expect("category_name is required"),
                title: self.title.expect("title is required"),
                content: self.content,
                tag_names: distinct_tag_names.into_iter().collect(),
            }),
            Err(report) => Err(errors::ValidationError::Validation(report)),
        }
    }
}
