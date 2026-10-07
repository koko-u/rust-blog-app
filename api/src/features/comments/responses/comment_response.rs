use into_inner::IntoInner;

use crate::features::comments::models;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, utoipa::ToSchema)]
pub struct CommentResponse {
    pub id: uuid::Uuid,
    pub post_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub content: String,
}

impl From<models::CommentModel> for CommentResponse {
    fn from(model: models::CommentModel) -> Self {
        Self {
            id: model.id.into_inner(),
            post_id: model.post_id.into_inner(),
            user_id: model.user_id.into_inner(),
            content: model.content,
        }
    }
}
