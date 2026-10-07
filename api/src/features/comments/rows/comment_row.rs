use crate::features::comments::models;

#[derive(Debug, Clone, Eq, PartialEq, sqlx::FromRow)]
pub struct CommentRow {
    pub id: uuid::Uuid,
    pub post_id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub content: String,
}

impl From<CommentRow> for models::CommentModel {
    fn from(row: CommentRow) -> Self {
        Self {
            id: row.id.into(),
            post_id: row.post_id.into(),
            user_id: row.user_id.into(),
            content: row.content,
        }
    }
}
