use crate::features::tags::models;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, utoipa::ToSchema)]
pub struct TagResponse {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub name: String,
}

impl From<models::TagModel> for TagResponse {
    fn from(model: models::TagModel) -> Self {
        Self {
            id: model.id.into_inner(),
            user_id: model.user_id.into_inner(),
            name: model.name,
        }
    }
}
