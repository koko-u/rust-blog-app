use crate::features::comments::models;
use crate::features::posts::models as p_models;

#[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Path)]
pub struct PostCommentParam {
    pub post_id: p_models::PostId,
    pub comment_id: models::CommentId,
}
