use crate::features::categories::models;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, utoipa::ToSchema)]
pub struct CategoryResponse {
    pub id: uuid::Uuid,
    pub name: String,
    pub slug: String,
}

impl From<models::CategoryModel> for CategoryResponse {
    fn from(category: models::CategoryModel) -> Self {
        Self {
            id: category.id.into_inner(),
            name: category.name,
            slug: category.slug,
        }
    }
}
