use crate::features::tags::models;

#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct TagRow {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub name: String,
}

impl From<TagRow> for models::TagModel {
    fn from(row: TagRow) -> Self {
        Self {
            id: row.id.into(),
            user_id: row.user_id.into(),
            name: row.name,
        }
    }
}
