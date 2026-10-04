use crate::features::categories::responses as c_responses;
use crate::features::posts::models;
use crate::features::tags::responses as t_responses;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, utoipa::ToSchema)]
pub struct PostResponse {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub category: c_responses::CategoryResponse,
    pub title: String,
    pub slug: String,
    pub content: Option<String>,
    pub tags: Vec<t_responses::TagResponse>,
}

impl From<models::PostModel> for PostResponse {
    fn from(model: models::PostModel) -> Self {
        Self {
            id: model.id.into_inner(),
            user_id: model.user_id.into_inner(),
            category: model.category.into(),
            title: model.title,
            slug: model.slug,
            content: model.content,
            tags: model.tags.into_iter().map(|tag| tag.into()).collect(),
        }
    }
}
