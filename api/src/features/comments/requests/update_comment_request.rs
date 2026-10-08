use crate::errors;
use crate::features::comments::commands;
use crate::features::comments::models;
use crate::features::comments::repositories;
use crate::features::posts::models as p_models;
use crate::features::posts::repositories as p_repositories;
use crate::features::users::models as u_models;
use crate::shared::macros::merge;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, garde::Validate, utoipa::ToSchema)]
pub struct UpdateCommentRequest {
    #[garde(required, length(max = 4000))]
    #[schema(required, max_length = 4000)]
    pub content: Option<String>,
}

impl UpdateCommentRequest {
    pub async fn validate_into(
        self,
        id: models::CommentId,
        post_id: p_models::PostId,
        user_id: u_models::UserId,
        pool: &sqlx::PgPool,
    ) -> Result<commands::UpdateCommentCommand, errors::ValidationError> {
        use garde::Validate as _;

        let garde_result = self.validate();

        let mut post_result = Ok(());
        let post_exists = p_repositories::exists_by_id_user_id(pool, post_id, user_id).await?;
        if !post_exists {
            let mut report = garde::Report::new();
            report.append(
                garde::Path::new("post_id"),
                garde::Error::new(format!("Post with id {} does not exist", post_id)),
            );
            post_result = Err(report);
        }

        let mut comment_result = Ok(());
        let comment_exists = repositories::exists_by_id_post_id_user_id(pool, id, post_id, user_id).await?;
        if !comment_exists {
            let mut report = garde::Report::new();
            report.append(
                garde::Path::new("id"),
                garde::Error::new(format!("Comment with id {} does not exist", id)),
            );
            comment_result = Err(report);
        }

        match (garde_result, post_result, comment_result) {
            (Ok(()), Ok(()), Ok(())) => Ok(commands::UpdateCommentCommand {
                id,
                post_id,
                user_id,
                content: self.content.expect("content should be required"),
            }),
            (r1, r2, r3) => {
                let report = merge!(r1, r2, r3);
                Err(errors::ValidationError::Validation(report))
            }
        }
    }
}
