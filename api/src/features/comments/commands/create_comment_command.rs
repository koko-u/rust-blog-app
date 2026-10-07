use crate::features::posts::models as p_models;
use crate::features::users::models as u_models;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CreateCommentCommand {
    pub post_id: p_models::PostId,
    pub user_id: u_models::UserId,
    pub content: String,
}
