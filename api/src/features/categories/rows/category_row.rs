use crate::features::categories::models;

#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct CategoryRow {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub name: String,
    pub slug: String,
}

impl From<CategoryRow> for models::CategoryModel {
    fn from(row: CategoryRow) -> Self {
        Self {
            id: row.id.into(),
            user_id: row.user_id.into(),
            name: row.name,
            slug: row.slug,
        }
    }
}
