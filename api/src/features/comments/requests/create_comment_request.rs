use crate::errors;
use crate::features::comments::commands;
use crate::features::posts::models as p_models;
use crate::features::posts::repositories as p_repositories;
use crate::features::users::models as u_models;
use crate::shared::macros::merge;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, garde::Validate, utoipa::ToSchema)]
pub struct CreateCommentRequest {
    #[garde(required, length(max = 4000))]
    #[schema(required, max_length = 4000)]
    pub content: Option<String>,
}

impl CreateCommentRequest {
    pub async fn validate_into(
        self,
        post_id: p_models::PostId,
        user_id: u_models::UserId,
        pool: &sqlx::PgPool,
    ) -> Result<commands::CreateCommentCommand, errors::ValidationError> {
        use garde::Validate as _;

        let garde_result = self.validate();

        let mut post_result = Ok(());
        let exists = p_repositories::exists_by_id_user_id(pool, post_id, user_id).await?;
        if !exists {
            let mut report = garde::Report::new();
            report.append(
                garde::Path::new("post_id"),
                garde::Error::new(format!("Post with id {} does not exist", post_id)),
            );
            post_result = Err(report);
        }

        match (garde_result, post_result) {
            (Ok(()), Ok(())) => Ok(commands::CreateCommentCommand {
                post_id,
                user_id,
                content: self.content.expect("content should be required"),
            }),
            (r1, r2) => {
                let report = merge!(r1, r2);
                Err(errors::ValidationError::Validation(report))
            }
        }
    }
}
